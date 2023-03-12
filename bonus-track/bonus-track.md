# ChatGPT Lambda function with Rust and CDK

&#x2B11; [Return to index](../README.md)

This is a small project that uses CDK and Rust to build a Lambda function that will send a prompt to ChatGPT and will return the received reply to the user.

## Requirements

- Rust
- AWS CDK
- Docker

## Content

```shell
.
├── cdk
│   ├── README.md
│   ├── bin
│   ├── cdk.json
│   ├── jest.config.js
│   ├── lib
│   ├── package.json
│   ├── tsconfig.json
│   └── yarn.lock
└── lambda-code
    ├── Cargo.lock
    ├── Cargo.toml
    ├── build.sh
    ├── src
    └── target
```

We have the `cdk` folder containing the CDK project that will allow us deploy the Rust Lambda function and a Secret parameter where to store the ChatGPT APIKey.

And we also have the `lambda-code` folder that contains the Rust application to be deployed in the Lambda function.

## References

- [aws-config crate](https://crates.io/crates/aws-config)
- [aws-sdk-secretsmanager crate](https://crates.io/crates/aws-sdk-secretsmanager)
- [lambda_runtime crate](https://crates.io/crates/lambda_runtime)
- [reqwest crate](https://crates.io/crates/reqwest)
- [tokio crate](https://crates.io/crates/tokio)
- [serde crate](https://crates.io/crates/serde)
- [serde_json crate](https://crates.io/crates/serde_json)
- [tracing crate](https://crates.io/crates/tracing)
- [tracing-subscriber crate](https://crates.io/crates/tracing-subscriber)
- [AWS CDK](https://github.com/aws/aws-cdk)
- [Docker image to build the Lambda Rust code](https://github.com/rust-serverless/lambda-rust)
