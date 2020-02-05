+++
date = "2020-02-04T00:28:53-07:00"
draft = false
title = "Red/green bugfix"
+++

Red/green bugfixes are the best technique for demonstrating bug reproduction and solution for asynchronous code reviews.

## A bug

Say a new bug has been discovered in a backend service. If it's found in production, the first instinct is to solve it as quickly as possible. While laudable, this kneejerk reaction can actually make the fix slower and harder for others to understand.

## Usual fixing procedure

1. Reproduce the bug
2. Using logs and/or reading the code, find where the issue is
3. Fix the bug, make a pull request, let coworkers review it

This procedure is straightforward and can get a bug fix out quickly. There are downsides to this process:

* How is the fix confirmed?
* How are regressions prevented?
* How is the bug communicated to the rest of the team? What if there are similar bugs lurking in the codebase?

## Red/green bugfix

A red/green bugfix works differently:

1. Reproduce the bug
2. Using logs and/or reading the code, find where the issue is
3. Cover the broken behavior with a test: assert on expected behavior
4. Push the code and let the build pipeline demonstrate a red/failing state due to the new test failing
5. Write the bug fix, see the test pass, push code and make a pull request: the build will go green/pass

By making separate commits, one for the new test case and one for the fix, the problem and solution is communicated via source control and the build pipeline. Others reviewing the code can clearly see what the issue was, see the test covers the broken behavior, then see the fix actually solves the problem.

This process also prevents regressions: a test is now in place to prevent the bug from surfacing again.

By taking a few extra steps to fix a bug, the change is easier to follow by others and regressions can be prevented.
