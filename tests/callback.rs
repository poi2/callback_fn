#[cfg(test)]
mod logging {
    use callback_fn::*;

    #[before_callback(my_logger("Start"))]
    #[after_callback(my_logger("End"))]
    fn hello(str: &str) {
        println!("Hello {}", str);
    }

    fn my_logger(msg: &str) {
        println!("{}: {}", msg, chrono::Local::now());
    }

    // hello will print:
    //
    // Start: 2024-04-01T00:00:00.000000+09:00
    // Hello world
    // End: 2024-04-01T00:00:000.000100+09:00
    #[test]
    fn test_hello() {
        hello("world");
    }
}

#[cfg(test)]
mod event_driven_programming {
    use core::time::Duration;

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
            if name.is_empty() {
                return Err("Name is empty".to_string());
            } else {
                let user = User { name };
                self.user_repository.save(user.clone()).await?;
                Ok(user)
            }
        }

        async fn create_user_cache(&self, user: User) -> Result<User, String> {
            if user.name.len() > 5 {
                return Err("Too long as cache".to_string());
            } else {
                self.user_cache.save(user.clone()).await?;
                Ok(user)
            }
        }
    }

    #[tokio::test]
    async fn test_create_user_and_create_cache() {
        let user_use_case = UserUseCase {
            user_repository: UserRepository {},
            user_cache: UserCache {},
        };
        user_use_case.create_user("foo".to_string()).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Too long as cache")]
    async fn test_create_user_but_failed_to_cache() {
        let user_use_case = UserUseCase {
            user_repository: UserRepository {},
            user_cache: UserCache {},
        };
        user_use_case
            .create_user("foobar".to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Name is empty")]
    async fn test_failed_to_create_user() {
        let user_use_case = UserUseCase {
            user_repository: UserRepository {},
            user_cache: UserCache {},
        };
        user_use_case.create_user("".to_string()).await.unwrap();
    }
}
