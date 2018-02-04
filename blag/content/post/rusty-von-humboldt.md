+++
date = "2018-02-03T21:28:53-07:00"
draft = false
title = "Rusty von Humboldt"
+++

Rusty von Humboldt is a [GitHub Archive](https://www.githubarchive.org/) data explorer and Extract, Transform and Load (ETL) tool.

<!--more-->

## What's in a name?

[Alexander von Humboldt](https://en.wikipedia.org/wiki/Alexander_von_Humboldt) jumped out from a list of explorers on Wikipedia. I got a chuckle out of replacing Alexander's name with "Rusty" so Rusty von Humboldt was born to explore the treasures of [GitHub Archive](https://www.githubarchive.org/) (GHA).

## GHA data

GitHub Archive has a record of public GitHub events since 2/12/2011. Every hour, public events are recorded in a gzipped JSON file. Every push, opened pull request, accepted pull request, new issue, comment on an issue, etc... are available.

The event formats differ before and after 1/1/2015. At that date, the format switches to GitHub's Event API format. In this post we'll focus on the 2015 and later format.

The amount of data isn't *massive*: the complete archive is approximately two terabytes of gzipped JSON. To be good citizens, we'll have our own mirror of GHA on an AWS S3 bucket.

## Why Rust?

There are plenty of options for analyzing this data. One could use BigQuery, Apache Spark, AWS Elastic MapReduce (EMR), etc... Let's look at these options.

### BigQuery

Pros:

* Easiest to get started, with examples right on GHA's homepage
* One only has to write SQL, no management of code and servers

Cons:

* I don't have experience with BigQuery
* Nobody else I know has experience with BigQuery
* How do we get the results into a database for further use?

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
* Cross compiling from my Macbook to the Linux server is not pleasant, mostly due to OpenSSL

Another factor in my decision to create Rusty von Humboldt is to show off Rust, serde, rayon and Rusoto.

Yet another factor weighing in is "we don't have big data." Right now I can provision an EC2 instance with *four terabytes* of memory. Since our data fits into RAM, we don't have big data.

## Analyzing GHA data

With a goal of tracking the number of code committers to a repository, we'll need to traverse the entire history of data to extract the list of committers to a project. This means making a Rust representation of events. With serde, this is straightforward. Since every item in GHA is an "Event" we can start with that:

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
```


## Deployment and running

## Results

## Future work and blog posts