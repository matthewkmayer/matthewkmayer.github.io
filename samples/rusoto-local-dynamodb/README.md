# Example of running Rusoto against a local DynamoDB instance

## Start local Dynamo

[Download from Amazon](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.DownloadingAndRunning.html). This example uses the Java jar.

Run it:

`$ java -Djava.library.path=./DynamoDBLocal_lib -jar DynamoDBLocal.jar -sharedDb`

## Run the executable against it

Supply some fake AWS credentials, as any credentials work for the local version:

`$ AWS_ACCESS_KEY_ID=foo AWS_SECRET_ACCES_KEY=bar cargo run`

## Custom Region in Rusoto

See [main.rs](src/main.rs).

## Deleting the local Dynamo data

Where the `java` command was run, delete the `shared-local-instance.db` file:

`$ rm shared-local-instance.db`
