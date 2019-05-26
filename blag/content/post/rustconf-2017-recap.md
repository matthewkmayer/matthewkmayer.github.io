+++
date = "2017-08-20T21:28:53-07:00"
draft = false
title = "RustConf 2017 recap"
+++

My recap of RustConf 2017 in Portland, Oregon.  Or: how I avoided hotel surge pricing during the eclipse weekend!

## RustConf the second

I've been told the first conference for community or technology is the best.  I'm happy to report I don't believe it applies in this case!  The first RustConf set a tone of possibility and potentials whereas RustConf 2017 focused more on the progress made in the language, ecosystem and community.  As a whole, the conference was a great reflection of the language and community: where we've come from, where we're at and what's next.

## Presentations recap

#### Opening keynote

Rust's compiler is continuing to improve.  Mozilla doesn't have a stranglehold on Rust: only 17% of the core Rust team works for Mozilla.  Epochs are a way to evolve the language in opt-in ways for potential breaking changes, but if a breaking change affects an old, unmaintained crate, it can be set to use the old epoch and newer code can use the new one.  No Python 2/Python 3 split is going to happen.

RustBridge helps people get started programming in Rust.  Looks amazing and I hope to help with the effort.

#### A Tale of Teaching Rust

I missed the first few minutes of this due to hallway track shennanigans, but [Rustlings](https://github.com/carols10cents/rustlings) is apparently the way to go for teaching Rust.

#### Building Rocket

An excellent demystification of how Rocket works.  I was in the "too much magic" camp before but am now convinced Rocket simply automates how I'd hand-craft web server code.  Can't wait to have reduced-boilerplate for database interaction.  For an example, my walkthroughs of a super-simple Rocket and Diesel web server can be found here: [part one](https://matthewkmayer.github.io/blag/public/post/rusoto-rds-walkthrough/) and [part two](https://matthewkmayer.github.io/blag/public/post/rusoto-rds-mk2/).

#### Shipping a Solid Rust Crate

Apparently one must include lots of anime in a crate. 😉  Good overview of what to have for a crate: documentation, documentation, documentation, a filled out Cargo.toml file, good README, sample code, a Code of Conduct, etc...  My takeaway is Rusoto checks almost all the boxes, so we're on the right path!

#### Menhir and Friends: the State of the Art of Parsing in Rust

This talk was over my head: I haven't had to write a parser since college and have forgotten all that content.  I wish the talk started with a simple use case for parsers and then a comparison of a parser in Rust and a parser in another language to show why one would choose Rust in this case.

#### Type System Tips for the Real World

Highly polished, informative and fun presentation.  Just the right level of technical details and explainations for my current knowledge level.  Cleared up how Diesel operates and removes some of my fear it's "too much magic" like I view ActiveRecord.

#### Improving Rust Performance Through Profiling and Benchmarking

Nice deep dive into making Rust projects faster.  Can't wait to get to performance as a priority for my projects.

#### Fast, Safe, Pure-Rust Elliptic Curve Cryptography

Another presentation a bit over my head.  My grasp on elliptic curve cryptography is weak, to be generous.  The math involved could have used some animation to clearly explain how the steps happen.  This would have made it easier for me to follow.

#### Closing Keynote: Safe Systems Software and the Future of Computing

Microsoft's [Singularity](https://en.wikipedia.org/wiki/Singularity_(operating_system)) and [Midori](https://en.wikipedia.org/wiki/Midori_(operating_system)) experiences.  A bittersweet presentation: much progress made on improving operating system safety that was killed before its time.  But hope for the future with lessons learned.  I could listen to Joe Duffy talk all day. 🙂

## Hallway track

The "Hallway track" at a conference refers to the conversations happening in hallways instead of the main events and presentations.  I made my own "table track."  Another maintainer of [Rusoto](https://github.com/rusoto/rusoto/) happened to sit at the same table so we caught up a little.  Another person who's been in touch about the same project contact me before the conference and we got a chance to chat in person, which is always great.

The person sitting to my left gave me an excellent view into Haskell.  Funny how diverse the attendees' technical knowledge is.

## Unofficial afterparty

Last year I made a tweet right as the conference came to a close, suggesting attendees meet up to continue talking with each other.  We took over most of [Tugboat Brewing](http://www.tugboatbrewco.com/) and had a great time.  This year I organized it a bit better: I called ahead to various venues and reserved space for us at a sports bar and grill near the conference location.  Due to better publicizing, the unofficial afterparty had more people there and hope everyone had a great time sharing a drink of their choice with fellow Rustaceans.

## My trick to avoiding pricy hotels

RustConf's venue was a few blocks away from the office so I took light rail in and enjoyed a fun day. 😉

## Here's to another great year for Rust!
