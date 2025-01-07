# matthewkmayer.github.io

Welcome visitor, you're probably wanting to find where this blog source code is rendered and readable at. Look at  https://matthewkmayer.github.io/ to see all this nicely rendered in HTML.


## Notes Matthew uses for writing new posts

Hugo site follows https://gohugo.io/overview/quickstart/ .

Live reload editing in the `blag` directory: `~/Downloads/hugo_0.88.1_macOS-ARM64/hugo server -w`. Open http://localhost:1313/blag/public/ .

### Publishing:

Generate HTML in the `blag` directory: `~/Downloads/hugo_0.88.1_macOS-ARM64/hugo --baseURL=https://matthewkmayer.github.io/blag/public`. Commit and push, site is updated on merge to `master` branch.

### Working with old versions of hugo

Download Hugo 0.88.1 from their GitHub releases.

Use older version of tale theme:

`git submodule update --init`

Then in the theme folder:

`git checkout tags/v1.1.3`
