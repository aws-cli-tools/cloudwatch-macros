## Overview
This repository presents a collection of essential CloudFormation macros, designed for effortless deployment through SAM. It serves as an experiment in enhancing extensibility and improving the developer experience within CloudFormation and AWS SAM.

To learn more about CloudFormation, I highly recommend reading Alex DeBrie's [blog post](https://www.alexdebrie.com/posts/cloudformation-macros):

## Current Macros
- **UpperString**: Transforms strings to uppercase.
- **LowerString**: Transforms strings to lowercase.
- **MaxLength**: Restricts the maximum length of strings.
- **LatestLayerVersion**: Retrieves the latest version of a specified Lambda layer.

Each macro is implemented as an AWS Lambda Function using `Rust`. 

## Examples
* Use `Fn::Transform` to invoke the macro and pass the relevant parameter as part of the `Parameters` attribute.
```yaml
Resources:
  MyLambdaFunction:
    Type: AWS::Lambda::Function
    Properties:
      # Other function properties
      Layers:
        - Fn::Transform:
            Name: "LatestLayerVersion"
            Parameters:
              LayerArn: arn:aws:lambda:us-east-1:XXXXXX:layer:analytics-extension
```

## Contributing
We're open to contributions! If you have ideas for new macros or enhancements to existing ones, please submit a pull request or open an issue.
