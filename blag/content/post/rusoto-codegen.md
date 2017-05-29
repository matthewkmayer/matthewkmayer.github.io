+++
date = "2017-06-09T21:28:53-07:00"
draft = false
title = "Rusoto codegen"
+++

Writing a software development kit (SDK) is a challenge.  Especially when the target has a tremendous number of services, endpoints and different styles of interacting, such as REST, querystring and "other."  [Rusoto](https://github.com/rusoto/rusoto) uses the [botocore](https://github.com/boto/botocore) service definitions to create Rust code to interact with Amazon Web Services.  Let's dive into how it's done!

<!--more-->

## Using Simple Queue Service as an example

To make this post clearer, we'll follow the Simple Queue Service (SQS) from botocore service definition to the [rusoto_sqs crate](https://crates.io/crates/rusoto_sqs).

Botocore definitions are JSON representations of AWS services.  They contain information such as what protocol the service uses, such as `rest-xml` or `json`, as well as all the endpoints with their input and output shapes.

At a high level, the botocore definitions are fed into Rusoto codegen and it outputs Rust code.  Here's another way to depict the codegen flow:

```
SQS botocore definition => service_crategen => rusoto_sqs crate
```

## botocore definitions

The botocore definitions are brought in via a [git submodule](https://git-scm.com/docs/git-submodule).  This lets us bring in the botocore project without vendoring files in our git project.  We can lock it to certain versions, update as needed, etc...

You can see the submodule inclusion in [.gitmodules](https://github.com/rusoto/rusoto/blob/master/.gitmodules).  It tells git what submodules we have and where to put it:

```bash
[submodule "service_crategen/codegen/botocore"]
  path = service_crategen/codegen/botocore
  url = https://github.com/boto/botocore.git
```

We're putting the submodule named `service_crategen/codegen/botocore` into the `service_crategen/codegen/botocore` directory and telling it where the source is, which is `https://github.com/boto/botocore.git`.

Without the submodule, codegen doesn't know what to generate, so we [include instructions on fetching the submodule](https://github.com/rusoto/rusoto/blob/master/service_crategen/README.md) in the Rusoto project.

## codegen entry point

The codegen project is a binary project.  Think command line executable, like `rustc` or `cargo`.  After compiling `service_crategen`, we call it with `-c` for which input file to use and `-o` for where to put the generated code.  Here's how we run code generation: `cargo run -- -c ./services.json -o ../rusoto/services`

[services.json](https://github.com/rusoto/rusoto/blob/master/service_crategen/services.json) is a collection of AWS services to generate.  SQS's entry:

```json
"sqs" : {
  "version": "0.25.0",
  "coreVersion": "0.25.0",
  "protocolVersion": "2012-11-05"
}
```

For the key `sqs` we have the version of `rusoto_sqs` to generate, the required version of the `rusoto_core` crate and the protocol version.  The protocol version is used to look up which API definition we need from botocore: definitions get updated and we need to look at the right one.

This JSON file is deserialized into a Rust object inside the `service_crategen` project:

```rust
#[derive(Clone, Deserialize)]
struct ServiceConfig {
  pub version: String,

  #[serde(rename = "coreVersion")]
  pub core_version: String,

  #[serde(rename = "protocolVersion")]
  pub protocol_version: String,

  #[serde(rename = "customDependencies")]
  pub custom_dependencies: Option<BTreeMap<String, cargo::Dependency>>
}
```

We populate that object with this line of code, where we've read the JSON file into `contents` already:

```rust
let parsed: BTreeMap<String, ServiceConfig> = serde_json::from_str(&contents)
    .expect("Unable to parse services configuration file.");
```

From there we have our ServiceConfig object for SQS.  With that, we load the botocore service definition into our object:

```rust
let service = Service::load(name, &service_config.protocol_version);
```

Now we have the SQS service definition loaded into memory as a Rust object!  But what does it contain?

```rust
#[derive(Debug, Deserialize)]
pub struct Service {
  pub documentation: Option<String>,
  pub examples: Option<BTreeMap<String, String>>,
  pub metadata: Metadata,
  pub operations: BTreeMap<String, Operation>,
  #[serde(deserialize_with="ShapesMap::deserialize_shapes_map")]
  pub shapes: BTreeMap<String, Shape>,
  pub version: Option<String>,
}
```

This is all populated by Serde! 🎉

## The tricky part

With everything botocore knows about the service now in memory, we need to generate Rust code to talk to the AWS service.  We'll skip over how we generate the skeleton of the new crate, such as Cargo.toml and the README, and focus on the code that will be published as a crate.

Still in `service_crategen`'s `main.rs` file:

```rust
let mut gen_writer = BufWriter::new(gen_file);
rusoto_codegen::generator::generate_source(&service, &mut gen_writer).unwrap();
```

That's the magic!  We create a buffered writer, for better writing performance, and pass that into `generate_source` along with the service definition we populated above.  We'll visit the implementations of this in another post.

## Rustfmt, or formatting generated code is hard

Generated code, at least ours, has lots of "interesting" whitespace issues.  Since we're checking that code in, we want it to look decent, especially for people who are looking at the generated code to see how it works or to fix a bug.  Here's how we run [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) on the code we generate:

```rust
let _ = rustfmt::run(rustfmt::Input::File(gen_file_path), &rustfmt::config::Config {
  write_mode: rustfmt::config::WriteMode::Overwrite,
  error_on_line_overflow: false,
  ..rustfmt::config::Config::default()
});
```

We use all the defaults from Rustfmt, but tell it to overwrite the source file and not to error if the line is too long.  Long lines are common for code documentation, which we translate directly from the botocore definitions.  As an example, here's a documentation line in the generated SQS code.  Keep scrolling to the right...

```rust
#[doc="<p>The action the client wants to allow for the specified principal. The following values are valid:</p> <ul> <li> <p> <code>*</code> </p> </li> <li> <p> <code>ChangeMessageVisibility</code> </p> </li> <li> <p> <code>DeleteMessage</code> </p> </li> <li> <p> <code>GetQueueAttributes</code> </p> </li> <li> <p> <code>GetQueueUrl</code> </p> </li> <li> <p> <code>ReceiveMessage</code> </p> </li> <li> <p> <code>SendMessage</code> </p> </li> </ul> <p>For more information about these actions, see <a href=\"http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/acp-overview.html#PermissionTypes\">Understanding Permissions</a> in the <i>Amazon SQS Developer Guide</i>.</p> <p>Specifying <code>SendMessage</code>, <code>DeleteMessage</code>, or <code>ChangeMessageVisibility</code> for <code>ActionName.n</code> also grants permissions for the corresponding batch versions of those actions: <code>SendMessageBatch</code>, <code>DeleteMessageBatch</code>, and <code>ChangeMessageVisibilityBatch</code>.</p>"]
```

Here's an example of code before `rustfmt`:

```rust
                 impl Error for CreateQueueError {
                     fn description(&self) -> &str {
                         match *self {
                             CreateQueueError::QueueDeletedRecently(ref cause) => cause,
 CreateQueueError::QueueNameExists(ref cause) => cause,
 CreateQueueError::Validation(ref cause) => cause,
 CreateQueueError::Credentials(ref err) => err.description(),
 CreateQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
 CreateQueueError::Unknown(ref cause) => cause
                         }
                     }
                  }
 ```

 And after:

 ```rust
impl Error for CreateQueueError {
    fn description(&self) -> &str {
        match *self {
            CreateQueueError::QueueDeletedRecently(ref cause) => cause,
            CreateQueueError::QueueNameExists(ref cause) => cause,
            CreateQueueError::Validation(ref cause) => cause,
            CreateQueueError::Credentials(ref err) => err.description(),
            CreateQueueError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateQueueError::Unknown(ref cause) => cause,
        }
    }
}
 ```

Note how the whitespacing is corrected by moving the code to the left and the indentation levels are consistent.  This doesn't matter to the Rust compiler but it makes it a lot easier for humans to read.  A huge win for us!

## What we've seen

We've walked through the codegen progression shown at the beginning of this post:

```
SQS botocore definition => service_crategen => rusoto_sqs crate
```

From botocore definitions through crate generation to the resulting service crate, we've got a high level understanding of how Rusoto code generation works.

Thanks for reading!  The next blog post will cover the code generation more in depth.
