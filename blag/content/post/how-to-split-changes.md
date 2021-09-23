+++
date = "2021-09-22T00:28:53-07:00"
draft = false
title = "How to split code changes"
+++

## How to split code changes

We've all been there: when writing code, we fall victim to "while I'm here, I might as well..." and end up with a collection of code changes instead of a focused set of changes. One term for this is *shipwrights disease*. It's so easy to fix this typo, update this comment, change this dependency, etc...

There are ways out and methods to logically group changes made, depending on what's most important.

### Branches

With git, branches are cheap. With all the changes saved to a branch, perhaps using the [savepoint pattern](http://think-like-a-git.net/sections/testing-out-merges/the-savepoint-pattern.html), you're free to figure out what changes you want in a branch and which you do not. Since the savepoint branch keeps all your work, there's low risk in losing changes you've already made.

One approach is to go back to the main or master branch, start a new branch and check out the files you need to make a focused set of changes, commit those and push them to the upstream server.

Another method is to use a new branch, not your savepoint branch, and remove the changes that aren't related to the branch's focus.

One more option: you can always copy and paste code from the upstream server, such as GitHub, to undo unwanted changes in your local files. Not elegant but it can be quicker than other options.

### Commits

Sometimes it's easiest to show your work by having multiple commits that build on each other. Commits can also be used to theme changes: place all your comment updating in one commit, dependency updates in another, and so on.

If your changes are already made and committed locally, use the savepoint branch pattern to keep your work. Then you can start a new branch and manually copy in your themed changes commit by commit or use [interactive staging](https://git-scm.com/book/en/v2/Git-Tools-Interactive-Staging) to add just the parts you want into a commit.

### How to pick

Splitting code changes via branches or commits fulfill different needs.

Separating changes into branches helps continuous integration and continuous delivery. Small changes get merged into the main or master branch regularly and deployed quickly. Small changes to deployment means a smaller area to search if problems arise.

Using commits to segregate changes is for making reviewer's lives easier. They can look at related changes in each commit which limits the amount of time and focus required to follow a potentially complex set of changes.
