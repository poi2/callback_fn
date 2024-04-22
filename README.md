# Callback function for Rust

callback_fn is a library that adds functions before, after and around the target function.

## Features

- **Custom Functions**: Users can specify custom functions to be executed before, after, and around the target function.
- **Seamless Integration**: Integrates seamlessly into existing codebases.
- **Error Handling**: Handles errors that occur within the callback functions

## Uses

- **Callback Functions**: Add functions before and after the target function.
    - Useful for tasks such as logging, authentication, and other cross-cutting concerns.
- **Design-by-Contracts**: Add pre-conditions and post-conditions to the target function.
    - Specific conditions can be applied, such as only when testing, using the features flag.

## Installation

Add callback_fn to your `Cargo.toml`.

```toml
[dependencies]
callback_fn = "0.1.0"
```

## Examples

### For callback

After user created, user cache will be created.

```rust
use callback_fn::after_callback;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct User {
    name: String,
}

struct UserRepository {}
impl UserRepository {
    async fn save(&self, _user: User) -> Result<(), String> {
        tokio::time::sleep(Duration::from_micros(1)).await;
        Ok(())
    }
}

struct UserCache {}
impl UserCache {
    async fn save(&self, _user: User) -> Result<(), String> {
        tokio::time::sleep(Duration::from_micros(1)).await;
        Ok(())
    }
}

struct UserUseCase {
    user_repository: UserRepository,
    user_cache: UserCache,
}

impl UserUseCase {
    #[after_callback(let _ = self.create_user_cache(ret.clone()?).await?)]
    async fn create_user(&self, name: String) -> Result<User, String> {
        let user = User { name };
        self.user_repository.save(user.clone()).await?;
        Ok(user)
    }

    async fn create_user_cache(&self, user: User) -> Result<User, String> {
        self.user_cache.save(user.clone()).await?;
        Ok(user)
    }
}
```

### For logging

Add logging around the target function.

```rust
use callback_fn::around_callback;

#[around_callback(my_logger())]
fn hello(str: &str) {
    println!("Hello {}", str);
}

fn my_logger() {
    println!("{}", chrono::Local::now());
}

// hello will print:
//
// 2024-04-01T00:00:000.000000+09:00
// Hello world
// 2024-04-01T00:00:000.000100+09:00
#[test]
fn test_hello() {
    hello("world");
}
```

### For Authentication

Add authentication before UseCase function.

```rust
use callback_fn::before_callback;
use strum_macros::Display;

#[before_callback(has_permission(current_user, Permission::ReadPost).map_err(UseCaseError::from)?)]
fn get_post_by_id(current_user: &User, id: usize) -> Result<Post, UseCaseError> {
    Ok(Post {
        id,
        title: "Dummy Title".to_string(),
        body: "Dummy Body".to_string(),
    })
}

#[before_callback(has_permission(current_user, Permission::CreatePost).map_err(UseCaseError::from)?)]
fn create_post(current_user: &User, title: String, body: String) -> Result<Post, UseCaseError> {
    Ok(Post { id: 1, title, body })
}

#[derive(Debug)]
struct User {
    permissions: Vec<Permission>,
}

#[derive(Debug, Display, PartialEq)]
pub enum Permission {
    ReadPost,
    CreatePost,
}

fn has_permission(user: &User, permission: Permission) -> Result<(), PermissionError> {
    if user.permissions.contains(&permission) {
        Ok(())
    } else {
        Err(PermissionError::PermissionDenied(permission))
    }
}

#[derive(Debug, PartialEq)]
struct Post {
    id: usize,
    title: String,
    body: String,
}

#[derive(thiserror::Error, Debug)]
pub enum PermissionError {
    #[error("User don't have {0} permission.")]
    PermissionDenied(Permission),
}

#[derive(thiserror::Error, Debug)]
pub enum UseCaseError {
    #[error("PermissionError: {0}")]
    PermissionError(#[from] PermissionError),
}
```

### For Design-by-contract

After adding to the cart or cleaning, ensure if the total_price is correct.

```rust
use callback_fn::around_callback;

struct Cart {
    total_price: usize,
    items: Vec<Item>,
}
struct Item {
    price: usize,
}

impl Cart {
    fn new() -> Self {
        Self {
            total_price: 0,
            items: vec![],
        }
    }

    // Ensure total_price is correct around add_item.
    // Error handling is available in runtime when conditions are not ensure.
    #[around_callback(self.ensure_total_price()?)]
    fn add_item(&mut self, item: Item) -> Result<(), String> {
        self.items.push(item);
        self.update_total_price();
        Ok(())
    }

    fn update_total_price(&mut self) {
        self.total_price = self.items.iter().map(|item| item.price).sum()
    }

    fn ensure_total_price(&self) -> Result<(), String> {
        if self.total_price == self.items.iter().map(|item| item.price).sum() {
            Ok(())
        } else {
            Err("Total price is not correct".to_string())
        }
    }
}
```

### Use only in specific features

If you want to use callback_fn only in specific features, you can use `cfg_attr`.

```rust
use callback_fn::after_callback;

#[cfg_attr(test, after_callback(bar()))]
fn foo() {
    println!("foo");
}

fn bar() {
    println!("bar");
}
```

If you run `cargo run --features test`, foo function will be like below.

```rust
fn foo() {
    #[allow(unused_mut)]
    let mut ret = {
        {
            ::std::io::_print(format_args!("foo\n"));
        };
    };
    bar();
    ret
}
fn bar() {
    {
        ::std::io::_print(format_args!("bar\n"));
    };
}
```

If you run `cargo run`, foo function will be like below.

```rust
fn foo() {
    {
        ::std::io::_print(format_args!("foo\n"));
    };
}
fn bar() {
    {
        ::std::io::_print(format_args!("bar\n"));
    };
}
```
