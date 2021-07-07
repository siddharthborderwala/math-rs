# math expression evaluator

Build a math expression evaluator

Follows BODMAS/BEDMAS rule (obviously)

## What it is comprised of

1. Tokenizer
2. Parser
3. Evaluator

## How it works

1. The input is taken in as a string and the white-spaces are removed.
2. A tokenizer is instantiated that creates tokens by iterating through each character of the input.
3. An Abstract Syntax Tree is generated from the tokens.
4. Evaluation is then performed, generating a single [f64](https://doc.rust-lang.org/std/primitive.f64.html) value

## Example Expression

```sh
(5*32-2^5)*0.125  #16.0
```

## How to run

Make sure you have rust tool-chain installed, if not go [here](https://rustup.rs/)

```sh
cargo run
```

And then enter the expressions you want to evaluate.
