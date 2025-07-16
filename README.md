# Message Storage Contract

A simple ink! smart contract for storing and retrieving text messages.

## Features

- Store a text message on-chain
- Update the message anytime
- Retrieve the current message

## Functions

- `new(init_message: String)` - Create contract with initial message
- `default()` - Create contract with empty message
- `set_message(new_message: String)` - Update the stored message
- `get_message() -> String` - Get the current message

## Build

```bash
cargo contract build
```

## Test

```bash
cargo test
```

## Deploy

```bash
cargo contract instantiate --constructor new --args "Hello World" --suri //Alice
```