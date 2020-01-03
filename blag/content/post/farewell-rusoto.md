+++
date = "2020-01-02T00:28:53-07:00"
draft = false
title = "Farewell to Rusoto"
+++

TL;DR: I'm not working on Rusoto any more.

## A history of Rusoto

Anthony DiMarco and I started [Rusoto](https://github.com/rusoto/rusoto) in July of 2015. I knew we started the project shortly after Rust's 1.0 release, but I didn't realize we had our first commit less than two months after Rust 1.0 landed. Wow!

Some early parts of Rusoto are similar to how Rusoto looks now, others are quite a bit different. Our first iteration of code generation used a Python2 script to parse the botocore service definitions and output Rust. We support a few more services now and use a Rust program to generate the service definitions.

A big change was in crate structure: in [2017 we shattered the mega-crate](https://github.com/rusoto/rusoto/releases/tag/rusoto-v0.25.0). This split core behavior from the services, giving each service its own crate. Doing this helped keep compilation times sane and letting users only pay for what they needed: no more bringing in every service just to use one API call. This approach continued to pay dividends as Rusoto grew from 28k lines of code to over 860k lines for Rusoto 0.42.0.

I registered https://rusoto.org to have more documentation available for the project. It's been through a couple different site generators and is now using [mdbook](https://github.com/rust-lang/mdBook). The site has been a great resource since 2016 when the supplimental documentation was created.

## Accomplished goals

In the years since starting Rusoto, I've accomplished my goals for the project:

* Learn Rust: check.
* Run a non-trivial open source project: check.
* Deepen my understanding of AWS: check.

Time for me to move on to the next big thing.

Effective immediately, I'm setting a one-hour-a-week cap on Rusoto work. As of February 1st, 2020, I will only be available for hand-off assistance.

## What next?

For Rusoto: the project will continue one way or another. Either others step up to address maintainership needs or the project will fracture into a constellation of forks.

For me: address neglected duties and hobbies.

## Thanks for the time and effort, everyone

Rusoto didn't happen in a vacuum. There are 124 contributors to the project and counting. Rusoto has seen almost 500,000 downloads from crates.io and I've had a chance to work with a great many of excellent people. It's a bittersweet goodbye to the project but I wouldn't trade the journey for anything.

Keep hacking, keep working, keep being awesome. 👍
