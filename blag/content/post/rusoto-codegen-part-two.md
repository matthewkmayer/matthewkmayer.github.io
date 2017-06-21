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

ServiceConfig is populated from the list of services we want to generate.  It stores this JSON:

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

We're finally seeing some slightly more advanced Rust features: `generate<P, E>` means we're using generics and the `?` operator replaces the `try!` macro.

Breaking it down:

```rust
fn generate<P, E>(writer: &mut FileWriter, service: &Service, 
    protocol_generator: P, error_type_generator: E) -> IoResult

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

Here's the next part of `generate`:

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

In here, the `protocol_generator` variable refers to `QueryGenerator`.  This is from the earlier `match` statement: `generate(writer, service, QueryGenerator, XmlErrorTypes)`.  `QueryGenerator` is implemented in [service_crategen/src/codegen/generator/query.rs](https://github.com/rusoto/rusoto/blob/master/service_crategen/src/codegen/generator/query.rs).  Here's how it implements the `GenerateProtocol` trait:

```rust
pub struct QueryGenerator;

impl GenerateProtocol for QueryGenerator {
...
  fn generate_prelude(&self, writer: &mut FileWriter, _service: &Service) -> IoResult {
      writeln!(writer,
               "use std::str::FromStr;
          use xml::EventReader;
          use xml::reader::ParserConfig;
          use rusoto_core::param::{{Params, ServiceParams}};
          use rusoto_core::signature::SignedRequest;
          use xml::reader::XmlEvent;
          use rusoto_core::xmlutil::{{Next, Peek, XmlParseError, XmlResponse}};
          use rusoto_core::xmlutil::{{characters, end_element, start_element, skip_tree, peek_at_name}};
          use rusoto_core::xmlerror::*;

          enum DeserializerNext {{
              Close,
              Skip,
              Element(String),
      }}")
  }
...
```

There's more to that trait, but we'll focus on `generate_prelude`.  While we've already created a generic prelude all services share, the `query` type needs additional imports.  For example we need to parse the XML payloads returned by SQS, so we bring in items from the [xml crate](https://github.com/netvl/xml-rs).  We also bring in Rusoto xmlutil helpers to make the code more concise.

Moving on to `generate_types(writer, service, &protocol_generator)?;`, it's in [service_crategen/src/codegen/generator/mod.rs](https://github.com/rusoto/rusoto/blob/master/service_crategen/src/codegen/generator/mod.rs):

```rust
fn generate_types<P>(writer: &mut FileWriter, service: &Service, protocol_generator: &P) -> IoResult
  where P: GenerateProtocol {

  let (serialized_types, deserialized_types) = filter_types(service);

  for (name, shape) in service.shapes().iter() {
        let type_name = mutate_type_name(&name);

        // We generate enums for error types, so no need to create model objects for them
        if shape.exception() {
            continue;
        }

        // If botocore includes documentation, clean it up a bit and use it
        if let Some(ref docs) = shape.documentation {
            writeln!(writer, "#[doc=\"{}\"]",
                               docs.replace("\\", "\\\\").replace("\"", "\\\""))?;
        }

        let deserialized = deserialized_types.contains(&type_name);
        let serialized = serialized_types.contains(&type_name);

        // generate a rust type for the shape
        if type_name != "String" {
            let generated_type = match shape.shape_type {
                ShapeType::Structure => {
                    generate_struct(service,
                                     &type_name,
                                     &shape,
                                     serialized,
                                     deserialized,
                                     protocol_generator)
                }
                ShapeType::Map => generate_map(&type_name, &shape),
                ShapeType::List => generate_list(&type_name, &shape),
                shape_type => {
                    generate_primitive_type(&type_name,
                                             shape_type,
                                             protocol_generator.timestamp_type())
                }
            };
            writeln!(writer, "{}", generated_type)?;
        }

        if deserialized {
            if let Some(deserializer) = protocol_generator.generate_deserializer(&type_name, &shape, service) {
                writeln!(writer, "{}", deserializer)?;
            }
        }

        if serialized {
            if let Some(serializer) = protocol_generator.generate_serializer(&type_name, &shape, service) {
                writeln!(writer, "{}", serializer)?;
            }
        }
      }
      Ok(())
}
```

The first thing we do is filter out the types.  This splits types into how they are used: inputs to AWS services and outputs.  This allows us to generate deserializers for outputs from AWS and serializers for inputs.  If we didn't do this, the generated code would have lots of unused code and make the services files larger than they have to be.  We then iterate over each shape in the service and generate its Rust equivalent.  Taking `generate_primitive_type` as an example:

```rust
fn generate_primitive_type(name: &str, shape_type: ShapeType, for_timestamps: &str) -> String {
  let primitive_type = match shape_type {
    ShapeType::Blob => "Vec<u8>",
    ShapeType::Boolean => "bool",
    ShapeType::Double => "f64",
    ShapeType::Float => "f32",
    ShapeType::Integer => "i64",
    ShapeType::Long => "i64",
    ShapeType::String => "String",
    ShapeType::Timestamp => for_timestamps,
    primitive_type => panic!("Unknown primitive type: {:?}", primitive_type),
  };

  format!("pub type {} = {};", name, primitive_type)
}
```

This is fairly straightforward.  Take the type defined in the service definition and map them to a Rust type.  Booleans map to `bool`, floats to `f32`, etc...  One special case is `for_timestamps` where we let the caller determine how it wants it to look.

The `generate_types` function also handles translating botocore documentation to rustdoc in the code.  Towards the end of the function it handles making serializers and deserializers if the shape needs them.  Finally it returns `Ok(())` to mark that part of codegen as complete.

The remainder of the code generation follows the same pattern: take the botocore service definition and translate that to Rust code.  Here's the parts we haven't covered:

```rust
error_type_generator.generate_error_types(writer, service)?;
generate_client(writer, service, &protocol_generator)?;
generate_tests(writer, service)?;
```

Error types are errors AWS can return for requests.  We turn those into Rust code so we can have typed error messages.  `generate_client` is where the rubber meets the road and we create the Rust client for the service.  Take a look at [SqsClient docs](https://rusoto.github.io/rusoto/rusoto_sqs/struct.SqsClient.html) to see what it generates.  Finally, `generate_tests` looks through botocore and translates the parsing tests from botocore to Rust code.  This means we have generated, automated tests for ensuring our deserializers and error handlers can handle examples of what AWS returns without actually making AWS calls.

## Packaging up the code

After this deeper look of how we make Rust code, we still need to go over how the generated code is used in the crate.  Continuing with SQS, the next post will look into how the `rusoto_sqs` crate is created from scratch and populated with code from codegen covered in this post.

Thanks for reading!
