+++
date = "2026-01-04T00:28:53-07:00"
draft = false
title = "It's 2026, you don't need to deal with git"
+++

It's 2026, you don't need to deal with git.

<!--more-->

That's a strong one! In my career I've enjoyed seeing the produtivity gains by using the right tools. At my first professional software job, my group didn't use version control and bug tracking was my pen and paper notepad. I quickly introduced Subversion (SVN) and Mantis Bug Tracker, which allowed us to ensure bugs got tracked and fixed and eliminated the "this used to work before but doesn't now, what changed?"

My next job used SVN for source control and we moved to git and GitHub. This was another step up: lightweight branches, no locking of files, far easier merges. We used GitFlow because it was there and multiple versions of software needed support. Think versions 4.3.1, 5.0.5, 6.0.0, etc...

When we had more software running in the cloud instead of installed on customer machines, that cloud software used a more GitHub Flow style of releasing for better velocity and lower time to fix bugs.

In 2026, there's a new step up for source control: [Jujutsu](https://github.com/jj-vcs/jj).

## What's Jujutsu?

Jujutsu, also known as `jj`, is a new version control system. It splits the user interface and version control parts from the storage systems, or backends. There's an example "SimpleBackend" that shows a proof of concept of how jj works completely self-contained, a closed source backend used at Google for their Piper/Clients in the Cloud backend and important for us, the git backend.

Having a git backend means a jj repository can use git and git tools, including forges like GitHub, IDE integrations like GitLens for Visual Studio Code and command line tools.

**You can use jj and keep your existing git workflows and tools.**

## What's special about jj

The biggest mindset change for git users is the lack of a staging area: jj automatically tracks and commits changes when you run a jj command. For example, if you edit a file, save it and run `jj status` the change is committed. I often hear git users state they want to tell the tool when to commit, and you can do that! Having changes automatically committed doesn't prevent you from crafting a chain of commits like you'd be able to make with `git add` or `git add -p`.

This idea of a working copy as a commit enables ["no modes" behavior](https://en.wikipedia.org/wiki/Mode_(user_interface)). `jj` commands do the same thing all the time because there are no different modes. Compare that to git during a rebase with merge conflicts, where you're in a different mode and only certain commands work.

Conceptually nearby is how `git commit` requires use of the staging area. You have to manage the state of the staging area to make commits. Without a staging area, jj commands operate on your working copy, but can also operate on other commits.

The working copy as a commit also enables **undo**! I've lost count the number of times a coworker tries to perform an action in git like a rebase or commit splitting and ended up in a strange spot from which they don't know how to recover. The fact [Oh Shit, Git!?!](https://ohshitgit.com/) exists demonstrates this is a common issue.

With jj, you can try something and `jj undo` will undo the most recent change. Keep running `jj undo` until you're back to where you started. Far easier than other approaches, like the [Savepoint branch pattern](https://think-like-a-git.net/sections/testing-out-merges.html) and other techniques that require a human to remember to do something before a tricky manipulation of the commit tree or going through git's reflog.

Another mindset change is anonymous branches. With many years of starting new work with `git checkout main; git pull; git checkout -b thing-im-doing` it's refreshing to not having to think about a branch name right off the bat. In jj, bookmarks are similar to git branches and can be created closer to when they're needed, such as pushing changes to GitHub. `jj git fetch; jj new main` is all you need to start work on the tip of the main branch.

I've totally embraced anonymous branches. Any fear I had about how strange and uncomfortable not having a branch name went away quickly after using jj full time.

Another great point for jj is the built in visibility. I used [`git lg1`](https://stackoverflow.com/questions/1057564/pretty-git-branch-graphs) for a long, long time and jj has similar but more powerful commit graphs out of the box. The default `jj log` command shows what's important and you can recreate `git lg1` with arguments to it, usually put in the jj config as an alias. Example: `lll = ["log", "-r", "all()"]` makes `jj lll` show what `git lg1` shows.

Merge conflicts aren't a "stop the world" event like git. They can be addressed right away or saved until later due to how jj [records them](https://docs.jj-vcs.dev/latest/conflicts/).

## Splitting commits

`jj commit -i` lets you interactively choose which changes go into a commit and which ones get left in the working copy. You can chain these to make a commit history that didn't exist but makes it easier for others to review your code.

Commits can be retroactively split with `jj split`, so that mega commit can be sliced into seperate commits.

## What's not so hot

jj doesn't play well with some things, such as git submodules. jj can't create annotated git tags. Bookmarks don't "float" like git branches do, but the [`jj tug` alias](https://shaddy.dev/notes/jj-tug/) lets you move bookmarks with ease.

## Give Jujutsu a try in 2026

jj is [simpler and more powerful than git](https://steveklabnik.github.io/jujutsu-tutorial/introduction/what-is-jj-and-why-should-i-care.html). This big claim has proven true for me.

I've enjoyed less friction, less mental effort, economy of mental movement and the ability to fall back to git if needed as a mental safety net for the switch.

Check out [Steve Klabnik's Jujutsu Tutorial](https://steveklabnik.github.io/jujutsu-tutorial/introduction/what-is-jj-and-why-should-i-care.html) and my [jj for my workflow](https://matthewkmayer.github.io/blag/public/post/jj-for-my-workflow/).

Give it a try in one git repo. jj's git colocation means you can always use git for operations if you're stuck, but I found I quickly stopped using git. jj is that good.
