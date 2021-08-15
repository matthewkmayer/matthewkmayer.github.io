+++
date = "2021-08-14T00:28:53-07:00"
draft = false
title = "Closed loop development"
+++

## Closed loop development

Brace yourself for car analogies.

### Vehicles: open/closed loops

Modern cars have two modes of operation: open loop and closed loop.

Open loop: get engine inputs such as engine speed, air temperature and how much air is going into the engine. Apply a set of rules to determine how much fuel to put in.

Closed loop: using the above data *combined with information from the exhaust*, adjust how much fuel goes into the engine. Reading information from the engine's output lets the engine computer accurately determine how much fuel to supply.

### Software: open/closed loops

Software can be thought of similarly.

Open loop: code up a feature or make a bug fix. Ship it, don't check the changes did what they are supposed to do.

Closed loop: same coding of a feature or bug fix, but *verify the new behavior works*.

Automating verification via tests can work but sometimes a manual check is sufficient.

### My story of Doing It Wrong

As a new hire in a previous job, I was in a hurry to prove myself. I took a bug that looked straightforward, made my fix and shipped it. My open loop approach skipped verification of the fix. The bug remained. Customers were unhappy and my professional ego took a blow.

This situation taught the importance of closed loop development. All future features or bug fixes involved doing the work and *verifying* the change worked.

### One more analogy

Imagine your sink faucet is leaky. You call a plumber. They do some work, hand you a bill and leave. When you turn on the faucet, water sprays everywhere and the leak is still present. That's a bad look for the plumber - you immediately call their professionalism into question. The plumbing company gets a call back and they need to do the work again. Nobody is happy in this situation. Much time could be saved by the plumber testing their work before leaving.

### Do closed loop development

Make changes. Test the changes. Verify it works. Be the employee that says the change is done and everyone *knows* it's true.
