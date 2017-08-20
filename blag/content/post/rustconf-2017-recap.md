+++
date = "2017-08-20T21:28:53-07:00"
draft = false
title = "RustConf 2017 recap"
+++

My recap of RustConf 2017 in Portland, Oregon.  Or: how I avoided hotel surge pricing during the eclipse weekend!

<!--more-->

## RustConf the second

Was told first conference for something is the best.  In this case the first one set a tone of possibility and potentials.  The second RustConf, in 2017, focused less on this and more on the progress made and a little more nitty-gritty details (epochs).

## Presentations recap

#### Opening keynote

Rust's compiler is continuing to improve.  Mozilla doesn't have a stranglehold on Rust: only 17% of the core Rust team works for Mozilla.  Epochs are a way to evolve the language in opt-in ways for potential breaking changes, but if a breaking change affects an old, unmaintained crate, it can be set to use the old epoch and newer code can use the new one.  No Python 2/3 split is going to happen.

RustBridge helps people get started programming in Rust.  Looks amazing and I hope to help with the effort.

#### A Tale of Teaching Rust

I missed the first few minutes of this due to hallway track shennanigans, but [Rustlings](https://github.com/carols10cents/rustlings) is apparently the way to go for teaching Rust.

#### Building Rocket

An excellent demystification of how Rocket works.  I was in the "too much magic" camp before but am now convinced Rocket simply automates how I'd hand-craft web server code.  Can't wait to have reduced-boilerplate for database interaction.  (Link to Rusoto + Rocket + Diesel blag post)

#### Shipping a Solid Rust Crate

Apparently one must include lots of anime in a crate. 😉  Good overview of what to have for a crate: documentation, documentation, documentation, a filled out Cargo.toml file, good README, sample code, a Code of Conduct, etc...

#### Menhir and Friends: the State of the Art of Parsing in Rust

This talk was over my head: I haven't had to write a parser since college and have forgotten all that content.

#### Type System Tips for the Real World

Highly polished, informative and fun presentation.  Just the right level of technical details and explainations for my current knowledge level.  Cleared up how Diesel operates and removes some of my fear it's "too much magic" like I view ActiveRecord.

#### Improving Rust Performance Through Profiling and Benchmarking

Nice deep dive into making Rust projects faster.  Can't wait to get to performance as a priority for my projects.

#### Fast, Safe, Pure-Rust Elliptic Curve Cryptography

Another presentation a bit over my head.  My grasp on elliptic curve cryptography is weak, to be generous.  The math involved could have used some animation to clearly explain how the steps happen.  This would have made it easier for me to follow.

#### Closing Keynote: Safe Systems Software and the Future of Computing

Microsoft's Singularity and Midori experiences.  A bittersweet presentation: much progress made on improving operating system safety that was killed before its time.  But hope for the future with lessons learned.

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