// This example shows that if the user has permission, it will return Ok(()).
// Otherwise, it will return Err(PermissionError).
// You need make sure that the callback function and main function have the same return type.
#[cfg(test)]
mod authentication {
    use callback_fn::before_callback;
    use strum_macros::Display;

    #[before_callback(has_permission(current_user, Permission::ReadArticle).map_err(UseCaseError::from)?)]
    fn get_article_by_id(current_user: &User, id: usize) -> Result<Article, UseCaseError> {
        Ok(Article {
            id,
            title: "Dummy Title".to_string(),
            body: "Dummy Body".to_string(),
        })
    }

    #[before_callback(has_permission(current_user, Permission::CreateArticle).map_err(UseCaseError::from)?)]
    fn create_article(
        current_user: &User,
        title: String,
        body: String,
    ) -> Result<Article, UseCaseError> {
        Ok(Article { id: 1, title, body })
    }

    #[derive(Debug)]
    struct User {
        permissions: Vec<Permission>,
    }

    #[derive(Debug, Display, PartialEq)]
    pub enum Permission {
        ReadArticle,
        CreateArticle,
    }

    fn has_permission(user: &User, permission: Permission) -> Result<(), PermissionError> {
        if user.permissions.contains(&permission) {
            Ok(())
        } else {
            Err(PermissionError::PermissionDenied(permission))
        }
    }

    #[derive(Debug, PartialEq)]
    struct Article {
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
            permissions: vec![Permission::ReadArticle],
        }
    }

    fn admin_user() -> User {
        User {
            permissions: vec![Permission::ReadArticle, Permission::CreateArticle],
        }
    }

    #[test]
    fn general_user_can_get_article_by_id() {
        assert!(get_article_by_id(&general_user(), 1).is_ok());
    }

    #[test]
    fn admin_user_can_get_article_by_id() {
        assert!(get_article_by_id(&admin_user(), 1).is_ok());
    }

    #[test]
    fn general_user_can_not_create_article() {
        assert!(create_article(&general_user(), "title".to_string(), "body".to_string()).is_err());
    }

    #[test]
    fn admin_user_can_create_article() {
        assert!(create_article(&admin_user(), "title".to_string(), "body".to_string()).is_ok());
    }
}
