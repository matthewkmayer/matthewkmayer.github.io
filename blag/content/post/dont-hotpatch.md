+++
date = "2019-05-07T00:28:53-07:00"
draft = false
title = "Don't hot patch"
+++

A hot take on hot patching/fixing.

<!--more-->

## Definitions

A hot patch, or hot fix, is a method of fixing a critical bug found in production by following a branching strategy that's outside the norm of software development. Specifically called out in [Gitflow](https://nvie.com/posts/a-successful-git-branching-model/), this process is more complex and error prone than treating a critical bug as a regular bug or feature.

When following Gitflow, a hot patch can get a fix to production faster by making the change off the master branch, which is then also applied to the release branch. When releases are slow and expensive, this can be successful. However, there are simpler approaches available.

## Hot patching

The usual approach for writing a new feature or fixing a bug in Gitflow is to make a new branch off the `develop` branch, do work then merge it back to `develop`. Straightforward enough. But when a new release requires merging the `develop` branch into the `release` branch, then cutting a new release by merging `release` into `master`, it can bring along other changes into the new release, which may not be desired.

Since a Gitflow hot patch is branched off `master` the change can be smaller and only bring along the fix.

A major downside is requiring the person writing the patch to merge their hotfix branch into both `master` and `develop`. If the patch isn't merged into `develop` as well, it's possible to introduce a regression on the next release.

When a critical bug is found in production, there's a higher level of stress present than everyday coding. Requiring a stressed human to remember an infrequently used workflow is very likely to result in a step of the workflow to be missed. The bug bites twice when the next release has a regression.

## Don't have that problem

A patch for a bug should follow the same process as feature work, regardless of bug severity. This reduces the cognitive load of the humans involved, who are already stressed enough about a critical bug.

## Trunk based development

[Trunk based development](https://trunkbaseddevelopment.com/) (TBD) is a well thought out and battle tested method to use source control. When a bug is found in TBD, [the reproduction test and fix is applied to the trunk/master branch](https://trunkbaseddevelopment.com/branch-for-release/) and cherry picked to the release branch, if used.

The upside to this approach is a regression is not possible: the fix is already in the main branch, so the next release will receive the fix. It also achieves the goal of only bringing the critical bug fix into a quick bug fix release.

## GitLab flow

[GitLab flow](https://docs.gitlab.com/ee/workflow/gitlab_flow.html) is another branching strategy to use. One can use GitLab flow to closely mirror how trunk based development works, or it can be used closer to Gitflow.

When a critical bug is found in GitLab flow, the fix is made in a short lived branch off `master`. This is merged back into master, ensuring future releases will not have a regression. Note this is the same way all bug fixes or feature work is done. If needed, the commit with the critical bug fix can be applied to an existing release branch to get the fix out sooner.

## Out of the ordinary or business as usual?

Both trunk based development and GitLab flow have the option of quickly and easily cutting a new release to get a critical bug fix deployed. With a [strong CI/CD pipeline](https://matthewkmayer.github.io/blag/public/post/ci-cd-pipeline/), a bug fix can be released with other features and bug fixes with little fuss. This is much harder to do with Gitflow, where releases require careful thinking and source control commands.

## Don't spend time wrangling source control

Relying on stressed humans to perfectly execute a rarely used workflow is a recipe for sadness and regressions. Letting developers use the same process they always use is one less moving piece for them to keep track of while fixing a bug.

Spend time fixing bugs instead of cherry picking commits or manually applying code changes.
