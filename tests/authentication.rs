// This example shows that if the user has permission, it will return Ok(()).
// Otherwise, it will return Err(PermissionError).
// You need make sure that the callback function and main function have the same return type.
#[cfg(test)]
mod authentication {
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

    fn general_user() -> User {
        User {
            permissions: vec![Permission::ReadPost],
        }
    }

    fn admin_user() -> User {
        User {
            permissions: vec![Permission::ReadPost, Permission::CreatePost],
        }
    }

    #[test]
    fn general_user_can_get_post_by_id() {
        assert!(get_post_by_id(&general_user(), 1).is_ok());
    }

    #[test]
    fn admin_user_can_get_post_by_id() {
        assert!(get_post_by_id(&admin_user(), 1).is_ok());
    }

    #[test]
    fn general_user_can_not_create_post() {
        assert!(create_post(&general_user(), "title".to_string(), "body".to_string()).is_err());
    }

    #[test]
    fn admin_user_can_create_post() {
        assert!(create_post(&admin_user(), "title".to_string(), "body".to_string()).is_ok());
    }
}
