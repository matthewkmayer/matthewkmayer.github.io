+++
date = "2019-05-10T00:28:53-07:00"
draft = false
title = "How much testing is enough?"
+++

Most software developers will tell you that tests are good. More tests are better. How much is enough? How can one recognize diminishing returns? *What* types of tests should be written? Here are some guidelines for determining what a team's "enough" level of testing is.

## Definitions and goals

* unit test: tests a single unit of code. Think a single function or method. If there are more than one or three mocks being used, it's probably not a unit test
* functional test: tests behavior of code, staying withing a single process
* integration test: a test that crosses a process boundary. For example: code that talks to a real database
* end to end/smoke test: test the entire stack: apply power and see if any smoke comes out

The goal is to get enough test coverage as quickly as possible. A test suite that takes a half hour to run makes it hard to stay focused and stay in flow. Insufficient test coverage means second guessing and worrying about deployments.

## Unit tests

These should be *fast*. Most of a codebase's tests should be unit tests. I've heard a guideline of one millisecond (1 ms) per unit test. I like that aggressive goal. It means a codebase could have 10,000 tests and they can run in 10 seconds.

## Functional tests

Testing the function of the code can be performed at the API layer. For backend HTTP services, this means using the HTTP calls the consumers of the service would call. Using an in-memory store instead of an external store such as a database means these can quickly run. 100ms each should be an easy level to reach; 25ms is probably closer to what one should aim for. 1,000 functional tests can run in 25 seconds with 25ms per test.

## Integration tests

Since integration tests involve two operating system processes, usually the code under test and its backing store, these can take 50 to 100 milliseconds to run. At 100ms each, 100 integration tests can run in 10 seconds as well.

It's possible to reuse the functional tests as integration tests: set up the system under test to use the real backing store and run the same tests.

## End to end/smoke tests

Smoke tests tend to be expensive. It may take one to ten seconds for each test. This means there can only be so many of them: make these test the *most important parts of your system*. What would cause income to screech to a halt? What would make customers grumpy? Ensure those situations don't happen with smoke tests.

## Ok, but how many tests? Really?

My personal line in the sand is all happy paths should be tested. For every feature and behavior the project has, the path where everything works right *must* be tested. This is the bare, bare minimum. Anything less requires a shift from new features toward writing tests to achieve this minimum requirement.

My comfort level is when bug fixes have regression tests. Keeping a bug from happening twice is important: reproduce the bug with a test, show the broken behavior, then fix it.

## General pitfalls

Note the omission of code coverage metrics. While these can help, code coverage can also be abused. I've seen projects with 99% code coverage still not behave right. Chasing a code coverage number can be detrimental to having a fast running suite of effective tests. If one must use code coverage metrics, use branch coverage: make sure all branches of `if`, `switch`, etc... statements are covered by tests.

Keep the approximate shape of the test pyramid: have more unit tests than any other. Smoke tests should have the least number of tests, due to their slowness.

When tests flap between pass and fail, step back and rethink how the code is tested. False positives are frustrating and slow down development work: fix them!

## Move to the left

Reproduce a bug with a smoke test? Make it a functional test. Better yet, make it a unit test!

Think of the build pipeline as a series of steps, left to right:

`compile => unit test => functional test => integration test => smoke test`

Turning a bug fix test from a smoke test taking one second to run to a unit test that takes one millisecond to run is a huge win. This prevents the urge to remove a slow moving test that always passes.

## Reuse tests

One can use testing frameworks that help with test code reuse. For example, correctly writing [Cucumber tests](https://cucumber.io/) allows them to be used as functional tests, integration tests and sometimes smoke tests as well. Write tests once, run them in multiple scenarios.

## Take action

Here's one way to get started with "enough" tests:

1. Make unit tests for all happy path scenarios
2. Write Cucumber tests for functional and integration test use
3. Move smoke tests into unit, functional or integration tests if possible
4. Find the slowest tests and find ways to make them run faster: amortize costs by leaving various resources running between tests, refactor code to test smaller chunks, use a faster build/test service, parallelize testing

Happy testing!