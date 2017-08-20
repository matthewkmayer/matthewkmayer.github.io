+++
date = "2017-08-20T21:28:53-07:00"
draft = false
title = "RustConf 2017 recap"
+++

My recap of RustConf 2017 in Portland, Oregon.  Or: how I avoided hotel surge pricing during the eclipse weekend!

<!--more-->

## RustConf the second

Was told first conference for something is the best.  In this case the first one set a tone of possibility and potentials.  The second RustConf, in 2017, focused less on this and more on the progress made and a little more nitty-gritty details (epochs).

## Hallway track

Meetin' people is great.

## bullet point time

* Rust epochs - Rust 2019 should have a polished release with stable rustc, cargo, IDE tooling, etc... No ecosystem split - the Python 2/3 lesson has been learned
* non-lexical lifetimes have finally been explained in a way I understand :)
* async/await in the works
* Rust book coming out through NoStarch Press soon
* RustBridge - Community Team runs these, gets people started with Rust.
* IntelliJ IDEA + Rust plugin is first class citizen - try it out!
* Is there an envconfig-like library for Rust?
* Rustlings - web site with practice problems
* Chatted with Tim, next to me, about Haskell, which I've never touched
* Rocket's demystification of code gen was excellent
* I need to do a lot of exercises around `.map` because I'm out of practice
* Rocket 0.4's db interaction removes much boilerplate
* Rocket won't be on nightly forever!
* Folks want the builder pattern for requests in Rusoto
* I need to revisit error-chain
* herald.community.rs
* Blog post ideas: Rusty cucumbers, Rust test-driven development/test-first, traits instead of mocks, add Rust to my Strangler Application presentation
* Presentations: best ones explain why you should be excited/interested
* `.expect` is eagerly evaluated.  Parallels to Go's `defer()`
* I need to use `cargo bench` more
* I need to look up what `taskset` does
* Godbolt's Rust support lets you dive deep into compiler output
* Animations in presentations can help with more complex ideas
* `Rust, but verify` - from the crypto presentation
* I could listen to Joe Duffy talk all day
* Are there Rust circuit breakers?

## After party

Always amazed at the people who come to RustConf.  Folks are smart and passionate about their craft.

## My trick to avoiding pricy hotels

RustConf's venue was a few blocks away from the office so I took light rail in and enjoyed a fun day. 😉

## Personal notes/project ideas

What will it take to make Rust viable in cloud-native app space?

* strong http servers
* something like Rocket, Gotham, etc...
* jwt libs
* envconfig: easy environment configs (12 factor app)
* AWS SDK 😉
* circuit breakers