+++
date = "2020-05-06T21:28:53-07:00"
draft = true
title = "Rusty von Humboldt"
+++

Rusty von Humboldt is a GitHub Archive data explorer and Extract, Transform and Load (ETL) tool. Or: seeing how far the tools you know can take you.

<!--more-->

## A blog post written across two years

It's been more than two years since the first draft of this post. Since then, Rusty von Humboldt (RvH) has seen a few tweaks but the maturing ecosystem has made some things much easier. An example is Rusoto supporting [rustls](https://github.com/ctz/rustls) which avoids a lot of heartache dealing with OpenSSL.


## What's in a name?

[Alexander von Humboldt](https://en.wikipedia.org/wiki/Alexander_von_Humboldt) jumped out from a list of explorers on Wikipedia. I got a chuckle out of replacing Alexander's name with "Rusty" so Rusty von Humboldt was born to explore the treasures of [GitHub Archive](https://www.githubarchive.org/) (GHA).

## GHA data

GitHub Archive has a record of public GitHub events since 2/12/2011. Every hour, public events are recorded in a gzipped JSON file. Every push, opened pull request, accepted pull request, new issue, comment on an issue, etc... is available.

The event formats differ before and after 1/1/2015. At that date, the format switches to GitHub's Event API format. In this post we'll focus on the 2015 and later format.

The amount of data isn't *massive*: the complete archive is approximately two terabytes of gzipped JSON. To be good citizens, we'll have our own mirror of GHA on an AWS S3 bucket. *Note: this data was gathered during the first iteration of this post in 2017 or 2018*

Another way of getting an idea for scale: the record of public activity for GitHub during the first hour of December 1st, 2019 is 17 megabytes of JSON, gzipped. Multiply that by 24 hours in a day and there's about 408 megabytes of compressed files for *one day*. Decompressed, that single hour of data is 120 megabytes on my Mac. To keep decompressed files around for one day is almost three gigabytes. At the December rate for one year the numbers come out to be around a terabyte of **decompressed** data.

## Strategy

When approaching this problem, I looked at the volume of data to process, where it was located and how the data gleaned from GHA was planned to be used. I decided to make a proof of concept using Rust, Rusoto and see how far I could get with the tools I knew.

## Why Rust?

There are plenty of options for analyzing this data. One could use BigQuery, Apache Spark, AWS Elastic MapReduce (EMR), etc... Let's look at these options.

### BigQuery

Pros:

* Easiest to get started, with examples right on GHA's homepage
* One only has to write SQL, no management of code and servers

Cons:

* I don't have experience with BigQuery
* Nobody else I know has experience with BigQuery
* How do we automated putting the results into our own database for further use?

### Apache Spark

Pros:

* Documentation and examples are available: we're not forging a new path
* This work is a very close match for Spark's workflow

Cons:

* I don't have experience with Spark
* Nobody else I know has experience with Spark
* Price: running multiple instances racks up a bill quickly

### AWS Elastic MapReduce

Pros:

* Less instance management than roll-your-own Apache Spark
* Again, our work is a good match for EMR's workflow

Cons:

* I don't have experience with EMR
* Nobody else I know has experience with EMR
* Price: running multiple instances racks up a bill quickly 

### Rust

Pros:

* Proven performance
* Embarassingly easy safe multithreading
* Lower AWS bill than other options

Cons:

* Nobody else on my team knows Rust
* [Rusoto](https://github.com/rusoto/rusoto), the AWS SDK, is still a work in progress
* Cross compiling from my Macbook to the Linux server is not pleasant, mostly due to OpenSSL *(edit for 2020: rustls is a great replacement with low friction)*

Another factor in my decision to create Rusty von Humboldt is to show off Rust, serde, rayon and Rusoto.

Yet another factor weighing in is "we don't have big data." Right now I can provision an EC2 instance with *four terabytes* of memory. Since our compressed data fits into RAM, we don't have big data. 😉

## Workflow overview

// TODO: flesh out
Fetch data. Process. Deduplicate as needed. Output SQL into another S3 bucket. Run something to ingest data from SQL in S3 to an RDS Postgres instance.

## Fetching GitHub Archive data

// TODO: flesh out
S3 interactions etc...

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

Each line of the GHA JSON file is one of these events which means the file can be read line by line to get a collection of Events. Deserializing is easy with the serde annotations. Here's a code snippet from a test:

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

The actual implementation of Rusty von Humboldt has a rough actor-like messaging system. The RvH takes the user input of what year to look at, how many hours in that year to process, then what mode to operate in: committer count or repository ID mapping.

Repository IDs internal to GitHub, exposed in GitHub Archive, are IDs that don't change. However, the repository name or location can move, so the repo ID is what we use to track the latest name of a repository. Perhaps we can explore how that part of RvH works at a different time.

Committer counts is while we're here: we want to see how many people contribute to a repo on GitHub. This is done by processing each event and see if it's a `commit event`. These functions are in the `impl` block of `Event`:

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

// TODO: example of flags to provide RvH to analyze a year of data

RvH starts with getting the list of files from S3 that match the requirements: year and number of hours. It then splits that in two and sends it to two worker threads that download each file and use rayon to parse the decompressed JSON data.

Deduplication relies heavily on Postgres' `ON CONFLICT` abilities for upserts: we update data if the information we have is newer and ignore data already in the database. 

// TODO: More words on batching upserts for performance

## Deployment and running

Due to complexities and roughness of containers with Rust, Rusoto and openssl, we opted to make an AMI with RvH installed. To do this we built RvH on an EC2 instance and saved the machine image (AMI). The service was triggered weekly to start the AMI with a startup script providing configuration such as S3 bucket source, year to process, what mode to run in and where to upload the results.

*2020 update: Dockerization is far easier with rustls: RvH was converted to use AWS ECS Fargate for "serverless" instead of using AMIs*

## Results

It works, it powered data analysis at my day job for years before a rewrite in Go because our company has more Go expertise.

## Future work and blog posts

*2020 update: stdlib channels were replaced with crossbeam-channel and vectors were replaced with BTreeMaps for better performance*

## Lessons learned

(Some big blog post about moving to BigQuery for analytics was proud of: )
That project: 36 core machine, 0.8 TB of gzipped JSON, four hours.

Rusty von Humboldt: couple of 2 core Fargate VM, 2 TB of gzipped JSON, three hours.

There are things I'd do differently if I could:

* S3-select to be easier on instance bandwidth and speed things up
* Figure out better ways of passing messages around - this was pretty hacky and perhaps a real actor system would work better
* Code organization wasn't even a second thought on this project: things are scattered about and there's lots of mixing of concerns
* Spend a week learning about other ways to solve the problem: Athena is cheaper now, for example
