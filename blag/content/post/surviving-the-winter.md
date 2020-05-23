+++
date = "2020-05-23T00:28:53-07:00"
draft = false
title = "Surviving the upcoming winter"
+++

This is my perspective of the current state of Rusoto as a semi-retired maintainer and Amazon outsider. The post borrows the term `winter` from [AI Winter](https://en.wikipedia.org/wiki/AI_winter): a time period of decreased interest and funding.

## Amazon is hiring engineers for an AWS Rust SDK

Amazon has two public job openings for [Senior Software Development Engineers - AWS Rust SDK](https://www.amazon.jobs/en/jobs/1124901/senior-software-development-engineer-aws-rust-sdk). There's now enough customer demand for Amazon to make an official SDK for Rust. 

The fact Amazon has these job openings means they are getting serious about supporting Rust with AWS. It's a win for the Rust community: an official AWS SDK is on the horizon. There's also a downside for Rusoto.

## Winter from chilling effects

Operating under the assumption when Amazon releases an official AWS SDK for Rust, the community will (and should) move to use that. Amazon has at least two options: ask to take over Rusoto or make their own SDK. There has been no word on which path they will choose.

Amazon's unknown intentions cause a chilling effect on Rusoto. With the possibility of Amazon making their own SDK from scratch, any work on Rusoto has a short shelf life: it won't be the popular way to interact with AWS when an official SDK is released. Spending time and effort on a project that could be replaced by an official version at any moment is a gamble.

## Summer is coming

It's time to wait and see how Amazon acts instead of working on Rusoto. But summer is coming, one way or another.
