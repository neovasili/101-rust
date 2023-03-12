import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as logs from 'aws-cdk-lib/aws-logs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as secrets from 'aws-cdk-lib/aws-secretsmanager';

export class ChatGptLambdaRust extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Lets create a secrete where to store the API Key, initially with no relevant auto generated value
    const chatGptApiKey = new secrets.Secret(this, 'ChatGptApiKey', {
      description: 'Secret containing the ChatGpt API key',
      secretName: 'ChatGptApiKey',
      generateSecretString: {
        passwordLength: 64,
      },
    });

    // Create Lambda with custom runtime to hold the Rust code
    const chatGptRustLambda = new lambda.Function(this, 'ChatGptRustLambda', {
      functionName: 'ChatGptRustLambda',
      handler: 'bootstrap',
      runtime: lambda.Runtime.PROVIDED_AL2,  // Custom runtime
      architecture: lambda.Architecture.ARM_64,  // ARM architecture ♥️
      code: lambda.Code.fromAsset('../build/bootstrap.zip'),
      environment: {
        RUST_LOG: 'INFO',  // This can be used to setup logging level
        API_KEY_SECRET_NAME: chatGptApiKey.secretName,  // And we pass the secret name to the function
      },
      timeout: cdk.Duration.seconds(90),  // ChatGPT can take some time to reply
    });
    if (!chatGptRustLambda.role) {
      throw new Error('Lambda role was not created');
    }
    // We grant permissions to Lambda to retrieve the API key stored in the secret
    chatGptApiKey.grantRead(chatGptRustLambda.role);

    // We do this to avoid creating unnecessary custom resources when providing logRetention to Lambda construct
    new logs.LogGroup(this, 'ChatGptRustLambdaLogGroup', {
      logGroupName: '/aws/lambda/ChatGptRustLambda',
      retention: logs.RetentionDays.ONE_WEEK,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });
  }
}
