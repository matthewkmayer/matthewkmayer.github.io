+++
date = "2019-12-26T00:28:53-07:00"
draft = false
title = "Advent of Code 2019, the wrong way (part one)"
+++

Per [my tweet](https://twitter.com/Motoblag/status/1203557633648553984): doing Advent of Code 2019 the wrong way. 😁

Today we're easing into #serverless.

## WHAT are we doing?

[Advent of Code](https://adventofcode.com/2019/about)!

`Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like.`

It's an excuse to play around with smaller coding problems and enjoy some recreational coding. In this series of blog posts, we're going to go about solving the problems the wrong way: as much serverless as possible.

## Yup, no servers

Instead of solving the puzzles on my computer, I'm going to use serverless as much as I can. This theme will certainly create hilariously overbuilt and overly complicated results, but that's going to be part of the fun this year.

## Day one, part one

There are two parts to day one: reading the problem input and making a function for calculating fuel load: ` Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.`

Let's ease into serverless by starting on [the Rust playground](https://play.rust-lang.org/).
