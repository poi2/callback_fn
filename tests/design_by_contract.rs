// after_callback can add a function after the main function.
// This example shows an update of the cart's total price after adding an item or clearing items.
#[cfg(test)]
mod update_cart_total_price {
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

        // Ensure total_price is correct around clear_items.
        // Error handling is available in runtime when conditions are not ensure.
        #[around_callback(self.ensure_total_price()?)]
        fn add_item_with_bug(&mut self, item: Item) -> Result<(), String> {
            self.items.push(item);
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

    #[test]
    fn test_add_item() {
        let mut cart = Cart::new();

        cart.add_item(Item { price: 100 }).unwrap();
        cart.add_item(Item { price: 200 }).unwrap();

        assert_eq!(cart.total_price, 300);
    }

    #[test]
    #[should_panic(expected = "Total price is not correct")]
    fn test_add_item_with_bug() {
        let mut cart = Cart::new();

        cart.add_item_with_bug(Item { price: 100 }).unwrap();
    }
}
