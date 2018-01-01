+++
date = "2017-12-31T21:28:53-07:00"
draft = false
title = "Rusoto: 2017 in review"
+++

[Rusoto](https://github.com/rusoto/rusoto), an unofficial SDK for [Amazon Web Services](https://aws.amazon.com/), had a wonderful year! We made great strides in improving functionality, increasing the number of services covered and making various ergonomic changes.

<!--more-->

## Improved functionality

* Added all publicly available AWS regions
* Custom endpoint support
* HTTP connection pools for connecting to services
* S3 supports streaming responses: no more loading the entire file to download into a Vector

## More supported services

A non-exhaustive list:

* Security Token Service, Simple Email Service, Organizations, Snowball, StepFunctions, ServiceCatalog, Support, Rekognition, OpsWorksCM, Lightsail,
Kinesis Analytics, GameLift, Database Migration Service, Cost and Usage Reports, CodeBuild, 
Cognito Identity Provider, WAF Regional, AppStream, Server Migration Service, Marketplace Metering, Health,
Batch, Glacier, Polly, Mechanical Turk, API Gateway, Application Autoscaling, Athena, X-Ray, Cloud Directory,
CloudSearch Domain, GreenGrass, Elastic Filesystem, Workdocs, Cognito Sync, Shield, Glue, DynamoDB Accelerator,
Discovery, CodeStar, Migration Hub, Marketplace Entitlement, CloudHSMv2, ResourceGroupsTaggingApi, Lex,
Lex Models, Budgets

As of the end of 2017, Rusoto is *three* services shy of implementing publicly available AWS services from before re:Invent 2017.

## Ergonomics

* The mega-crate of `rusoto` was split into many crates: `rusoto_core`, `rusoto_credential` and a crate per service. Improves compilation performance and reduces overall crate size.
* Each crate has its own documentation, available at https://rusoto.github.io/rusoto/rusoto_core/index.html or https://docs.rs/
* Region types can be serialized and deserialized for infrastructure-as-code uses
* Added a CHANGELOG
* S3 streaming downloads implement the `Read` trait for better memory control

## Rust, Rusoto, AWS

Rust and Rusoto have demonstrated they can fit well with AWS. I [gave a talk at PDXRust](https://www.meetup.com/PDXRust/events/244697789/) on this topic. While writing the presentation, I was surprised at how much Rusoto is used for infrastructure related tasks.  Some examples:

* Kubernetes cluster lifecycle management tool
* AWS Docker container registry cleaner
* AMI building tool
* Configuration management accelerator (Rust speed > Python speed)
* Infrastructure as code

I was expecting to hear more use cases of running on AWS. Perhaps it was the sample set of users I contacted.
Rumor has it there are big companies using Rusoto in applications. I'm hoping to get more information so we can make Rusoto even better.

## The future

We're going to keep on keeping on!  Some goals for 2018:

* Support all AWS services
* Improve ergonomics
* Increase performance
* Add more maintainers
* Flesh out https://rusoto.org with more information

## A big thank-you to all contributors!

We couldn't have done it without you. [See the list of contributors on GitHub.](https://github.com/rusoto/rusoto/graphs/contributors)