+++
date = "2019-01-02T00:28:53-07:00"
draft = false
title = "Chaining Rusoto Futures"
+++

Rusoto supports asynchronously running commands to AWS. While immediately making a blocking call to AWS is a common use case, one can create actions to run async and let the `tokio` runtime handle it. This opens up the ability to run multiple AWS calls concurrently. 

If a project sets up infrastructure on AWS, the actions that don't depend on others to finish can run concurrently. For example: creating an S3 bucket and an SQS queue. Instead of creating a bucket and waiting for the request to finish, then moving on to creating the queue, one can put the Rusoto `Futures` together and concurrently run them.

In this post we'll explore to examples of using futures and the Rusoto DynamoDB client.

<!--more-->

## Samples

There are two companion projects for this blog post: [rusoto-chained-futures](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-chained-futures) and [rusoto-chained-futures-2](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-chained-futures-2). These projects use [local DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.DownloadingAndRunning.html) so they can be run without an AWS account.

Basic knowledge of `futures` and `tokio` are suggested. The [tokio site](https://tokio.rs/docs/futures/overview/) is a good resource to gain an understanding. Experience with Rusoto is also recommended since its API is very AWS flavored and isn't necessarily idiomatic Rust. Check [rusoto.org](https://rusoto.org/) for samples and links to API documentation.

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
fn make_create_table_future(client: &DynamoDbClient)
  -> impl Future<Item = CreateTableOutput>
```

Taking a reference to the DynamoDbClient, it returns a Future with an expected `Item` of `CreateTableOutput`. Nothing else can happen in this future: it can't return an error.

```rust
let chained_futures = create_table_future
  .then(|_| upsert_item_future)
  .then(|_| item_from_dynamo_future);
```

We use [.then()](https://docs.rs/futures/0.1/futures/future/trait.Future.html#method.then) to pass the successful result from `create_table_future` to `upsert_item_future`. Then we do the same with `upsert_item_future`'s success and pass it to `item_from_dynamo_future`. Since we use the underscore, `_`, for the success returned by the future, it's ignored. This lets us ignore error handling for problems creating the table or upserting the item.

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
item_from_dynamo is GetItemOutput 
{ consumed_capacity: None, item: Some({"foo_name": AttributeValue 
{ b: None,..., s: Some("baz"), ss: None } }) }
```

We've successfully run the commands asynchronously! The project can be run multiple times and will succeed despite the create table call failing due to the table already existing: we throw away the error from that call and move forward.

## Second sample: mapping an error

If we want to do something with an error, we need to return it from the future. The [second sample project](https://github.com/matthewkmayer/matthewkmayer.github.io/tree/master/samples/rusoto-chained-futures-2) shows examples of that.

```rust
let make_table_future_the_second = create_table_future_with_error_handling(&client);
let get_item_future = make_get_item_future_with_error_handling(&client, &item);
```

We need to use a different function signature to return a future that can return an error. `create_table_future_with_error_handling` looks like this:

```rust
fn create_table_future_with_error_handling(
    client: &DynamoDbClient,
) -> impl Future<Item = CreateTableOutput, Error = CreateTableError>
```

The difference here is we specify both `Future<Item>` and `Future<Error>`. Let's chain the futures and see what we can do with the error type available. We'll run the chained/combined future the same way as before:

```rust
let chained_with_failure_handling = match core.run(chained) {
  Ok(result) => format!("Got item: {:?}", result),
  Err(e) => format!("Didn't get item: {}", e),
};

println!(
  "chained_with_failure_handling is {}",
  chained_with_failure_handling
);
```

```rust
let chained = make_table_future_the_second
  .map_err(|e| {
    println!("We could do something with the table creation error here.");
    // We need to make any error return look like the innermost error returned.
    // Let's create a GetItemError:
    GetItemError::InternalServerError(format!(
      "Actually from us! Real error from attempting to making the table: {}",
      e
    ))
  })
  .and_then(|r| {
    // If we're here, we successfully made the new table.
    // r will be the CreateTableOutput:
    println!("r is {:?}\n\n", r);

    // Finally, call the get_item_future and return the Result<GetItemOutput, GetItemError>
    // and we can match on that later.
    get_item_future
  });
```

`make_table_future_the_second` uses `map_err()` to map the error result to the closure, which is just code to run if an error happens. In the code block, we print a notice something went wrong then make a new `GetItemError`. We have to create a `GetItemError` because that's what the combined future, `chained`, requires as its type. It's a `Future<Item = GetItemOutput, Error = GetItemError>` because the last future, `get_item_future`, has that type.

Since `.and_then()` will only run on successful, non-error completion of the future it's called on, we won't call `get_item_future` if we couldn't create the table. For example, if we run the project and local DynamoDB isn't running, we see this output:

```
We could do something with the table creation error here.

chained_with_failure_handling is Didn't get item: 
Actually from us! Real error from attempting to making the table: 
an error occurred trying to connect: Connection refused (os error 61)
```

That's both our print statement and the error returned by running the `chained` future.

If the table was successfully created, we see this output:

```
r is CreateTableOutput { table_description:...table_status: Some("ACTIVE") }) }

chained_with_failure_handling is Got item: 
GetItemOutput { consumed_capacity: None, item: None }
```

The future ran successfully! We printed the result of the table creation call in the closure we provided `.and_then()`:

```rust
.and_then(|r| {
  // If we're here, we successfully made the new table.
  // r will be the CreateTableOutput:
  println!("r is {:?}\n\n", r);

  // Finally, call the get_item_future and return the Result<GetItemOutput, GetItemError>
  // and we can match on that later.
  get_item_future
})
```

The `|r|` syntax is how we passed it into our closure to run on successful completion of the first future.

## Give it a try

While these examples don't execute much differently than using Rusoto's `.sync()` command, they show what can be done with creating futures and chaining them together. Maybe futures can be created on a different thread and passed to a `tokio::Core` instance so all async commands are run through a single tokio core, leaving the main thread open to handling other events coming in. An event of "An S3 bucket has been requested" could be turned into a create bucket future and sent to the tokio core to be executed. Non-blocking IO!

If you're running into issues with Rusoto futures, please make an issue on the [Rusoto repo](https://github.com/rusoto/rusoto) on GitHub.
