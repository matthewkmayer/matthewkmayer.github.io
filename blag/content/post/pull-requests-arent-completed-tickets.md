+++
date = "2025-04-14T00:28:53-07:00"
draft = false
title = "Pull requests aren't (necessarily) completed tickets"
+++

Cutting the cord between pull requests (PRs) and tickets.

<!--more-->

## The issue

"One pull requests per ticket!"

A ticket is *what* to do, coding and branching strategies are *how* to do it.

## What does a ticket represent?

A new or correct behavior in a system.

## What does a pull request mean?

A pull request is a signal for a conversation or review, not necessarily "please merge this." It can be both.

Scott Chacon [says](https://scottchacon.com/2011/08/31/github-flow/):

> Actually, we use it more as a branch conversation view more than a pull request. You can send pull requests from one branch to another in a single project (public or private) in GitHub, so you can use them to say “I need help or review on this” in addition to “Please merge this in”.

Paul Hammant [says](https://trunkbaseddevelopment.com/continuous-review/):

> A PR is one or more commits towards a goal described in an accompanying piece of text. The act of creating the PR from the branch signals the end (or a pause) in work, and the wish for the reviewers to get busy 

## What makes a good pull request?

Smaller is generally better than larger: easier to review, less chance of merge conflicts, allows short-lived branches.

## If a ticket can have many pull requests, how can we know when the ticket is done?

The same way for a single PR: verify the desired behavior is present.
