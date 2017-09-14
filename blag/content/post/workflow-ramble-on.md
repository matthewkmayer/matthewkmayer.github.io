+++
date = "2017-09-13T21:28:53-07:00"
draft = false
title = "Workflow Ramblings"
+++

While a software enginerd/pixie wrangler at heart, I've found I can't just grab tickets, work on them, shove the results to QA and call it a day.  By changing how I approach problems to solve, I've picked up a few methods to maximize the effects of the work put in.  Think force multiplier, but don't think 10x developer.

<!--more-->

## Overview and influences

Agile Manifesto, especially the 12 principles page.

Kanban: limit work in progress, do pull-based task movement, slack time.

Business side: show progress early and often with minimal ceremony.  Ceremony is time spent not working.

* Favor describing behavior from the user's perspective
* Small tasks
* It's not done until it's in production
* Limit work in progress
* Working, delivered software now is better than perfect software sometime in the future
* Frequent deployments mean deployments are less scary

## Favor describing behavior from the user's perspective

Tickets don't need to always follow the "as a $persona, I want to $foo so I can $bar."  The purpose of the format is to encourage thinking from a user perspective.  

A common anti-pattern is to see "as a member of the scrum team, I want to $foo so I can $bar."  The customer, the person paying your salary, problem doesn't care about what the $foo is.  What they *want* is working software.  Maybe faster software, or software with less bugs. Those are all user stories.  Switching data stores, moving to a different hosting provider, etc... are all things *the user doesn't care about*.  Make them technical tasks instead.

## Small tasks

Resources: hamburger slicing.

## It's not done until it's in production

Scrum guide doesn't say you have to wait until the end of the sprint to ship software.  Don't wait.

What if it doesn't work after deployment?  If the last time someone worked on it was a month ago, how much time is wasted context-switching to the task?  How much less is spent if you last looked at the code this morning?

Feature flags can decouple releases from deployments, but don't let features sit too long without being flipped on.

## Limit work in progress

Anti-pattern: a single person having three or more tickets they are "working on" at the same time.

Goal: take a single ticket from "TODO" to "deployed and tested in production."  This maximizes overall throughput by minimizing context-switching.

## Working, delivered software now is better than perfect software sometime in the future

Agile Manifesto:

```
Our highest priority is to satisfy the customer
through early and continuous delivery
of valuable software.
```

A prototype system, feature or even bug workaround delivered to a customer today is better than a textbook-worthy, gold-plated perfection of software engineering in six months.  Get working software out early, incrementally add to it and continue to iterate.

## Frequent deployments mean deployments are less scary

Anti-pattern: "war room" deploys, done outside of normal business hours.

Anti-pattern: avoiding deployments because they are messy, error-prone and have a high failure rate.

Goal is to be able to deliver software as quickly as possible.  What's the lead time from an idea to it being delivered to the customer?

Automation is your friend here.

## A thank you to my mentors

My usage of these techniques didn't arise in a vacuum.  I've been fortunate to have a few high-quality mentors in my career who I'm immensely grateful for their help growing my professional skills.

## Call to action

Try one of these approaches!  Run an experiment with it.  Form your hypothesis, run experiment, gather data, see if the hypothesis still stands.  See: GROWS method.
