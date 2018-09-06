+++
date = "2018-09-05T00:28:53-07:00"
draft = false
title = "Build servers, Continuous Integration, Continuous Delivery"
+++

Ever hear someone say "CI/CD build pipeline" and weren't sure that that means, *exactly?*

Let's define and explore the concepts of:

* Build server
* Continuous Integration (CI)
* Continous Delivery (CD)

<!--more-->

## Three separate ideas

*Build server* is a server, servers or a system that automatically builds and tests code.

*Continuous Integration* and *Continuous Delivery* are both processes, guiding humans on what to do.

## Build server (or build service)

Whenever code is pushed to the repository, the build server checks out that commit or branch and tries to build the code and run tests. The exact build and run steps depend on the language being used. For a Go project, the server may run `go build` followed by `go test ./...` and report back success or failure. This ensures what's checked in to source control can be built by someone else, not just the person who wrote the code.

Examples:

* Jenkins
* TravisCI
* CircleCI
* SemaphoreCI
* TeamCity

## Continuous integration

Guideline: branches are short lived. If a branch lives longer than a day or two, it's not continuous integration.

This process is for *continuously integrating* code back into the main branch. Keeping branches short lived prevents them from drifting out of date, causing merge conflicts or massive amounts of code to review. The bigger a team gets, the more likely two people are going to change something in the same spot, causing a merge conflict.

Usually this is a one time setup. After that, builds run automatically and report success or failure.

Examples:

* commit directly to master (careful!)
* [GitHub Flow](https://scottchacon.com/2011/08/31/github-flow.html) with short lived branches
* [Trunk Based Development](https://trunkbaseddevelopment.com/)

Counter-examples:

* [git flow](https://nvie.com/posts/a-successful-git-branching-model/): lots of moving pieces makes it difficult to integrate to the *release* branch every day or two.

## Continuous delivery

Guideline: code gets delivered to production within a day or two of it being approved.

Once code is accepted to the "main" branch, it should be shipped to production within a day or two. This requires deployments to be a polished, every day event. There are tricks that can be used to deliver code without having it available for use: feature flags are one technique to ship code but not make it available to consumers of the code.

Examples:

* [GitHub Flow](https://scottchacon.com/2011/08/31/github-flow.html)

Counter-examples:

* [git flow](https://nvie.com/posts/a-successful-git-branching-model/): again, lots of moving pieces complicates release process
* cargo-cult scrum: the [scrum guide](https://www.scrumguides.org/scrum-guide.html) doesn't preclude shipping during an iteration

## Separation of concepts

Now that we have an understanding of what each concept is, we can see how it can be used by itself: no part needs the other. To make this clearer, let's use a theoretical Go project on GitHub as a thought experiment.

One could have a build server by itself. In our theoretical example, we'll hook up TravisCI and configure it to run `go build && go test ./...`. Whenever code is pushed to GitHub, the TravisCI job will run and report back if it succeeded or failed.

To get CI by itself, a simple example is to commit directly to the `master` branch. The changes are continually integrated back into `master` because they are committed right there. No chance of branches getting out of date or having merge conflicts.

One way of doing CD by itself is to have a shell script checked into the repository. This script can do the repetitive work of creating a build and deploying it. The code can be continuously delivered if the committer runs the script after each commit.

## A real world example of separation

These concepts scale from single developer projects to massive corporations with a single repository for all projects. For example, I have a solo developer project that uses all three of these concepts separately.

For Continuous Integration I either commit directly to the `master` branch or make a branch that lives for a few hours, then merge it back to `master`.

I use [AWS CodeBuild](https://aws.amazon.com/codebuild/) to build, test and push a new Docker container of my code.

Continuous Deployment is done manually: when the new Docker image is ready, I do a manual deploy on [AWS ECS](https://aws.amazon.com/ecs/) to push my changes to production.

## Wrapup

*Continuous Integration* and *Continuous Delivery* are processes that tell humans what techniques to use with source control.

*Build server* is a server, servers or a system that automatically builds and tests code.

A followup blog post will show how these separate concepts can be put together and create an advantage bigger than the sum of its parts: a true *CI/CD pipeline*.
