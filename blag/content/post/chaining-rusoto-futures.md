+++
date = "2019-01-02T00:28:53-07:00"
draft = false
title = "Chaining Rusoto Futures"
+++

Rusoto supports asynchronously running commands to AWS. While immediately making a blocking call to AWS is a common use case, one can create actions to run async and let the `tokio` runtime handle it. This opens up the ability to run multiple AWS calls concurrently. 

If a project sets up infrastructure on AWS, the actions that don't depend on others to finish can run concurrently. For example: creating an S3 bucket and an SQS queue. Instead of creating a bucket and waiting for the request to finish, then moving on to creating the queue, one can put the Rusoto `Futures` together and concurrently run them.

<!--more-->

## Samples

There are two companion projects for this blog post: [rusoto-chained-futures]() and [rusoto-chained-futures-2](). These projects use [local DynamoDB]() so they can be run without an AWS account.

Basic knowledge of `futures` and `tokio` are suggested. The [tokio site](https://tokio.rs/docs/futures/overview/) is a good resource to gain an understanding. Experience with Rusoto is also recommended since its API is very AWS flavored and isn't necessarily idiomatic Rust. Check [rusoto.org]() for samples and links to API documentation.

## First sample: throwing away errors

Our sample project uses these dependencies:

```toml
[dependencies]
rusoto_core = "0.36"
rusoto_dynamodb = "0.36"
futures = "0.1"
tokio-core = "0.1"
```

We'll use the latest releases of `futures` 0.1 and `tokio-core` 0.1.

Here's the main part of the project, with comments removed:

```rust
let item = make_item();
let client = get_dynamodb_local_client();

let mut core = Core::new().unwrap();

let create_table_future = make_create_table_future(&client);
let upsert_item_future = make_upsert_item_future(&client, &item);
let item_from_dynamo_future = make_get_item_future(&client, &item);

let chained_futures = create_table_future
    .then(|_| upsert_item_future)
    .then(|_| item_from_dynamo_future);

let item_from_dynamo = match core.run(chained_futures) {
    Ok(item) => item,
    Err(e) => panic!("Error completing futures: {}", e),
};

println!("item_from_dynamo is {:?}", item_from_dynamo);
```

To improve clarity, functions have been extracted.

```rust
let item = make_item();
let client = get_dynamodb_local_client();
```

The first line makes a DynamoDB item for us to use for insertion and retrieval. The second line creates a DynamoDB client configured to point at local DynamoDB instead of Amazon's DynamoDB.

```rust
let mut core = Core::new().unwrap();
```

`Core` is a `tokio_core::reactor::Core` object. It's what we'll hand the futures to in order to run asynchronously.

The next few lines create the Rusoto futures and saves them in variables. Looking at the create_table_future, we see the function has this signature:

```rust
fn make_create_table_future(client: &DynamoDbClient) -> impl Future<Item = CreateTableOutput>
```

Taking a reference to the DynamoDbClient, it returns a Future with an expected `Item` of `CreateTableOutput`. Nothing else can happen in this future: it can't return an error.

```rust
let chained_futures = create_table_future
  .then(|_| upsert_item_future)
  .then(|_| item_from_dynamo_future);
```

We use [.then]() to pass the successful result from `create_table_future` to `upsert_item_future`. Then we do the same with `upsert_item_future`'s success and pass it to `item_from_dynamo_future`. Since we use the underscore, `_`, for the success returned by the future, it's ignored. This lets us ignore error handling for problems creating the table or upserting the item.

```rust
let item_from_dynamo = match core.run(chained_futures) {
  Ok(item) => item,
  Err(e) => panic!("Error completing futures: {}", e),
};
```

With our chained, or combined, future as a new variable, we can pass it to `core.run()`. The type `core.run()` returns is the type `item_from_dynamo_future` is. The signature for the function that created the variable:

```rust
fn make_get_item_future(
  client: &DynamoDbClient,
  item: &HashMap<String, AttributeValue>,
) -> impl Future<Item = GetItemOutput, Error = GetItemError>
```

This returns a `Result` after running. It's similar to calling `client.get_item(get_item_request).sync()` for synchronously running the command, but we'll only get it after the previously chained futures complete.

```rust
let item_from_dynamo = match core.run(chained_futures) {
  Ok(item) => item,
  Err(e) => panic!("Error completing futures: {}", e),
};

println!("item_from_dynamo is {:?}", item_from_dynamo);
```

Printing the result shows this:

```
item_from_dynamo is GetItemOutput { consumed_capacity: None, item: Some({"foo_name": AttributeValue { b: None, bool: None, bs: None, l: None, m: None, n: None, ns: None, null: None, s: Some("baz"), ss: None }}) }
```

We've successfully run the commands asynchronously! The project can be run multiple times and will succeed despite the create table call failing due to the table already existing: we throw away the error from that call and move forward.

## Second sample: mapping an error

## Give it a try

If you're running into issues with Rusoto futures, please make an issue on the [Rusoto repo]() on GitHub.
