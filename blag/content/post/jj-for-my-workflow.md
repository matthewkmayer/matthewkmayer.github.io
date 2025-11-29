+++
date = "2025-11-28T00:28:53-07:00"
draft = false
title = "JJ for my workflow"
+++

## Jujutsu (JJ) for my workflow

This is how I use JJ.

There's a lot one can do with source control tools; this is what I do. Terms are a bit loose, especially commits versus changes and branches versus bookmarks.

<!--more-->

### Cloning and committing

JJ has a "colocated" repo mode, where git and JJ coexist. This can be nice for being able to drop into `git` for commands such as tagging and letting various tools operate on git such as seeing which committer last changed the line you're looking at.

```bash
jj git clone --colocate git@github.com:matthewkmayer/matthewkmayer.github.io.git
```

This will clone the repo, set it up for JJ and git and track the default branch:

```bash
$ jj git clone --colocate git@github.com:matthewkmayer/matthewkmayer.github.io.git

Fetching into new repo in "/Users/matthewmayer/go/src/github.com/matthewkmayer/matthewkmayer.github.io"
remote: Enumerating objects: 2949, done.
remote: Total 2949 (delta 81), reused 173 (delta 66), pack-reused 2732 (from 1)
bookmark: master@origin                       [new] tracked
```

Move into that directory and you can see the log of changes:

```bash
$ jj

@  qwyrqzwp matthew@themayers.org 2025-11-29 10:22:50 990be9b8
│  (empty) (no description set)
◆  zyoruoll matthew@themayers.org 2025-01-06 19:36:36 master git_head() ec08563f
│  (empty) Merge pull request #120 from matthewkmayer/hugo-reboot
```

I've created a new file for this post and can see there are changes:

```bash
$ jj

@  qwyrqzwp matthew@themayers.org 2025-11-29 10:37:39 3b2cec57
│  (no description set)
◆  zyoruoll matthew@themayers.org 2025-01-06 19:36:36 master git_head() ec08563f
│  (empty) Merge pull request #120 from matthewkmayer/hugo-reboot
```

And can see what files changes:

```bash
$ jj st

Working copy changes:
A blag/content/post/jj-for-my-workflow.md
```

And the diff:

```bash
$ jj diff

Added regular file blag/content/post/jj-for-my-workflow.md:
        1: +++
        2: date = "2025-11-28T00:28:53-07:00"
        3: draft = false
        4: title = "JJ for my workflow"
        5: +++
(snipped)
```

When I think I've got enough to record what I've changed I make a commit:

```bash
$ jj commit -m "Draft first pass."

Working copy  (@) now at: vwqkntqu a17ccbf5 (empty) (no description set)
Parent commit (@-)      : qwyrqzwp 831deb89 Draft first pass.
```

JJ can show we've got that commit recorded and I've written more in my document, hence the top line not saying "(empty)" next to "(no description set)."

```bash
$ jj

@  vwqkntqu matthew@themayers.org 2025-11-29 10:40:19 280f02bb
│  (no description set)
○  qwyrqzwp matthew@themayers.org 2025-11-29 10:40:08 git_head() 831deb89
│  Draft first pass.
◆  zyoruoll matthew@themayers.org 2025-01-06 19:36:36 master ec08563f
│  (empty) Merge pull request #120 from matthewkmayer/hugo-reboot
```

Now I'll push my changes to GitHub, creating a new anonymous branch:

```bash
$ jj git push -c @-

Creating bookmark push-qwyrqzwpvosw for revision qwyrqzwpvosw
Changes to push to origin:
  Add bookmark push-qwyrqzwpvosw to 831deb89628c
remote: Resolving deltas: 100% (3/3), completed with 3 local objects.
remote:
remote: Create a pull request for 'push-qwyrqzwpvosw' on GitHub by visiting:
remote:      https://github.com/matthewkmayer/matthewkmayer.github.io/pull/new/push-qwyrqzwpvosw
```

`push -c` creates a new branch. `@-` says to push the parent of the working copy `@`.

Now we have a bookmark for the push:

```bash
$ jj

@  vwqkntqu matthew@themayers.org 2025-11-29 10:42:23 37a83542
│  (no description set)
○  qwyrqzwp matthew@themayers.org 2025-11-29 10:40:08 push-qwyrqzwpvosw git_head() 831deb89
│  Draft first pass.
```

Let's commit what we've written so far and add that to the branch:

`jj commit -m "Continuing."`

`jj tug`:

```bash
$ jj

@  ptzlqusl matthew@themayers.org 2025-11-29 10:44:31 650217fd
│  (no description set)
○  vwqkntqu matthew@themayers.org 2025-11-29 10:44:09 push-qwyrqzwpvosw* git_head() 5d222383
│  Continuing.
○  qwyrqzwp matthew@themayers.org 2025-11-29 10:40:08 push-qwyrqzwpvosw@origin 831deb89
│  Draft first pass.
```

`tug` is an alias to tug the closest bookmark to the parent of the working commit. In git, you'd be on a branch and the branch will move to the latest commit on it, but it's not automatic in JJ. I used to find it annoying before moving to other workflows JJ enables that are more difficult to do in git.

Tug defined in `~/.config/jj/config.toml`:

```toml
[aliases]
tug = ["bookmark", "move", "--from", "heads(::@- & bookmarks())", "--to", "@-"]
```

A `jj git push` will push the new commit on our branch/bookmark to GitHub.

If the Pull Request on GitHub looks good, it can be merged and a `jj git fetch` will get the update. To start a new change, `jj new master` will create a new working copy off the updated master branch.

That's the most basic workflow and it'll work for a long time.

### Two ways to squash

I regularly used `git rebase -i master` to squash commits. The same can be done with JJ.

We currently have:

```bash
$ jj

@  ptzlqusl matthew@themayers.org 2025-11-29 10:50:32 4886b854
│  (no description set)
○  vwqkntqu matthew@themayers.org 2025-11-29 10:44:09 push-qwyrqzwpvosw git_head() 5d222383
│  Continuing.
○  qwyrqzwp matthew@themayers.org 2025-11-29 10:40:08 831deb89
│  Draft first pass.
◆  zyoruoll matthew@themayers.org 2025-01-06 19:36:36 master ec08563f
│  (empty) Merge pull request #120 from matthewkmayer/hugo-reboot
```

Squash the two commits our branch has into one:

```bash
$ jj squash --from q..v --to q
```

This is using the revision ID of qwyrqzwp and vwqkntqu. Instead of using the whole change ID we use the short code for them, which is colored in the terminal.

Now it's squashed and knows our branch is different in our repo than the remote:

```bash
$ jj

@  ptzlqusl matthew@themayers.org 2025-11-29 10:51:08 4d9788b8
│  (no description set)
○  qwyrqzwp matthew@themayers.org 2025-11-29 10:51:06 push-qwyrqzwpvosw* git_head() 3fe548d0
│  Draft first pass.
◆  zyoruoll matthew@themayers.org 2025-01-06 19:36:36 master ec08563f
│  (empty) Merge pull request #120 from matthewkmayer/hugo-reboot
```

A `jj git push` will update the branch/bookmark on GitHub. This will be a force push which is fine for your branches but if others are using it, be cautious rewriting history, the same as git's `rebase`.

The other way to squash changes is to take what's in your working commit and squash it into the parent. `jj squash` does that: what's in your working copy, what you see in the filesystem, is added to the previous commit.

### Splitting changes

I generally don't split changes but when it's time to, `jj commit -i` lets you interactively choose what to put into a commit and leaves the rest in your working copy. I've most recently used this to take the actual bugfix from my working copy and leave all the debug parts littered in the code out of the fix commit.

From that state I can either manually remove the no-longer-needed bits or use `jj restore` to throw away the changes in the working copy.

### Rebasing branches

If you rebase in git this will be similar but without any "oh no fix it now" issues that merge conflicts and git have. As a bonus you can also rebase branches you aren't on to something else.

For example I have an old branch whose first commit off master is `po`. I can tell JJ to rebase that branch onto the latest I have for the master bookmark without switching to it:

```bash
jj rebase -s po -d master
```

### undo

It's possible to recover from almost any undesired state in git, but takes action before the potentially risky command, such as using a [savepoint branch](https://think-like-a-git.net/sections/testing-out-merges/the-savepoint-pattern.html) or digging through the reflog.

In JJ, it's easy to try those actions because they can be undone with `jj undo`. How difficult is bringing an old branch up to date? Try it! Doesn't work? `jj undo`. Want to squash it before rebase? Sure, and if a step fails, `jj undo` will undo one operation at a time, so run it until you're back at the initial state.

This perk is incredible and unlike git, it doesn't rely on the user knowing what could go wrong and to take action beforehand or do an out-of-the-ordinary process like reflog spelunking.

### Viewing repo history

I've been using [`git lg1`](https://stackoverflow.com/questions/1057564/pretty-git-branch-graphs) for a long time. JJ's default `log` command doesn't show the information I like, such as bookmarks I'm not tracking. Great for keeping track of your work but if others are working in the same repo, their changes aren't as visible.

That's fixed with an alias in my config file at `~/.config/jj/config.toml`:

```toml
[aliases]
ll = ["log", "-r", "all()", "--limit", "20"]
lll = ["log", "-r", "all()"]
```

`jj ll` will show one page of changes in my terminal, which lets you quickly see the most recent changes. Since it fits on the terminal the pager isn't called so I don't have to exit from it to get back to the shell.

When it's time to see much longer branches I use `jj lll` to show as much as I need.

### Give it a try

I've been using JJ as my daily driver for the better part of a year now. It's been 100% worth it. I've found it far more ergonomic to use than git.

It's easy to start small. Pick one repository and test drive it for a few days or weeks. I bet you'll find it's far more rewarding to switch than you expect.
