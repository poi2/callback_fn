#[cfg(test)]
mod logging {
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
}

#[cfg(test)]
mod event_driven_programming {
    use core::time::Duration;
    use std::collections::HashMap;

    use callback_fn::after_callback;

    #[allow(dead_code)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct User {
        id: usize,
        name: String,
    }
    impl User {
        fn new(id: usize, name: String) -> Result<Self, String> {
            // Failed to create user when name is empty for testing.
            if name.is_empty() {
                return Err("Name is empty".to_string());
            }
            Ok(Self { id, name })
        }
    }

    struct UserRepository {}
    impl UserRepository {
        async fn save(&self, _user: User) -> Result<(), String> {
            tokio::time::sleep(Duration::from_micros(1)).await;
            Ok(())
        }
    }

    struct UserCache {
        map: HashMap<usize, User>,
    }
    impl UserCache {
        async fn save(&mut self, user: User) -> Result<(), String> {
            tokio::time::sleep(Duration::from_micros(1)).await;
            self.map.insert(user.id, user.clone());
            Ok(())
        }
    }

    struct UserUseCase {
        user_repository: UserRepository,
        user_cache: UserCache,
    }

    impl UserUseCase {
        #[after_callback(let _ = self.create_user_cache(ret.clone()?).await?)]
        async fn create_user(&mut self, id: usize, name: String) -> Result<User, String> {
            let user = User::new(id, name)?;
            self.user_repository.save(user.clone()).await?;
            Ok(user)
        }

        async fn create_user_cache(&mut self, user: User) -> Result<User, String> {
            // Failed to cache when user id is even for testing.
            if user.id % 2 == 0 {
                return Err("Failed to cache".to_string());
            }
            self.user_cache.save(user.clone()).await?;
            Ok(user)
        }
    }

    #[tokio::test]
    async fn test_create_user_and_create_cache() {
        let mut user_use_case = UserUseCase {
            user_repository: UserRepository {},
            user_cache: UserCache {
                map: HashMap::new(),
            },
        };

        let user = user_use_case
            .create_user(1, "alice".to_string())
            .await
            .unwrap();

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "alice".to_string());

        let user_in_cache = user_use_case.user_cache.map.get(&1).unwrap();
        assert_eq!(user_in_cache.id, 1);
        assert_eq!(user_in_cache.name, "alice".to_string());
    }

    #[tokio::test]
    #[should_panic(expected = "Name is empty")]
    async fn test_create_user_but_failed_to_create_user() {
        let mut user_use_case = UserUseCase {
            user_repository: UserRepository {},
            user_cache: UserCache {
                map: HashMap::new(),
            },
        };

        user_use_case.create_user(1, "".to_string()).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Failed to cache")]
    async fn test_create_user_but_failed_to_cache() {
        let mut user_use_case = UserUseCase {
            user_repository: UserRepository {},
            user_cache: UserCache {
                map: HashMap::new(),
            },
        };

        user_use_case
            .create_user(2, "alice".to_string())
            .await
            .unwrap();
    }
}
