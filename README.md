# Callback function for Rust

callback_fn is a library that adds functions before and after the target function.

## Features

- User can specify custom functions that are executed before and after the target function.
- Can be seamlessly integrated into existing codebase.
- Can handle errors that occur in the callback function.

## Uses

- Callback function: Add function before and after the target function.
- Design-by-contracts: Add pre-conditions and post-conditions to the target function.

## Installation

Add callback_fn to your `Cargo.toml`.

```toml
[dependencies]
callback_fn = "0.0.1"
```

## Examples

### For callback

After user created, user cache will be created.

```rust
```

### For logging

Add logging before and after the target function.

```rust
```

### For Design-by-contract

After adding to the cart or cleaning, ensure if the total_price is correct.

```rust
```

### For Authentication

Add authentication before UseCase function.

```rust
```
