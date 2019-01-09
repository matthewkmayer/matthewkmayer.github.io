# Example of running Rusoto against a local DynamoDB instance

## Start local Dynamo

[Download from Amazon](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.DownloadingAndRunning.html). This example uses the Java jar.

Run it:

`$ java -Djava.library.path=./DynamoDBLocal_lib -jar DynamoDBLocal.jar -sharedDb`

## Run the executable against it

Supply some fake AWS credentials, as any credentials work for the local version:

`$ AWS_SECRET_KEY=foo AWS_SECRET_ACCES_KEY=bar cargo run`

## Custom Region in Rusoto

See source code/here's a snippet.
