+++
date = "2021-08-09T00:28:53-07:00"
draft = false
title = "Closed loop development"
+++

## Closed loop development

Brace yourself for car analogies.

### Vehicles: open/closed loops

Modern cars have two modes of operation: open loop and closed loop.

Open loop: get data about engine inputs such as engine speed, air temperature, how much air is going into the engine, etc... Apply a set of rules to determine how much fuel to put in.

Closed loop: using the above data *combined with information from the exhaust*, adjust how much fuel goes into the engine. Reading information from the engine's output lets the engine computer accurately determine how much fuel to supply.

### Software: open/closed loops

Software can be thought of similarly.

Open loop: code up a feature or make a bug fix. Ship it, don't check the changes did what they are supposed to do.

Closed loop: same coding of a feature or bug fix, but *verify the new behavior works*.

Automating the check can work but sometimes a manual check is sufficient.
