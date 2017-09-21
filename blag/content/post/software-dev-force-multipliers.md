+++
date = "2017-09-20T21:28:53-07:00"
draft = false
title = "Software development force multipliers"
+++

While a software enginerd/pixie wrangler at heart, I've discovered the stereotypical developer path of *grabbing a ticket from Jira, working on it, shoving the results to QA and repeat* isn't for me.  The biggest reason is how this leaves a tremendous amount of efficiency on the table. This inefficient approach is difficult to reconcile with empathy for the customer buying and using the product.

My code writing speed has peaked: I can't write code any faster.  But I've found new tools and methods to maximize my efficiency.  Let's go over a sampling of the more important ones.

<!--more-->

## Overview and influences

The following ideas are nothing new.  Most of the influences come from the [Agile Manifesto](http://agilemanifesto.org/), especially the [12 principles page](http://agilemanifesto.org/principles.html), [kanban](<https://en.wikipedia.org/wiki/Kanban_(development)>) and years of interacting with the wonderful [AgilePDX](http://agilepdx.org/) community.

Here are six topics that act as force multipliers for software work:

* **Favor describing behavior from the user's perspective**
* **Small tasks**
* **It's not done until it's in production**
* **Limit work in progress**
* **Working, delivered software now is better than perfect software sometime in the future**
* **Frequent deployments mean deployments are less scary**

## Favor describing behavior from the user's perspective

Tickets should be from the perspective of the user and describe how they want the system to act.  Leave solutions out, just describe the behavior.

By keeping tickets written from the perspective of the user, this allows the team performing the work to figure out what the best way to deliver the requested changes are.  More options tend to be better and allow for experimentation and learning.

Anti-pattern: ticket written similar to "as a **member of the scrum team**, I want to **foo** so I can **bar**."  The customer, the person paying your salary, problem doesn't care about what the **foo** is.  What they *want* is working software.  Maybe faster software, or software with less bugs. Those are all user stories.  Switching data stores, moving to a different hosting provider, etc... are all things *the user doesn't care about*.  Make them technical tasks instead.  **Corollary:** many technical tasks and not many user stories is an anti-pattern.

Anti-pattern: having a technical solution prescribed during ticket creation or grooming.  Pinning a solution to a problem when the least is known about the problem leads to pain, rework and technical debt.

## Small tasks

Favoring small tasks over large tasks give benefits such as:

* getting working software in front of the customer faster
* less work is thrown away if priorities change
* it's easier to see tickets that are stuck or not progressing

These points are prominent in [Lean software development](https://en.wikipedia.org/wiki/Lean_software_development).

[Hamburger slicing](https://gojko.net/2012/01/23/splitting-user-stories-the-hamburger-method/) is one method to get a handle on making tickets smaller.  It's just a framework or tool and not the be-all, end-all, but a great place to start.  This works best with the above **Favor describing behavior from the user's perspective**.

Anti-pattern: a task or ticket being "in progress" for weeks or months.  This is usually caused by large unknowns, which can be solved by a [spike](<https://en.wikipedia.org/wiki/Spike_(software_development)>) or experiment, or describing a very complex problem.

## It's not done until it's in production

It's far too common to see a ticket be marked as "done" when a pull request is created, or it's given to QA, or it's slated for release but not yet deployed.

Downsides:

* if there's an issue discovered during code review or a bug found by QA, the developer needs to context switch from their new ticket back to the old ticket
* it encourages working on multiple tickets quickly instead of completing one at a time
* code completed but not yet deployed is not as valuable to the business or customer as deployed code

A common reason for delaying releases is "we use Scrum, we release at the end of the sprint." Luckily, [the Scrum guide](http://www.scrumguides.org/scrum-guide.html) doesn't say you have to wait until the end of the sprint to ship software.  Don't wait.

[Feature flags](https://launchdarkly.com/featureflags.html) can decouple releases from deployments, but don't let features sit too long without being flipped on.

## Limit work in progress

Goal: take a single ticket from "TODO" to "deployed and tested in production."  This maximizes overall throughput by minimizing context-switching.

For more information, there is a wonderful serious of posts on [personalkanban.com](http://www.personalkanban.com/pk/primers/the-basics-of-limiting-wip-why-limit-wip-series-post-1/) that applies to software development as well as using kanban for personal tasks.

Anti-pattern: a single person having three or more tickets they are "working on" at the same time.

## Working, delivered software now is better than perfect software sometime in the future

The Agile Manifesto states:

```
Our highest priority is to satisfy the customer
through early and continuous delivery
of valuable software.
```

A prototype system, feature or even bug workaround delivered to a customer today is better than a textbook-worthy, gold-plated perfection of software engineering in six months.  Get working software out early, incrementally add to it and continue to iterate.

## Frequent deployments mean deployments are less scary

The goal is to be able to deliver quality software as quickly as possible.  What's the lead time from an idea to a solution delivered to the customer?  If it takes six months to go from the company going all-in on an idea to something the customer can use, it's too long.  Customer needs will change, the business landscape will change, more nimble competitors will swoop in and delivery value first.

Automation is your friend here.  Make deploys repeatable and minimize the amount of manual steps.  A human touching a keyboard to do a deploy is far less reliable than a tool.

Not sure where to start?  Take notes on the most painful items during the next deploy.  Fix one before the next deployment.  Repeat until the biggest headaches are gone.

Anti-pattern: "war room" deploys, done outside of normal business hours.

Anti-pattern: avoiding deployments because they are messy, error-prone and have a high failure rate.

## Call to action

Try one of these changes!  Run an experiment with it.

Something to keep in mind: when all these ideas are combined, they also have a force multiplying effect.  The more you implement, the more effective they, thus you, become.
