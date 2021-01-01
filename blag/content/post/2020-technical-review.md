+++
date = "2020-12-27T00:28:53-07:00"
draft = false
title = "2020 technical review"
+++

As the challenging year of 2020 wraps up, here's what happened in the past twelve months with a focus on my experience in technology.

## Rust

I've been writing Rust since mid 2015. That's over five years of working with the language. At the start of 2020 [I stepped back from my work on Rusoto](https://github.com/rusoto/rusoto/issues/1651), an AWS SDK for Rust. 

Looking back: every line of Rust I've written has been unpaid, volunteer work. Every line.

### What Rust taught me about Ruby

I was prejudiced against Ruby for most of my career. Why use a slow language with a syntax that looks nothing like my beloved C#?

This view was chipped away by watching people I respect do amazing work with Ruby. Unfamiliar things previously written off as magic are now seen as concise, descriptive ways to perform a task. One example is list comprehension, also known as lists from lists.

Rust drove this home with its iterators and methods on them: `filter`, `map`, `collect` and more. You can have your cake and eat it, too.

### What Rust taught me about Go

Compilation speed is important. Go nails this 100%.

A language that encourages error handling, even if it's verbose, is a step forward for coders.

Ease of deployment with a single precompiled binary improves quality of life an incredible amount.

## AWS

My disillusion with the cloud increased, especially with microservices in containers and their required orchestration. Maybe the monolithic Ruby on Rails peeps are onto something.

AWS is set up to encourage [Architecture Astronauts](https://www.joelonsoftware.com/2001/04/21/dont-let-architecture-astronauts-scare-you/). I've been and will likely continue to be guilty of this design pitfall. But I'm working on improving and simplifying my designs and architectures.

## Where I'm going

I'm heading out of the Rust community. When there's a fit for the language there's a tiny number of real competitors to Rust. My jobs haven't been in those areas and won't be for the foresable future. I'm going to work with languages better fit for the tasks at hand.

I am also easing out of the AWS community. After getting into use of AWS in 2013 or so, I even worked for a consultancy specialized in helping companies migrate to and use AWS better. In that role I earned one of the first 900 AWS Solutions Architect Professional level certifications in the world. A large driver of starting Rusoto was to learn more about AWS.

After years of spearheading adoptions of Rust with AWS from the open source side, I'm exhausted. The situations is far better now than it was when I started Rusoto but it's fallen short of where I thought it would be in 2021. I've no drive left to push it forward so I'm completely handing that off to others.

## 2021 plans

Double down on what works and is fit for purpose. Right now this looks like writing more Go and simplifying architectures.

No more coding for others for free. I'll keep writing code on my own time, but it will be to scratch my own itch and probably won't be open source.

Do something new. Expand my horizons. Perhaps it's time to use TypeScript in anger. Maybe it's implementing simpler system architectures that are easier to follow and reason about, on Microsoft Azure. Or dive into 3D printing and machining for classic cars and motorcycles, which is looking more and more rewarding.

## Despite the losses, keep trying

I've failed so many times and will keep failing. But each time I learn something new and won't repeat the same mistake.

Here's to 2021: a year better than 2020.
