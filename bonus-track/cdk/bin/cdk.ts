#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { ChatGptLambdaRust } from '../lib/chat-gpt-lambda-rust';

const app = new cdk.App();
new ChatGptLambdaRust(app, 'ChatGptLambdaRust', {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION
  },
});
