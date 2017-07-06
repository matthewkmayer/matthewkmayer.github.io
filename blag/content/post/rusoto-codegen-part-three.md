+++
date = "2017-07-11T21:28:53-07:00"
draft = false
title = "Rusoto codegen, part three"
+++

This is part three of Rusoto code generation.  The first two parts went over how code inside a crate is generated.
In in post, we'll take a look at how we make the crate for an AWS service.

<!--more-->

## Parts one and two recap

In the previous two posts, we followed code generation from the Simple Queue Service (SQS) botocore service definition to Rust code.
We glossed over where the generated code went in order to concentrate on the generation itself.  As a refresher, this is how the code flows:

```
SQS botocore definition => service_crategen => rusoto_sqs crate
```

Let's get into how we make a crate from scratch!

## Final output crate

To help us understand what we're making, here's how the `rusoto_sqs` crate is laid out at the end of code generation.

Trimmed output from `tree` in the `rusoto/services/sqs` directory:

```bash
.
├── Cargo.toml
├── README.md
└── src
    ├── custom
    │   ├── custom_tests.rs
    │   └── mod.rs
    ├── generated.rs
    └── lib.rs
```

This is your run-of-the-mill library crate setup with minor tweaks with the `custom` directory and a `generated.rs` file.  Diving into the [Cargo.toml file](https://github.com/rusoto/rusoto/blob/master/rusoto/services/sqs/Cargo.toml):

```bash
[package]
authors = ["Anthony DiMarco <ocramida@gmail.com>", "Jimmy Cuadra <jimmy@jimmycuadra.com>", "Matthew Mayer <matthewkmayer@gmail.com>", "Nikita Pekin <contact@nikitapek.in>"]
description = "AWS SDK for Rust - Amazon Simple Queue Service @ 2012-11-05"
documentation = "https://rusoto.github.io/rusoto/rusoto_core/index.html"
keywords = ["AWS", "Amazon", "sqs"]
license = "MIT"
name = "rusoto_sqs"
readme = "README.md"
repository = "https://github.com/rusoto/rusoto"
version = "0.27.0"
homepage = "https://www.rusoto.org/"

[build-dependencies]

[dependencies]
hyper = "0.10.0"
xml-rs = "0.6"

[dependencies.rusoto_core]
version = "0.27.0"
path = "../../core"
[dev-dependencies.rusoto_mock]
version = "0.25.0"
path = "../../../mock"
```

This file contains contains the usual crate information, such as authors, description, etc... as well as dependencies needed to build the SQS service client.

The first item that's customized for the SQS crate is the description: `"AWS SDK for Rust - Amazon Simple Queue Service @ 2012-11-05"`.
This contains both the service name and the botocore API definition version.  There's also the crate name, `rusoto_sqs`.

Breaking out the dependencies section:

```bash
[dependencies.rusoto_core]
version = "0.27.0"
path = "../../core"
[dev-dependencies.rusoto_mock]
version = "0.25.0"
path = "../../../mock"
```

All services require `rusoto_core`.  That's where code used by all services live.  For example, the AWS request signature calculation functions.  `rusoto_mock` is brought in for testing purposes when the SQS crate is being developed.

```bash
[dependencies]
hyper = "0.10.0"
xml-rs = "0.6"
```

Other services may bring in more dependencies, but SQS only needs `hyper` and `xml-rs`.  And that wraps up Cargo.toml!

The [README](https://github.com/rusoto/rusoto/blob/master/rusoto/services/sqs/README.md) is a short overview to help when navigating the project on Github.  It has links to [https://rusoto.org](https://rusoto.org), [API documentation](https://rusoto.github.io/rusoto/rusoto_core/index.html) and slightly customized information for the service such as the service name and how to bring it into a project's Cargo.toml.

Before we look at the other files, here's a refresher on the directory structure:

```bash
.
├── Cargo.toml
├── README.md
└── src
    ├── custom
    │   ├── custom_tests.rs
    │   └── mod.rs
    ├── generated.rs
    └── lib.rs
```

The files we'll concentrate on now are the ones under the `src` directory.  Both `lib.rs` and `generated.rs` are created by our codegen.  Let's look at the shorter one first, `lib.rs`:

```rust
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

//! <p>Welcome to the <i>Amazon Simple Queue Service API Reference</i>.</p> <p>Amazon Simple Queue Service (Amazon SQS) is a reliable, highly-scalable hosted queue for storing messages as they travel between applications or microservices. Amazon SQS moves data between distributed application components and helps you decouple these components.</p> <note> <p> <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/standard-queues.html">Standard queues</a> are available in all regions. <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues.html">FIFO queues</a> are available in US West (Oregon) and US East (Ohio).</p> </note> <p>You can use <a href="http://aws.amazon.com/tools/#sdk">AWS SDKs</a> to access Amazon SQS using your favorite programming language. The SDKs perform tasks such as the following automatically:</p> <ul> <li> <p>Cryptographically sign your service requests</p> </li> <li> <p>Retry requests</p> </li> <li> <p>Handle error responses</p> </li> </ul> <p> <b>Additional Information</b> </p> <ul> <li> <p> <a href="http://aws.amazon.com/sqs/">Amazon SQS Product Page</a> </p> </li> <li> <p> <i>Amazon SQS Developer Guide</i> </p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/MakingRequestsArticle.html">Making API Requests</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html">Using Amazon SQS Message Attributes</a> </p> </li> <li> <p> <a href="http://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html">Using Amazon SQS Dead Letter Queues</a> </p> </li> </ul> </li> <li> <p> <i>Amazon Web Services General Reference</i> </p> <ul> <li> <p> <a href="http://docs.aws.amazon.com/general/latest/gr/rande.html#sqs_region">Regions and Endpoints</a> </p> </li> </ul> </li> </ul>
//!
//! If you're using the service, you're probably looking for [SqsClient](struct.SqsClient.html) and [Sqs](trait.Sqs.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
```

At the top we have a fancy warning to help guide putting code in the correct spot.  This file is generated from scratch by codegen whenever it's run.  We see the dependencies required by the service and bringing in `generated` and `custom` modules, exporting them as public.

`generated.rs` is where the service interaction code lives.  We won't paste it here since it's currently over 4,000 lines long for this relatively simple AWS service.  The file can be [seen on Github](https://github.com/rusoto/rusoto/blob/master/rusoto/services/sqs/src/generated.rs) if you'd like to peruse it.  It's the output from the code generation we walked through on the previous two posts.

## Making the crate from scratch

Now that we've seen what the crate structure looks like and have seen the service client generation, let's dig into how we put it all together to make this crate from scratch.

The work happens in [the service_crategen folder](https://github.com/rusoto/rusoto/tree/master/service_crategen).  We use [clap](https://github.com/kbknapp/clap-rs) to allow us to run the `service_crategen` executable with command line arguments.  To produce crates for AWS services, we run it like so:

```bash
cargo run -- generate -c ./services.json -o ../rusoto/services
```

This loads the services.json file we saw in the previous posts, containing the lists of AWS services to generate, and outputs the crates to the `../rusoto/services` folder.  The crate generation starts with this part of `main.rs`:

```rust
if let Some(matches) = matches.subcommand_matches("generate") {
    let services_config_path = matches.value_of("services_config").unwrap();

    let service_configs = ServiceConfig::load_all(services_config_path)
      .expect("Unable to read services configuration file.");

    let out_dir = Path::new(matches.value_of("out_dir").unwrap());

    commands::generate::generate_services(service_configs, out_dir);
}
```

We find the service config location from command line arguments, load the files, create a variable for the output directory and pass that all into `generate_services`.  That function is a bit long at almost 300 lines, but we'll take it a chunk at a time to see what it does.

```rust
if !out_dir.exists() {
  fs::create_dir(out_dir).expect("Unable to create output directory");
}
```

First, we create the output directory if it doesn't exist.

```rust
services.par_iter().for_each(|(name, service_config)| {
...
```

We use `par_iter` from the [Rayon crate](https://github.com/nikomatsakis/rayon).  This lets us parallelize service generation, allowing us to use all available CPU cores.  A great boon when there's over 70 supported services so far!

```rust
let service = {
    let service_definition = ServiceDefinition::load(name, &service_config.protocol_version)
      .expect(&format!("Failed to load service {}. Make sure the botocore submodule has been initialized!", name));

    Service::new(service_config.clone(), service_definition)
  };

let crate_dir = out_dir.join(&name);
let crate_name = format!("rusoto_{}", &name.replace('-', "_"));

println!("Generating crate for {} @ {}...", service.full_name(), service.api_version());
```

Here's where we load the botocore service definition.  For SQS we load its definition and create a new Rusoto codegen `Service` object to store it.  Then we create the crate directory.  For SQS it'll be `rusoto_sqs`.

```rust
if !crate_dir.exists() {
    fs::create_dir(&crate_dir)
      .expect(&format!("Unable to create directory at {}", crate_dir.display()));
}

let service_dependencies = service.get_dependencies();

let extern_crates = service_dependencies.iter().map(|(k, _)| {
    if k == "xml-rs" {
        return "extern crate xml;".into();
    }

    let safe_name = k.replace("-", "_");
    let use_macro = k == "serde_derive" || k == "log" || k == "lazy_static";
    if use_macro {
        return format!("#[macro_use]\nextern crate {};", safe_name);
    }

    format!("extern crate {};", safe_name)

}).collect::<Vec<String>>().join("\n");
```

We create the crate directory if it's not present, then get the service dependencies. A quick detour to that definition:

```rust
match self.protocol() {
    "json" => {
        dependencies.insert("serde".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
        dependencies.insert("serde_derive".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
        dependencies.insert("serde_json".to_owned(), cargo::Dependency::Simple("1.0.1".into()));
    },
    "query" | "ec2" => {
        dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.6".into()));
    },
    "rest-json" => {
        dependencies.insert("log".to_owned(), cargo::Dependency::Simple("0.3.6".into()));
        dependencies.insert("serde".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
        dependencies.insert("serde_derive".to_owned(), cargo::Dependency::Simple("1.0.2".into()));
        dependencies.insert("serde_json".to_owned(), cargo::Dependency::Simple("1.0.1".into()));
    },
    "rest-xml" => {
        dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.6".into()));
    },
    protocol => panic!("Unknown protocol {}", protocol),
}
```

Here's where we insert dependencies based on the protocol type.  SQS is a `query` type so it gets `xml-rs`.

Back to the other code:

```rust
if !crate_dir.exists() {
    fs::create_dir(&crate_dir)
      .expect(&format!("Unable to create directory at {}", crate_dir.display()));
}

let service_dependencies = service.get_dependencies();

let extern_crates = service_dependencies.iter().map(|(k, _)| {
    if k == "xml-rs" {
        return "extern crate xml;".into();
    }

    let safe_name = k.replace("-", "_");
    let use_macro = k == "serde_derive" || k == "log" || k == "lazy_static";
    if use_macro {
        return format!("#[macro_use]\nextern crate {};", safe_name);
    }

    format!("extern crate {};", safe_name)

}).collect::<Vec<String>>().join("\n");
```

`service_dependencies` uses a sorted data structure so the output code has minimal churn: dependencies will always be in the same spot when we iterate over the collection.  This reduces change in the codegen output and makes code review easier.  

For each dependency, we inspect it.  This is to see if we need to allow `#[macro_use]` or not.  If we bring `#[macro_use]` and it's not used, our build will fail since we have `unused_imports` set to fail the build.

```rust
let mut cargo_manifest = OpenOptions::new()
  .write(true)
  .truncate(true)
  .create(true)
  .open(crate_dir.join("Cargo.toml"))
  .expect("Unable to write Cargo.toml");
```

We get ready to write out the Cargo.toml file, truncating it if it exists.

```rust
let manifest = cargo::Manifest {
  package: cargo::Metadata {
      authors: Some(vec![
          "Anthony DiMarco <ocramida@gmail.com>".into(),
          "Jimmy Cuadra <jimmy@jimmycuadra.com>".into(),
          "Matthew Mayer <matthewkmayer@gmail.com>".into(),
          "Nikita Pekin <contact@nikitapek.in>".into()
      ]),

      description: Some(format!("AWS SDK for Rust - {} @ {}", service.full_name(),
        service.api_version())),

      documentation: Some("https://rusoto.github.io/rusoto/rusoto_core/index.html".into()),

      keywords: Some(vec!["AWS".into(), "Amazon".into(), name_for_keyword]),

      license: Some("MIT".into()),

      name: crate_name.clone(),

      readme: Some("README.md".into()),

      repository: Some("https://github.com/rusoto/rusoto".into()),

      version: service_config.version.clone(),

      homepage: Some("https://www.rusoto.org/".into()),

      ..cargo::Metadata::default()
  },
  dependencies: service_dependencies,

  dev_dependencies: vec![
      ("rusoto_mock".to_owned(), cargo::Dependency::Extended {
          path: Some("../../../mock".into()),
          version: Some("0.25.0".into()),
          optional: None,
          default_features: None,
          features: None
      })
  ].into_iter().collect(),

  ..cargo::Manifest::default()
};

cargo_manifest.write_all(toml::to_string(&manifest).unwrap().as_bytes()).unwrap();
```

Here's where we create and write out the Cargo.toml file for our service.  We populate the metadata, dependencies and write it all out.  This uses code we've created to represent a Cargo file.  Here's an snippet:

```rust
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Manifest {
  pub package: Metadata,

  pub badges: Option<BTreeMap<String, Badge>>,

  #[serde(rename="build-dependencies")]
  #[serde(serialize_with = "toml::ser::tables_last")]
  pub build_dependencies: BTreeMap<String, Dependency>,

  #[serde(serialize_with = "toml::ser::tables_last")]
  pub dependencies: BTreeMap<String, Dependency>,

  #[serde(rename="dev-dependencies")]
  #[serde(serialize_with = "toml::ser::tables_last")]
  pub dev_dependencies: BTreeMap<String, Dependency>,

  pub features: Option<BTreeMap<String, Vec<String>>>
}
```

Note the `BTreeMap` type used: as mentioned earlier, this ensure stable ordering when iterating through it.  Per the [Rust docs](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), calling `iter` on the `BTreeMap`

```
Gets an iterator over the entries of the map, sorted by key.
```

At this point the Cargo.toml file is all created and populated!  We went from this code:

```rust
let manifest = cargo::Manifest {
  package: cargo::Metadata {
      authors: Some(vec![
          "Anthony DiMarco <ocramida@gmail.com>".into(),
          "Jimmy Cuadra <jimmy@jimmycuadra.com>".into(),
          "Matthew Mayer <matthewkmayer@gmail.com>".into(),
          "Nikita Pekin <contact@nikitapek.in>".into()
      ]),

      description: Some(format!("AWS SDK for Rust - {} @ {}", service.full_name(),
        service.api_version())),

      documentation: Some("https://rusoto.github.io/rusoto/rusoto_core/index.html".into()),

      keywords: Some(vec!["AWS".into(), "Amazon".into(), name_for_keyword]),

      license: Some("MIT".into()),

      name: crate_name.clone(),

      readme: Some("README.md".into()),

      repository: Some("https://github.com/rusoto/rusoto".into()),

      version: service_config.version.clone(),

      homepage: Some("https://www.rusoto.org/".into()),

      ..cargo::Metadata::default()
  },
  dependencies: service_dependencies,

  dev_dependencies: vec![
      ("rusoto_mock".to_owned(), cargo::Dependency::Extended {
          path: Some("../../../mock".into()),
          version: Some("0.25.0".into()),
          optional: None,
          default_features: None,
          features: None
      })
  ].into_iter().collect(),

  ..cargo::Manifest::default()
};

cargo_manifest.write_all(toml::to_string(&manifest).unwrap().as_bytes()).unwrap();
```

to this Cargo.toml:

```bash
[package]
authors = ["Anthony DiMarco <ocramida@gmail.com>", "Jimmy Cuadra <jimmy@jimmycuadra.com>", "Matthew Mayer <matthewkmayer@gmail.com>", "Nikita Pekin <contact@nikitapek.in>"]
description = "AWS SDK for Rust - Amazon Simple Queue Service @ 2012-11-05"
documentation = "https://rusoto.github.io/rusoto/rusoto_core/index.html"
keywords = ["AWS", "Amazon", "sqs"]
license = "MIT"
name = "rusoto_sqs"
readme = "README.md"
repository = "https://github.com/rusoto/rusoto"
version = "0.27.0"
homepage = "https://www.rusoto.org/"

[build-dependencies]

[dependencies]
hyper = "0.10.0"
xml-rs = "0.6"

[dependencies.rusoto_core]
version = "0.27.0"
path = "../../core"
[dev-dependencies.rusoto_mock]
version = "0.25.0"
path = "../../../mock"
```

The README file is generated in a similar manner, using the `format!` macro to populate the service-specific parts, such as the header, `Rusoto Sqs`.

The `lib.rs` file follows the same pattern: create the file if it doesn't exist, open it and overwrite the contents with output from codegen.

Finally, `generated.rs` is again created if not present and overwritten with output from codegen.  This is where the code from the previous two posts gets dropped into, resulting in a complete crate, from scratch! 🎉

## Wrapup - what we've seen

In the two previous posts, we saw how we made Rust code to interact with AWS, using the botocore service definitions to make structs and methods.  In this post, we saw how a crate was created and populated with all the files needed to have a complete crate: a Cargo.toml file, README, `lib.rs` and `generated.rs`.
