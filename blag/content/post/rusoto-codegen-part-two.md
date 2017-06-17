+++
date = "2017-06-16T21:28:53-07:00"
draft = false
title = "Rusoto codegen, part two"
+++

In the [previous post](https://matthewkmayer.github.io/blag/public/post/rusoto-codegen/) we took a quick tour of the major pieces of [Rusoto](https://github.com/rusoto/rusoto) code generation.  In this post we'll get deeper into code generation for the Simple Queue Service.

<!--more-->

## Picking up from part one

Part one described a few parts of Rusoto codegen:

1. Finding which services to generate
2. Making the service's crate
3. Generating the service and placing it inside its crate
4. `rustfmt` the generated code to make it look pretty

Step three is where this post will concentrate.  Since part one was written, some name spacing has changed but the code does the same thing.  In [service_crategen's main.rs file](https://github.com/rusoto/rusoto/blob/master/service_crategen/src/main.rs):

```rust
let mut gen_writer = BufWriter::new(gen_file);
codegen::generator::generate_source(&service, &mut gen_writer).unwrap();
```

The `generate_source` function is defined in [service_crategen/codegen/generator/mod.rs](https://github.com/rusoto/rusoto/blob/master/service_crategen/src/codegen/generator/mod.rs):

```rust
pub fn generate_source(service: &Service, writer: &mut FileWriter) -> IoResult {
    match service.protocol() {
        "json" => generate(writer, service, JsonGenerator, JsonErrorTypes),
        "query" | "ec2" => generate(writer, service, QueryGenerator, XmlErrorTypes),
        "rest-json" => generate(writer, service, RestJsonGenerator, JsonErrorTypes),
        "rest-xml" => generate(writer, service, RestXmlGenerator, XmlErrorTypes),
        protocol => panic!("Unknown protocol {}", protocol),
    }
}
```

The function signature shows it takes a reference to a `Service` and a mutable reference to a `FileWriter`.  The `FileWriter` type is supplied by the `BufWriter` we created just before calling `generate_source`.  The `Service` type is declared in [service.rs](https://github.com/rusoto/rusoto/blob/master/service_crategen/src/service.rs) in the service_crategen project:

```rust
#[derive(Debug)]
pub struct Service {
    config: ::ServiceConfig,
    definition: ServiceDefinition
}

impl Service {
    pub fn new(config: ServiceConfig, definition: ServiceDefinition) -> Self {
        Service {
            config: config,
            definition: definition
        }
    }

    pub fn name(&self) -> &str {
        match self.definition.metadata.service_abbreviation {
            Some(ref service_abbreviation) => service_abbreviation.as_str(),
            None => self.definition.metadata.service_full_name.as_ref()
        }
    }

    pub fn full_name(&self) -> &str {
        &self.definition.metadata.service_full_name
    }

    pub fn protocol(&self) -> &str {
        &self.definition.metadata.protocol
    }
...
```

The `Service` type wraps up both a `ServiceConfig` and `ServiceDefinition`.  ServiceDefinition is what we saw in part one and looked like this:

```rust
#[derive(Debug, Deserialize)]
pub struct ServiceDefinition {
    pub documentation: Option<String>,

    pub examples: Option<BTreeMap<String, String>>,

    pub metadata: Metadata,

    pub operations: BTreeMap<String, Operation>,

    #[serde(deserialize_with="ShapesMap::deserialize_shapes_map")]
    pub shapes: BTreeMap<String, Shape>,

    pub version: Option<String>
}
```

ServiceConfig is populate from the list of services we want to generate.  It stores this JSON:

```json
"sqs" : {
    "version": "0.25.0",
    "coreVersion": "0.25.0",
    "protocolVersion": "2012-11-05",
    "baseTypeName": "Sqs"
  }
```

in this form:

```rust
#[derive(Clone, Debug, Deserialize)]
pub struct ServiceConfig {
    pub version: String,
    #[serde(rename = "coreVersion")]
    pub core_version: String,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "customDependencies")]
    pub custom_dependencies: Option<BTreeMap<String, cargo::Dependency>>,
    #[serde(rename = "baseTypeName")]
    pub base_type_name: String
}
```

Back to `generate_source`!

```rust
pub fn generate_source(service: &Service, writer: &mut FileWriter) -> IoResult {
    match service.protocol() {
        "json" => generate(writer, service, JsonGenerator, JsonErrorTypes),
        "query" | "ec2" => generate(writer, service, QueryGenerator, XmlErrorTypes),
        "rest-json" => generate(writer, service, RestJsonGenerator, JsonErrorTypes),
        "rest-xml" => generate(writer, service, RestXmlGenerator, XmlErrorTypes),
        protocol => panic!("Unknown protocol {}", protocol),
    }
}
```

From the [botocore SQS definition](https://github.com/boto/botocore/blob/develop/botocore/data/sqs/2012-11-05/service-2.json) we know SQS uses the `query` protocol:

```json
"metadata" : {
    "apiVersion":"2012-11-05",
    "endpointPrefix":"sqs",
    "protocol":"query",
    "serviceAbbreviation":"Amazon SQS",
    "serviceFullName":"Amazon Simple Queue Service",
    "signatureVersion":"v4",
    "uid":"sqs-2012-11-05",
    "xmlNamespace":"http://queue.amazonaws.com/doc/2012-11-05/"
  }
```

This means we'll be taking this branch in the `match` statement:

```rust
"query" | "ec2" => generate(writer, service, QueryGenerator, XmlErrorTypes),
```

This passes the `writer` and `service` variables along with `QueryGenerator` and `XmlErrorTypes` into `generate`.  Let's look into `generate`, defined in [service_crategen/src/codegen/generator/mod.rs](https://github.com/rusoto/rusoto/blob/master/service_crategen/src/codegen/generator/mod.rs):

```rust
fn generate<P, E>(writer: &mut FileWriter, service: &Service, protocol_generator: P, error_type_generator: E) -> IoResult
    where P: GenerateProtocol,
          E: GenerateErrorTypes {

    writeln!(writer, "#[allow(warnings)]
        use hyper::Client;
        use hyper::status::StatusCode;
        use rusoto_core::request::DispatchSignedRequest;
        use rusoto_core::region;
        use std::fmt;
        use std::error::Error;
        use rusoto_core::request::HttpDispatchError;
        use rusoto_core::credential::{{CredentialsError, ProvideAwsCredentials}};
    ")?;

    protocol_generator.generate_prelude(writer, service)?;
    generate_types(writer, service, &protocol_generator)?;
    error_type_generator.generate_error_types(writer, service)?;
    generate_client(writer, service, &protocol_generator)?;
    generate_tests(writer, service)?;

    Ok(())
}
```

We're finally seeing some new Rust features: `generate<P, E>` means we're using generics and the `?` operator replaces the `try!` macro.

Breaking it down:

```rust
fn generate<P, E>(writer: &mut FileWriter, service: &Service, protocol_generator: P, error_type_generator: E) -> IoResult
    where P: GenerateProtocol,
          E: GenerateErrorTypes { ...
```

`generate` takes a total of four arguments.  Two are generics, `P` and `E`.  They are defined in the `where` block: `P: GenerateProtocol` means `P` accepts anything that fulfills the `GenerateProtocol` trait.  Similarly, `E` accepts anything that implements the `GenerateErrorTypes` trait.  This is used in the signature so the `protocol_generator` arg is anything that implements the `GenerateProtocol` trait and `error_type_generator` is for the `GenerateErrorTypes` trait.

Now that we've rephrased that a few times, let's go to the next part:

```rust
writeln!(writer, "#[allow(warnings)]
  use hyper::Client;
  use hyper::status::StatusCode;
  use rusoto_core::request::DispatchSignedRequest;
  use rusoto_core::region;
  use std::fmt;
  use std::error::Error;
  use rusoto_core::request::HttpDispatchError;
  use rusoto_core::credential::{{CredentialsError, ProvideAwsCredentials}};
")?;
```

Now we're getting to Rust code for SQS!  This is our prelude to the service.  Every AWS service Rusoto supports has this code at the top of `generated.rs`.  Take a peek at [SQS' generated.rs file](https://github.com/rusoto/rusoto/blob/master/rusoto/services/sqs/src/generated.rs).  This brings in all the items we need for talking to AWS: [hyper](https://github.com/hyperium/hyper) for HTTP(s) requests, wrappers for signing AWS requests from `rusoto_core::request`, AWS regions from `rusoto_core::region`, etc...  We also bring in AWS credential providing helpers.

```rust
protocol_generator.generate_prelude(writer, service)?;
generate_types(writer, service, &protocol_generator)?;
error_type_generator.generate_error_types(writer, service)?;
generate_client(writer, service, &protocol_generator)?;
generate_tests(writer, service)?;
```

This section is tailored to the specific service being generated.  `protocol_generator` implements talking to AWS and `error_type_generator` handles all the errors AWS could return for the service.  We'll get to that right after the last part of `generate`:

```rust
Ok(())
```

If we hit here, everything is okay and we return the `Result` type with an empty object, `()`.

Back to the service-specific code generation!

```rust
protocol_generator.generate_prelude(writer, service)?;
```
