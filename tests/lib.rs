// before_callback can add a function before the main function.
// This example shows that if the divisor is zero, it will return an error.
#[cfg(test)]
mod div_by_zero {
    use callback_fn::*;

    #[before_callback(check_zero(divisor)?)]
    fn div(dividend: i32, divisor: i32) -> Result<i32, String> {
        Ok(dividend / divisor)
    }

    fn check_zero(num: i32) -> Result<(), String> {
        if num == 0 {
            Err("Zero is not allowed".to_string())
        } else {
            Ok(())
        }
    }

    #[test]
    fn div_by_non_zero() {
        let ret = div(10, 2);

        assert_eq!(ret, Ok(5));
    }

    #[test]
    fn div_by_zero() {
        let ret = div(10, 0);

        assert_eq!(ret, Err("Zero is not allowed".to_string()));
    }
}

// after_callback can add a function after the main function.
// This example shows an update of the cart's total price after adding an item or clearing items.
#[cfg(test)]
mod update_cart_total_price {
    use callback_fn::after_callback;

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

        #[after_callback(self.update_total_price())]
        fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }

        #[after_callback(self.update_total_price())]
        fn clear_items(&mut self) {
            self.items.clear();
        }

        fn update_total_price(&mut self) {
            self.total_price = self.items.iter().map(|item| item.price).sum();
        }
    }

    #[test]
    fn update_total_price_after_add_item() {
        let mut cart = Cart::new();

        cart.add_item(Item { price: 100 });
        cart.add_item(Item { price: 200 });

        assert_eq!(cart.total_price, 300);
    }

    #[test]
    fn update_total_price_after_clear_items() {
        let mut cart = Cart::new();

        cart.add_item(Item { price: 100 });
        cart.add_item(Item { price: 200 });
        cart.clear_items();

        assert_eq!(cart.total_price, 0);
    }
}

// Currently, there is no macro to simultaneously add functions before or after a function.
// Please use before_callback and after_callback separately.
#[cfg(test)]
mod debug {
    use callback_fn::*;

    #[before_callback(println!("Before: {:?}", chrono::Local::now()))]
    #[after_callback(println!("After: {:?}", chrono::Local::now()))]
    fn hello(name: &str) {
        println!("Hello {}", name)
    }

    // hello will print:
    //
    // Before: 2024-04-01T00:00:00.000000+09:00
    // Hello world
    // After: 2024-04-01T00:00:000.000200+09:00
    #[test]
    fn test_hello() {
        hello("world");
    }
}

// This example shows that if the user has permission, it will return Ok(()).
// Otherwise, it will return Err(PermissionError).
// You need make sure that the callback function and main function have the same return type.
#[cfg(test)]
mod authentication {
    use callback_fn::*;

    #[before_callback(has_permission(user, Permission::Read).map_err(UseCaseError::from)?)]
    fn read(user: &User) -> Result<(), UseCaseError> {
        Ok(())
    }

    #[before_callback(has_permission(user, Permission::Create).map_err(UseCaseError::from)?)]
    fn create(user: &User) -> Result<(), UseCaseError> {
        Ok(())
    }

    fn general_user() -> User {
        User {
            permissions: vec![Permission::Read],
        }
    }

    fn admin_user() -> User {
        User {
            permissions: vec![Permission::Read, Permission::Create],
        }
    }

    #[derive(Debug)]
    struct User {
        permissions: Vec<Permission>,
    }

    fn has_permission(user: &User, permission: Permission) -> Result<(), PermissionError> {
        if !user.permissions.contains(&permission) {
            Err(PermissionError::PermissionDenied(format!(
                "{:?}",
                permission
            )))
        } else {
            Ok(())
        }
    }

    #[derive(Debug, PartialEq)]
    enum Permission {
        Read,
        Create,
    }

    #[derive(thiserror::Error, Debug)]
    pub enum PermissionError {
        #[error("Denied access to {0}")]
        PermissionDenied(String),
    }

    #[derive(thiserror::Error, Debug)]
    pub enum UseCaseError {
        #[error("PermissionError {0}")]
        PermissionError(#[from] PermissionError),
    }

    #[test]
    fn general_user_can_read() {
        assert!(read(&general_user()).is_ok());
    }

    #[test]
    fn admin_user_can_read() {
        assert!(read(&admin_user()).is_ok());
    }

    #[test]
    fn general_user_can_not_create() {
        assert!(create(&general_user()).is_err());
    }

    #[test]
    fn admin_user_can_create() {
        assert!(create(&admin_user()).is_ok());
    }
}

// Integration with other features and crates.
#[cfg(test)]
mod integration_with_other_features_and_crates {
    use callback_fn::*;

    /// Test for documentation comments.
    // Test for normal comments.
    #[before_callback(println!("Before: {:?}", chrono::Local::now()))]
    #[after_callback(println!("After: {:?}", chrono::Local::now()))]
    fn with_comments_1(name: &str) {
        println!("Hello {}", name)
    }

    #[test]
    fn test_for_comments_1() {
        with_comments_1("world");
    }

    #[doc = "Test for comments."]
    #[before_callback(println!("Before: {:?}", chrono::Local::now()))]
    #[after_callback(println!("After: {:?}", chrono::Local::now()))]
    fn with_comments_2(name: &str) {
        println!("Hello {}", name)
    }

    #[test]
    fn test_for_comments_2() {
        with_comments_2("world");
    }
}
