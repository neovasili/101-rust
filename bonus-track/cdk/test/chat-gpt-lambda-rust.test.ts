import * as cdk from 'aws-cdk-lib';
import { Template, Match } from 'aws-cdk-lib/assertions';
import { ChatGptLambdaRust } from '../lib/chat-gpt-lambda-rust';

describe('ChatGptLambdaRust stack high level specs', () => {
  let template: Template;

  beforeAll(() => {
    const app = new cdk.App();
    const stack = new ChatGptLambdaRust(app, 'MyTestStack');
    template = Template.fromStack(stack);
  });

  test('Secret for API Key is created', () => {
    template.hasResourceProperties('AWS::SecretsManager::Secret', {
      Name: 'ChatGptApiKey',
    });
  });

  test('Rust Lambda is created', () => {
    template.hasResourceProperties('AWS::Lambda::Function', {
      FunctionName: 'ChatGptRustLambda',
      Handler: 'bootstrap',
      Runtime: 'provided.al2',
      Timeout: 90,
      Architectures: [
        'arm64',
      ],
    });
  });

  test('Rust Lambda log group is created', () => {
    template.hasResourceProperties('AWS::Logs::LogGroup', {
      LogGroupName: '/aws/lambda/ChatGptRustLambda',
      RetentionInDays: 7,
    });
  });
});

describe('Rust Lambda low level specs', () => {
  let template: Template;

  beforeAll(() => {
    const app = new cdk.App();
    const stack = new ChatGptLambdaRust(app, 'MyTestStack');
    template = Template.fromStack(stack);
  });

  test('Rust Lambda have permissions to read API Key secret', () => {
    const apiKeySecret = template.findResources('AWS::SecretsManager::Secret', {
      Properties: {
        Name: 'ChatGptApiKey',
      },
    });
    const apiKeySecretLogicalId = Object.keys(apiKeySecret)[0];

    const rustLambda = template.findResources('AWS::Lambda::Function', {
      Properties: {
        FunctionName: 'ChatGptRustLambda',
      },
    });
    const rustLambdaLogicalId = Object.keys(rustLambda)[0];
    const lambdaRoleLogicaId = rustLambda[rustLambdaLogicalId].Properties.Role['Fn::GetAtt'][0];

    template.hasResourceProperties('AWS::IAM::Policy', {
      Roles: [{
        Ref: lambdaRoleLogicaId,
      }],
      PolicyDocument: {
        Statement: [{
          /* 
          Required actions to read secret value.
          Since we are using the `@aws-cdk/aws-iam:minimizePolicies` feature flag,
          actions will be received in alphabetical order
          */
          Action: Match.arrayWith([
            'secretsmanager:GetSecretValue',
            'secretsmanager:DescribeSecret',
          ]),
          Effect: 'Allow',
          // Only allow for this resource
          Resource: {
            Ref: apiKeySecretLogicalId,
          },
        }],
      },
    });
  });
});
