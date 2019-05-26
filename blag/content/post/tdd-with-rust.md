+++
date = "2017-10-19T21:28:53-07:00"
draft = false
title = "Test driven development with Rust"
+++

[Test Driven Development](https://en.wikipedia.org/wiki/Test-driven_development) (TDD) encourages better software design. When the desired behavior is known and expressible, it's highly effective to make modular and easily tested code.

Let's take a look at using TDD with Rust, using [release-party](https://github.com/matthewkmayer/release-party-BR) as an example.

## What we're changing

Release-party is a command line tool I made for my day job.  We've got a fair amount of repositories on GitHub: one for each microservice.  Our deployments are automated through TravisCI: the `master` branch is deployed to our testing environment and the `release` branch is deployed to production.  

This is a hybrid of [GitHub flow](http://scottchacon.com/2011/08/31/github-flow.html) and [trunk based development](https://trunkbaseddevelopment.com/).  Release-party automates the process of going to each repository in the organization, seeing if `release` is behind `master` and if so, create a new pull request.  It quickly became a time sink to do that manually, multiple times a week, and release-party will do the inspection and pull request creation as needed.

One required argument to the tool is the GitHub organization.  Until recently, users had to supply the entire GitHub API URL, such as `https://api.github.com/orgs/ORG-HERE/repos`.  To make things easier on users, I modified it to take just the org: `ORG-HERE`.  However, when run with the previous argument of the entire URL, it unceremonously keeled over with an unhelpful error message.

Here's how it looks when supplying the old style of argument:

```bash
RP_GITHUBTOKEN=ghtoken release-party-br --org "https://api.github.com/orgs/ORG-HERE/repos"
thread 'main' panicked at 
'expected repos: "Couldn\'t deserialize repos from github: invalid type: 
map, expected a sequence at line 1 column 1"', src/libcore/result.rs:860:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Our goal: be *helpful* if we are supplied the old style of argument.

## How we want it to behave

First pass: we'll check the `org` argument to see if it contains a URL.  If so, let the user know they just need to supply the org name.

## Red, green, refactor

Test first:

```rust
#[test]
fn handle_malformed_org() {
  assert_eq!(false, org_is_just_org("https://api.github.com/orgs/ORG-HERE/repos"));
}
```

`org_is_just_org` is a new function to check if a string contains just the org name.  In this test we pass it the complete API URL, which should fail.

Dogmatic TDD states we should run the tests now and watch it fail.  This is done by running `cargo test` and seeing the compilation error.  I prefer a less dogmatic approach: let's put in the `org_is_just_org` function, place it in the code base but the function will always return true:

```rust
fn org_is_just_org(org: &str) -> bool {
  true
}
```

It's called in another function:

```rust
fn make_org_url(matches: &clap::ArgMatches) -> String {
    let org = matches
        .value_of("ORG")
        .expect("Please specify a github org");

    if !org_is_just_org(&org) {
        panic!("Please make org just the organization name.")
    }

    format!("https://api.github.com/orgs/{}/repos", org)
}
```

Now when we run `cargo test` we see it fail:

```bash
test tests::handle_malformed_org ... FAILED
...
failures:

---- tests::handle_malformed_org stdout ----
	thread 'tests::handle_malformed_org' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`', src/main.rs:176:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::handle_malformed_org

test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

On to making the test pass!  Let's modify `org_is_just_org`:

```rust
fn org_is_just_org(org: &str) -> bool {
    if org.contains("https://api.github.com") {
        return false;
    }
    true
}
```

That looks better.  What does `cargo test` say?

```bash
running 7 tests
test github::tests::no_next_link ... ok
test github::tests::has_next_link ... ok
test github::tests::no_requests_left ... ok
test github::tests::plenty_of_requests_left ... ok
test tests::handle_malformed_org ... ok
test tests::get_ignored_repos_happy_path ... ok
test github::tests::finds_next_link ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Success!

Since our change was so small, there's little to nothing to refactor.  While we're in here, let's also ensure the happy path is tested:

```rust
#[test]
fn handle_okay_org() {
    assert_eq!(true, org_is_just_org("ORG-HERE"));
}
```

We're green on all the tests.

## How we want it to behave, phase two

Panicking with a message of `Please make org just the organization name.` isn't super helpful.  It's correct but we can do better.  Let's try to make a suggestion to the user.  If they provide `https://api.github.com/orgs/ORG-HERE/repos` let's respond with something along the lines of `did you mean ORG-HERE?`.

## Red, green, refactor

Our new test:

```rust
#[test]
fn suggestion_for_org() {
    assert_eq!("Try this", 
        suggest_org_arg("https://api.github.com/orgs/ORG-HERE/repos").unwrap());
}
```

New function:

```rust
fn suggest_org_arg(org: &str) -> Result<String, String> {
    Err("Can't make a suggestion".to_owned())
}
```

And the new function slotted in where we want it:

```rust
fn make_org_url(matches: &clap::ArgMatches) -> String {
    let org = matches
        .value_of("ORG")
        .expect("Please specify a github org");

    if !org_is_just_org(&org) {
        match suggest_org_arg(&org) {
            Ok(suggestion) => panic!("Try this for the org value: {}", suggestion),
            Err(_) => panic!("Please make org just the organization name."),
        }
    }

    format!("https://api.github.com/orgs/{}/repos", org)
}
```

We can see our test fail:

```bash
...
test tests::suggestion_for_org ... FAILED
...

failures:

---- tests::suggestion_for_org stdout ----
	thread 'tests::suggestion_for_org' panicked at 'called `Result::unwrap()` on an `Err` value: "Can\'t make a suggestion"', src/libcore/result.rs:906:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::suggestion_for_org

test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

And to make it pass:

```rust
fn suggest_org_arg(org: &str) -> Result<String, String> {
    if org.starts_with("https://api.github.com/orgs/") && org.ends_with("/repos") {
        return Ok("Try this".to_owned());
    }
    Err("Can't make a suggestion".to_owned())
}
```

```bash
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Yes!  We now have a passing test.  But the test doesn't actually test our end state: it doesn't return the suggestion.  Let's fix that:

```rust
#[test]
fn suggestion_for_org_happy() {
    assert_eq!("Try this: ORG-HERE", 
        suggest_org_arg("https://api.github.com/orgs/ORG-HERE/repos").unwrap());
}
```

`cargo test` will show us that fails because our implementation doesn't return the suggestion.  On to that:

```rust
fn suggest_org_arg(org: &str) -> Result<String, String> {
    if org.starts_with("https://api.github.com/orgs/") && org.ends_with("/repos") {
        let suggestion = org.replace("https://api.github.com/orgs/", "").replace("/repos", "");
        return Ok(format!("Try this: {}", suggestion).to_owned());
    }
    Err("Can't make a suggestion".to_owned())
}
```

`cargo test` shows all green:

```bash
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

For completeness we'll also test some error cases for the function:

```rust
#[test]
fn suggestion_for_org_sad() {
    assert_eq!(true, suggest_org_arg("https://api.github.com/orgs/ORG-HERE/").is_err());
    assert_eq!(true, suggest_org_arg("http://api.github.com/orgs/ORG-HERE/").is_err());
    assert_eq!(true, suggest_org_arg("api.github.com/orgs/ORG-HERE/repos").is_err());
}
```

Once again a `cargo test` shows the new test demonstrates our function works as expected:

```bash
running 10 tests
test github::tests::no_next_link ... ok
test github::tests::has_next_link ... ok
test github::tests::no_requests_left ... ok
test github::tests::plenty_of_requests_left ... ok
test tests::handle_malformed_org ... ok
test tests::handle_okay_org ... ok
test tests::get_ignored_repos_happy_path ... ok
test tests::suggestion_for_org_happy ... ok
test tests::suggestion_for_org_sad ... ok
test github::tests::finds_next_link ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## The proof of the pudding is in the eating

One last manual check to make sure it's all plumbed corectly:

```bash
RP_GITHUBTOKEN=foo cargo run -- --org https://api.github.com/orgs/my-org-name/repos
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/release-party-br --org 'https://api.github.com/orgs/my-org-name/repos'`
thread 'main' panicked at 'Try this for the org value: Try this: my-org-name', src/main.rs:66:30
```

We've done it!  When passed `--org 'https://api.github.com/orgs/my-org-name/repos'` we return with `'Try this for the org value: Try this: my-org-name'`.

## Future work

The first issue is the format: `'Try this for the org value: Try this: my-org-name'` repeats `Try this` and it should be cleaned up.

Second, panicking still looks ugly and the work we just did is still a bit hidden due to that.  There's [a ticket for fixing that behavior](https://github.com/matthewkmayer/release-party-BR/issues/64) later.

## Confidence!

We've added new behavior using composable functions that are easily tested.  Our tests are written and will always be around to ensure if we incorrectly change something, they'll catch it.

This gives us better confidence our code is correct.  Code on!