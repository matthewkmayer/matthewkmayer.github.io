+++
date = "2018-12-04T00:28:53-07:00"
draft = false
title = "Repos for getting hired"
+++

I'm often involved in hiring. When GitHub links are on a resume, I look at pinned repos and source repos.
Here's a checklist of important items to have in a GitHub public repository to catch the eye of reviewers. Or at least my eye when hiring, and why it's important.

<!--more-->

## Your goal, my goal

Yours: get hired. Mine: make an informed decision about hiring someone who can do the job.

## The checklist, in no particular order

* show (working) example code
* have a link to a deployed example
* make the READMEs useful for people who don't know what the project is or why it's important
* show off build badges
* published docker containers
* got a blog from a github repo? make sure there's a link to the public page

### Show (working) example code

A library should have working example code prominent in the README file. This indicates the project has been used in one fashion or another. Bonus points for showing how someone would solve the problem without the project and show how much better it is to use the solution the project provides. I find this important because I can get a gut feel for public API design or how a problem is approached by seeing the result as an end user.

### Have a link to a deployed example

There's an awful lot of repos consisting of a collection of HTML and JavaScript without an easy example of how to view the web page. Static pages are cheap to free these days: toss the page up somewhere and put a link to it. This is valuable to me because I've seen so many different ways to have demonstrable code, I'm not inclined to spend more than a few minutes trying to figure it out.

### Make the READMEs useful for people who don't know what the project is or why it's important

When a repo says `Solves baz with aplomb!` it doesn't state much unless one already knows what `baz` is. I **love** repos that state what the project is, why it was created and its current state and trajectory. The ability to communicate over text is very important to software development.

### Show off build badges

There's plenty of free build services for public repos. Being able to build a project on a different machine is vitally important and when repo after repo is missing this, it doesn't give a positive sign. Bonus points if a project billing itself as cross-platform **shows** it via build service badges.

### Published docker containers

In a DevOps world, where `you write it you run it`, Docker is almost the standard for shipping code. By understanding enough of how that works to package and publish a container, a repo stands out from others that are missing this, when applicable.

### Got a blog from a github repo? Link to the public page

Not everyone knows `username.github.io` is a public web page. Toss a link in the repo for that blog. Reducing friction for people finding out what you have to say helps them find it.

## Check those boxes

This is not an exhaustive list. It's a collection of aspects a repository should have in order to send a good signal to someone reviewing it for hiring purposes. All other things equal, if your repo(s) checks them all and another candidate misses them all, you're a step ahead of them.
