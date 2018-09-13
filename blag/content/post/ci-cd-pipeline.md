+++
date = "2018-09-12T00:28:53-07:00"
draft = false
title = "CI/CD Pipeline"
+++

[Previously](https://matthewkmayer.github.io/blag/public/post/build-ci-cd/), we defined Continuous Integration (CI), Continuous Deployment (CD) and build servers. Now we'll see how combining these concepts creates something greater than its parts: a true CI/CD pipeline.

<!--more-->

## Previously covered

*Continuous Integration* and *Continuous Delivery* are processes that tell humans what techniques to use with source control.

*Build server* is a server, servers or a system that automatically builds and tests code.

Putting these concepts together require *people* following *processes* so automated tasks execute well.

## Tying it together

By continuously integrating back to a shared or common branch, developers avoid spending time dealing with merge conflicts, or worse: code that's written and thrown away without customers seeing the results.

Continuous deployment reduces lead time for bug fixes and features being available for customers to use. "Lead time" is from Lean Software Development, which refers to the time between the delivery team knowing about a bug or desired feature and when it's available to customers. Lower lead time is a competetive advantage over other companies.

Build servers can be leaned on for monotonous, repetitive tasks. Computers excel at this whereas people are not as good. Use computers to automatically run unit tests, functional tests, deployments and functional testing of code after deployment. They won't forget to run `go test` or `rspec`.

## Build steps are the easiest part

Getting a build server to do repetitive tasks is the easiest part of making a true CI/CD pipeline. CI is harder and requires discipline and skill. CD is in the middle, requiring tooling to help and getting buy-in from parts of the business. Sometimes, batching CD helps. Think of a metered freeway onramp: things happen at a regular cadence, controlled to prevent massive amounts of change to keep the overall flow working.

In the past few years I've found good success with a *minimum* of deploying to production twice a week, using [release-party](https://github.com/matthewkmayer/release-party-BR). Almost all friction is gone from shipping code to production: run the tool on Tuesdays and Thursdays, inform team members there are release pull requests waiting for review, get them approved and merged into our `release` branch.

## Where does the delivery team spend its time?

When a team spends its time dealing with merge conflicts or being confused as to which branch bug fixes are applied, they waste time. Make it clear where fixes and features are, and get them to customers after enough testing, as quickly as possible. CI and CD make this easy to achieve.

## Multiplying force

With the disciple of continuous integration, multiplied by continuous deployment run on a build service, the overall ability to ship bug fixes and new features results in something greater than its parts.

By shipping bug fixes and features faster, reducing or eliminating time wasted by the delivery team and leaning on computers to do repetitive, error-prone tasks, these ideas make a true CI/CD pipeline that is greater than the sum of its parts.

A team putting these all together has a competitive advantage over a bigger and smarter team working on the same problem. More features, less bugs, delivered faster, is an edge to make a company succeed.
