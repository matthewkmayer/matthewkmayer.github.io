+++
date = "2017-09-13T21:28:53-07:00"
draft = false
title = "Workflow Ramblings"
+++

While a software enginerd/pixie wrangler at heart, I've discovered I can't follow the stereotypical developer path: grab a ticket from Jira, work on it, shove the results to QA and repeat.  This approach leaves a tremendous amount of efficiency on the table.

Outside that narrow path, there's many methods to maximize the effects of the work put in.  Think force multiplier, but don't think 10x developer.

<!--more-->

## Overview and influences

The ideas to follow are nothing new.  Most of the influences come from the [Agile Manifesto](http://agilemanifesto.org/), especially the [12 principles page](http://agilemanifesto.org/principles.html), [kanban](<https://en.wikipedia.org/wiki/Kanban_(development)>) and years of interacting with the [AgilePDX](http://agilepdx.org/) community.

Here's six topics that act as force multipliers for software work:

* Favor describing behavior from the user's perspective
* Small tasks
* It's not done until it's in production
* Limit work in progress
* Working, delivered software now is better than perfect software sometime in the future
* Frequent deployments mean deployments are less scary

## Favor describing behavior from the user's perspective

Tickets should be from the perspective of the user and describe how they want the system to act.  Leave solutions out, just describe the behavior.

A common anti-pattern is to see "as a **member of the scrum team**, I want to **foo** so I can **bar**."  The customer, the person paying your salary, problem doesn't care about what the **foo** is.  What they *want* is working software.  Maybe faster software, or software with less bugs. Those are all user stories.  Switching data stores, moving to a different hosting provider, etc... are all things *the user doesn't care about*.  Make them technical tasks instead.

By keeping tickets written from the perspective of the user, this allows the team performing the work to figure out what the best way to deliver the requested changes are.  

Another anti-pattern is having a technical solution prescribed during ticket creation or grooming.  Pinning a solution to a problem when the least is known about the problem leads to pain, rework and technical debt.

## Small tasks

Favoring small tasks over large tasks give benefits such as:

* getting working software in front of the customer faster
* less work is thrown away if priorities change
* it's easier to see tickets that are stuck or not progressing

[Hamburger slicing](https://gojko.net/2012/01/23/splitting-user-stories-the-hamburger-method/) is one method to get a handle on making tickets smaller.  It's just a framework or tool and not the be-all, end-all, but a great place to start.

## It's not done until it's in production

It's far too common to see a ticket be marked as "done" when a pull request is created, or it's given to QA, or it's slated for release but not yet deployed.  

Here's why this isn't a good practice:

* if there's an issue discovered during code review or a bug found by QA, the developer needs to context switch from their new ticket back to the old ticket
* it encourages working on multiple tickets quickly instead of completing one at a time
* code completed but not yet deployed is not as valuable to the business or customer as deployed code

A common reason for delaying releases is "we use Scrum, we release at the end of the sprint." Luckily, [the Scrum guide](http://www.scrumguides.org/scrum-guide.html) doesn't say you have to wait until the end of the sprint to ship software.  Don't wait.

Feature flags can decouple releases from deployments, but don't let features sit too long without being flipped on.

## Limit work in progress

Goal: take a single ticket from "TODO" to "deployed and tested in production."  This maximizes overall throughput by minimizing context-switching.

Anti-pattern: a single person having three or more tickets they are "working on" at the same time.

For more information, there is a wonderful serious of posts on [personalkanban.com](http://www.personalkanban.com/pk/primers/the-basics-of-limiting-wip-why-limit-wip-series-post-1/) that applies to software development as well as using kanban for personal tasks.

## Working, delivered software now is better than perfect software sometime in the future

Agile Manifesto:

```
Our highest priority is to satisfy the customer
through early and continuous delivery
of valuable software.
```

A prototype system, feature or even bug workaround delivered to a customer today is better than a textbook-worthy, gold-plated perfection of software engineering in six months.  Get working software out early, incrementally add to it and continue to iterate.

## Frequent deployments mean deployments are less scary

Goal is to be able to deliver software as quickly as possible.  What's the lead time from an idea to a solution delivered to the customer?

Automation is your friend here.  Make deploys repeatable and minimize the amount of manual steps.  A human touching a keyboard to do a deploy is far less reliable than a tool.

Anti-pattern: "war room" deploys, done outside of normal business hours.

Anti-pattern: avoiding deployments because they are messy, error-prone and have a high failure rate.

## A thank you to my mentors

My usage of these techniques didn't arise in a vacuum.  I've been fortunate to have a few high-quality mentors in my career who I'm immensely grateful for their help growing my professional skills.

## Call to action

Try one of these changes!  Run an experiment with it.  Form your hypothesis, run experiment, gather data, see if the hypothesis still stands.
