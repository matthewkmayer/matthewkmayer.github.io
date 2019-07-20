+++
date = "2019-07-19T00:28:53-07:00"
draft = false
title = "Rust as anti lock brakes"
+++

[Rust: a Language for the Next 40 Years](https://www.youtube.com/watch?v=A3AdN7U24iU) video put a seed of an idea on my head. I recommend watching the 55 minute video if you haven't had a chance yet.

The [recent posts on the Microsoft Security Response Center blog](https://msrc-blog.microsoft.com/2019/07/16/a-proactive-approach-to-more-secure-code/) helped the pieces click in my head.

Rust is like anti lock brakes (ABS) for motorcycles.

## Wait, what?

There are a lot more parallels than you'd expect at first glance.

Bringing a motorcycle to a stop as quickly as possible is a skill. The rider must have knowledge, experience and ability to understand what the motorcycle is doing. When this all lines up, motorcycles can stop in impressively short distances.

Things change when just one item is added to the mix: sand on the road, rider distraction, insufficient knowledge about how to brake. Without ABS, it's difficult to bring the bike to a stop when slowing over pavement with sand on it.

## The professional rider

Until the past few years, ABS equipped bikes took *longer* to stop than professional riders not using ABS, on perfect surfaces such as race tracks. Many riders assume they can match the professional's performance with inferior tires, training and road surface and eschew ABS for their bikes.

## Things go wrong

While ABS can be useful when everything is perfect, it really shines when things go wrong. Surface debris, abrupt inputs from a nervous or new rider, etc... all happen even though the rider doesn't want it to.

## Modern ABS

A modern motorcycle with ABS can brake about as well as a professional on a closed course/race track. Since it's also effective on the street where things vary and are never in a perfect state, there's nothing to lose to pick a bike with ABS over one that does not.

## C, Rust and ABS

When everything works perfectly, C and C++ work great. But like motorcycling, most coding isn't done on a closed course: software is released into a world where people intentionally throw sand in front of programs to see what they do.

With C and C++, memory related errors cause security issues. This is true even with training, code review, sanitizing tools, fuzzers and more. About 70% of CVEs Microsoft assigned from 2006 to 2018 were from memory related issues.

Rust is similar to anti lock brakes. Instead of relying on skill, training and perfect execution, have a backup strategy. Whether it's a computer cycling the brakes faster than a human ever could or leaning on a compiler to prevent a class of errors, it turns a disastrous event into a non-issue.

## The bona fides

Been motorcycling for 15 years, been an instructor for 13 years in multiple states.
