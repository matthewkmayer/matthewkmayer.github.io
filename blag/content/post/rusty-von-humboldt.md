+++
date = "2020-06-23T00:28:53-07:00"
draft = false
title = "Rusty von Humboldt"
+++

Rusty von Humboldt is a GitHub Archive data explorer and Extract, Transform and Load (ETL) tool. Or: **seeing how far the tools you know can take you.**

<!--more-->

## A blog post written across two years

It's been more than two years since the first draft of this post. Since then, Rusty von Humboldt (RvH) has seen a few tweaks and the maturing ecosystem has made some things much easier. An example is [Rusoto](https://github.com/rusoto/rusoto) supporting [rustls](https://github.com/ctz/rustls) which avoids a lot of heartache dealing with OpenSSL.


## What's in a name?

[Alexander von Humboldt](https://en.wikipedia.org/wiki/Alexander_von_Humboldt) jumped out from a list of explorers on Wikipedia. I got a chuckle out of replacing Alexander's name with "Rusty" so Rusty von Humboldt was born to explore the treasures of [GitHub Archive](https://www.githubarchive.org/) (GHA).

## GHA data

GitHub Archive has a record of public GitHub events since 2/12/2011. Every hour, public events are recorded in a gzipped JSON file. Every `git push`, opened pull request, accepted pull request, new issue, comment on an issue, etc... is available.

The event formats differ before and after 1/1/2015. At that date, the format switches to GitHub's Event API format. In this post we'll focus on the 2015 and later format.

The amount of data isn't *massive*: the complete archive is approximately two terabytes of gzipped JSON. *Note: this data was gathered during the first iteration of this post in 2017 or 2018*

Another way of getting an idea for scale: the record of public activity for GitHub during the first hour of December 1st, 2019 is 17 megabytes of JSON, gzipped. Decompressed, that single hour of data is 120 megabytes on my Mac. Multiply the 17 MB of compressed data by 24 hours in a day and there's about 408 megabytes of compressed files for *one day*.  To keep decompressed files around for one day is almost three gigabytes. At the December rate for one year the numbers come out to be around a terabyte of **decompressed** data.

## Strategy

The goal of the project is to track the number of committers to public repositories on GitHub without relying on the GitHub API. Making calls to GitHub isn't possible in certain customer environments, so we have to get our own copy of the data we need.

When approaching this problem, I looked at the volume of data to process, where our copy was located on S3 and how the data gleaned from GHA was planned to be used. I decided to make a proof of concept using Rust.

## Why Rust?

Instead of learning a new tool like BigQuery, Apache Spark, AWS Elastic MapReduce (EMR), etc... **I wanted to see how far the tools I knew could get me.** Crunching lots of data is a great fit for Rust and this project would let me use Rusoto in an intensive manner.

## Workflow overview

1. Fetch data from S3.
2. Find events we care about.
3. Remove duplicated events.
4. Output SQL to a different S3 bucket.
5. Run script to ingest SQL from S3 into an RDS Postgres instance.

## Fetching GitHub Archive data

The source data in S3 is organized in this manner: keys have the form of `year/month/day/hour.json.gz`. So the first hour of January 1st, 2019 would be at `2019/01/01/01.json.gz`. The list of files matching the requested year is retrieved via `ListObjectsV2Request` and includes handling paging from S3 to get all the items. Once the list of files is retrieved, it's split into two Vecs and each is sent to a thread.

Each thread downloads files from the list sequentially, decompressing them and then parsing the JSON.

## Processing GitHub Archive data with serde

With a goal of tracking the number of code committers to a repository, we'll need to traverse the entire history of data to extract the list of committers to a project. This means making a Rust representation of events. With serde, this is straightforward. Since every item in GHA is an "Event" we can start with that. Here's a JSON representation of a commit to a repository:

```json
{
  "id": "5785865382",
  "type": "PushEvent",
  "actor": {
    "id": 1234,
    "login": "direct_committer",
    "display_login": "direct_committer",
    "url": "https://api.github.com/users/direct_committer"
    },
  "repo": {
    "id": 255,
    "name": "foo/bar",
    "url": "https://api.github.com/repos/foo/bar"
  },
  "payload": {
    "push_id": 1234567,
    "size": 1,
    "distinct_size": 1
  },
  "created_at": "2017-05-01T07:00:00Z"
}
```

And the Rust representation:

```rust
/// 2015 and later github archive event.
#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(deserialize_with = "from_str")] pub id: i64,
    pub created_at: DateTime<Utc>,
    #[serde(rename = "type")] pub event_type: String,
    pub actor: Actor,
    pub repo: Repo,
    pub payload: Option<Payload>,
}

/// Type containing if it's a push event or pull request event.
#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Payload {
    pub action: Option<String>,
    #[serde(rename = "pull_request")]
    pub pull_request: Option<PullRequest>,
    pub commits: Option<Vec<Commit>>,
}
```

We discard everything in the source JSON we don't need, such as the `payload` section and the internal GitHub `id`.

RvH also needs to count contributions via accepted pull requests, like this one:

```json
{
  "id": "12345",
  "type": "PullRequestEvent",
  "actor": {
    "id": 1,
    "login": "owner-login",
    "display_login": "owner-login"
  },
  "repo": {
    "id": 155,
    "name": "foo/reponame",
    "url": "https://api.github.com/repos/foo/reponame"
  },
  "payload": {
    "action": "closed",
    "pull_request": {
      "state": "closed",
      "user": {
        "id": 5,
        "login": "committer-login"
      },
      "created_at": "2017-04-30T13:14:51Z",
      "updated_at": "2017-05-01T07:01:53Z",
      "closed_at": "2017-05-01T07:01:53Z",
      "merged_at": "2017-05-01T07:01:53Z",
      "head": {
        "repo": {
          "id": 155,
          "name": "reponame"
        }
      },
      "base": {
        "label": "foo:master",
        "ref": "master",
        "sha": "a829c2e22381a1ff55824602127b9a7e440d7dc5",
        "repo": {
          "id": 1234,
          "name": "reponame",
          "full_name": "foo/reponame",
          "created_at": "2014-12-03T22:47:01Z",
          "updated_at": "2017-04-27T09:13:53Z",
          "pushed_at": "2017-05-01T07:01:53Z"
        }
      },
      "merged": true
    }
  },
  "public": true,
  "created_at": "2017-05-01T07:01:53Z"
}
```

The Rust version of the data looks similar to the sample above, but with different fields extracted.

Each line of the GHA JSON file is one of these events. This means the file can be read line by line to get a collection of Events. Deserializing is easy with the serde annotations. Here's a code snippet from a test:

```rust
let event: Event = match serde_json::from_str(&pr_text) {
    Ok(event) => event,
    Err(err) => panic!("Found a weird line of json, got this error: {:?}.", err),
};
```

There's also a snippet of code that ensures any string representation of a number is correctly translated into a number. It's been quite a while since the code was written for my recollection is hazy: I think this had to be done because the type checker wasn't clear how the deserialization with serde would output results.

```rust
/// Allows us to convert "1234" to 1234 integer type
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
```

## Putting things together

The actual implementation of Rusty von Humboldt has a rough actor-like messaging system. The RvH takes input of what year to look at, how many hours in that year to process, then what mode to operate in: committer count or repository ID mapping.

Repository IDs internal to GitHub, exposed in GitHub Archive, are IDs that don't change. However, the repository name or location can move, so the repo ID is what we use to track the latest name of a repository. Perhaps we can explore how that part of RvH works at a different time.

Committer counts is why we're here: we want to see how many people contribute to a repo on GitHub. This is done by processing each event and see if it's a `commit event`. These functions are in the `impl` block of `Event`:

```rust
pub fn is_commit_event(&self) -> bool {
    self.is_accepted_pr() || self.is_direct_push_event()
}

pub fn is_accepted_pr(&self) -> bool {
    if self.event_type != "PullRequestEvent" {
        return false;
    }
    match self.payload {
        Some(ref payload) => match payload.pull_request {
            Some(ref pr) => match pr.merged {
                Some(merged) => merged,
                None => false,
            },
            None => false,
        },
        None => false,
    }
}

pub fn is_direct_push_event(&self) -> bool {
    if self.event_type != "PushEvent" {
        return false;
    }
    match self.payload {
        Some(ref payload) => match payload.commits {
            Some(ref commits) => !commits.is_empty(),
            None => false,
        },
        None => false,
    }
}
```

We count a merged PR or a commit directly to the repo as a commit. PRs not accepted aren't counted.

## Getting the data we want

RvH starts with getting the list of files from S3 that match the requirements: year and number of hours. It splits the list in two and sends one to a worker thread that download each file and use rayon to parse the decompressed JSON data. Here's how to get all events for a year:

`MODE=committer_count GHABUCKET=sourcebucketname DESTBUCKET=destbucketname GHAYEAR=2016 GHAHOURS=8766 cargo run --release`

Using environment variables for configuration was an ease-of-use choice, which would probably be replaced with command line arguments if I was to update RvH.

Deduplication is required because if GitHub account `A` makes multiple pushes to repository `foo`, we only care about the number of GitHub accounts doing so and count it as one committer. The first iteration of RvH stored `Event`s in a `Vec` and used a hand written function for testing equality to use with [`Vec` dedup_by](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by). This inefficient approach was changed to using a BTreeMap to get much better performance:

```rust
let mut commiter_events_bt: BTreeMap<CommitEvent, i64> = BTreeMap::new();
...
commiter_events_bt
  .entry(item.event.as_commit_event())
  .or_insert(1);
```

This was a massive win in terms of overall memory usage.

## Making the data available

See the `DESTBUCKET` option above? That's where RvH puts the SQL it generates once all the parsing work has been completed. While RvH deduplicates internally using a BTreeMap, sometimes there will be duplicate entries due to multiple threads having their own BTreeMap to store their results.

Final deduplication relies heavily on Postgres' `ON CONFLICT` abilities for upserts: we update data if the information we have is newer and ignore data already in the database. 

The schema in the destination database has a mapping table between repositories and committer IDs. Committer count is done by selecting the count of entries in that table for the requested repository ID. To populate the data, SQL like this is generated:

```sql
INSERT INTO committer_repo_id_names (repo_id, actor_name)
VALUES (1, 'bar'), (2, 'bar'), (2, 'baz'), (1, 'foo'), (2, 'foo')
ON CONFLICT DO NOTHING;
```

This example SQL, lifted from a test in RvH, has `bar` and `foo` contributing to repo `1` and committers `bar`, `baz` and `foo` committing to repo 2. A more concrete way of looking at it: it could be `matthewkmayer` committed to repos `rusty-von-humboldt` and `rusoto`. If there's already a record for the user committing to the account, Postgres will take the `ON CONFLICT` clause and do nothing.

Note the multiple parts in the `VALUES` section of the statement: batching these upserts into Postgres is far, far faster than doing one at a time.

In RvH this is created by taking the committer IDs and repos IDs and collecting them into a Vec then taking iterating in batches of 20:

```rust
a.chunks(20).map(|c| {
    let collector = c.iter().cloned().map(|x| x).collect::<Vec<String>>().join(", ");
    format!("INSERT INTO committer_repo_id_names (repo_id, actor_name) VALUES {} ON CONFLICT DO NOTHING;", collector)
})
```

Once all processed events have been collected into SQL, the string is compressed via gzip and uploaded to the destination bucket. Once a week, an ingest process runs with the Postgres command line tool to load the data into the destination Postgres instance.

## Deployment and running

Due to complexities and roughness of containers with Rust, Rusoto and openssl, we opted to make an AMI with RvH installed. To do this we built RvH on an EC2 instance and saved the machine image (AMI). The image was automatically run weekly with a startup script providing configuration such as S3 bucket source, year to process, what mode to run in and where to upload the results.

*2020 update: Dockerization is far easier with rustls: RvH was converted to use AWS ECS Fargate for "serverless" instead of using AMIs*

## Results

Rusty von Humboldt powered data analysis at my day job for years, achieving customer requirements.  

Eventually it was rewritten in Go because our company has far more Go expertise, but the work I did on RvH showed the approach was not only feasible but fulfilled our latency requirements of data being in GHA to being available to customers. The same patterns RvH used were applied in the Go version.

## Lessons learned

Rust fit the problem space very well. Its performance was rock solid and this was a wonderfully fun project to write! Rayon, Serde and Rusoto all shone in performance and stability.

Months after I stopped working on RvH, I found a blog post of a company touting their performance improvements doing a similar task. They took 0.8 TB of gzipped JSON and ran their ETL pipeline in four hours on a 36 core machine. RvH could handle 2015 through 2019 data, over two TB of gzipped JSON, on a couple of two core AWS Fargate VMs, in three hours. Mission accomplished. :)

There are things I'd do differently if I could:

* Get buy-in from management to do this during work hours instead of an after-hours and weekends project
* Use S3-Select to be easier on instance bandwidth and speed things up (as of writing this in 2020, Rusoto still does not support S3-Select)
* Figure out better ways of passing messages around - this was pretty hacky and I think I could reduce complexity by signaling via closing channels
* Code organization wasn't even a second thought on this project: things are scattered about and there's lots of mixing of concerns
* Spend a week learning about other ways to solve the problem: Athena is cheaper now, for example

In the end, the project was a success and I'm still referring to some of my work in the [Rusty von Humboldt repo](https://github.com/matthewkmayer/rusty-von-humboldt) for patterns.
