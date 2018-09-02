+++
date = "2018-09-02T00:28:53-07:00"
draft = true
title = "Build servers, Continuous Integration, Continuous Delivery"
+++

Ever hear someone say "CI/CD build pipeline" and weren't sure that that means, *exactly?*

Let's define and explore the concepts of:

* Build server
* Continuous Integration (CI)
* Continous Delivery (CD)

<!--more-->

## Three separate ideas

**Build server** is a server, servers or a system that automatically builds and tests code.

**Continuous Integration** and **Continuous Delivery** are both processes, guiding humans on what to do.

## Build server (build service?)

What it does:

Examples:

* Jenkins
* TravisCI
* CircleCI
* SemaphoreCI
* TeamCity

## Continuous integration

Guideline: branches are short lived. If a branch lives longer than a day or two, it's not continuous integration.

Examples:

Counter-examples:

git flow

## Continuous delivery

Guideline: code gets delivered to production within a day or two of it being approved.

Examples:

github flow
what ion does

Counter-examples:

git flow

## Separation of concepts

Build server by itself.

CI by itself.

CD by itself.