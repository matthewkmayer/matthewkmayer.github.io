+++
date = "2018-12-07T00:28:53-07:00"
draft = false
title = "Rust 2019 Roadmap"
+++

Keeping with my annual updates on the State of Rust, here's my end-of-2018 edition. See [January 2018's view](https://matthewkmayer.github.io/blag/public/post/rust-2018/) for my outlook from a year ago.

<!--more-->

## Did we do it?

`tl;dr`: it's better.

Overall, I am satisfied with the progress Rust and its ecosystem made over the past year. A lot of people have done some wonderful work. Keep it up!

Now let's visit my 2018 wishlist, which was trimmed down to the *two* most important items: 

* better compilation performance
* more blog posts, docs and books

## Success

This past year, there's been lots more documentation, blog posts and tutorials! There are also some new books almost at the printers, so let's chalk them up as a win in 2018. See: [Programming WebAssembly with Rust](https://pragprog.com/book/khrust/programming-webassembly-with-rust) and [Rust in Action](https://www.manning.com/books/rust-in-action).

## To improve

Continuously harping on compile times doesn't help morale. It's known Rust compile times need to improve. Something that can help here is an up to date guide on reducing compile times by avoiding situations that make it hard for the compiler to do its job. I'm more than willing to jump through some hoops in my code to make this happen. There's a few guides that explain what kind of code to avoid and how to replace it with more compiler-friendly versions, and I'd love an updated version.

An XML parser that works similarly to `serde_json` would be amazing. There is a very large amount of code in [Rusoto](https://github.com/rusoto/rusoto) just to handle XML parsing, which could be done more concisely and with more performance than our current method.

More cloud computing SDKs would be greatly welcome. AWS isn't the only game in town. Enabling Rust to be used on other cloud providers will greatly help its adoption.

## Next parts to build

Strike while the AWS iron is hot from reInvent. Rust is supported in Lambda and [Rusoto](https://github.com/rusoto/rusoto) has support for almost 130 different AWS services. Let's build cloud-native solutions that are safe and inexpensive to run.
