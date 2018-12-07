## Rusoto S3 Select example

[S3 Select](https://aws.amazon.com/blogs/aws/s3-glacier-select/) lets S3 do server side filtering of objects so you can only download what you need instead of entire objects.

This repo is meant to be an example of how to use this feature with Rusoto.

## Running the sample

Right now this example is a work in progress so you'll need to change the bucket name or the sample won't work, as I currently have the bucket. :)

After changing that part of `main.rs`, run:

`$ cargo run`

To see logging, including what `hyper` sends, use this command:

`$ RUST_LOG=rusoto,hyper=debug cargo run`
