///
/// You can use before_callback and after_callback like design-by-contracts.
/// By matching the return value of the main function and the callback function, you can return any error.
///
/// We assume that each function has the following specifications.
///
/// - has_permission() checks if the user has the required Permission and returns PermissionError if the user does not have it.
/// - A user with Permission::Read can execute read().
/// - A user with Permission::Create can execute create().
/// - If a user without Permission tries to execute read() or create(), UseCaseError is returned.
///
/// In this case, we will check if the user has the required Permission in before_callback.
/// has_permission() returns PermissionError, so we convert it to UseCaseError and return it.
///
use callback_rs::before_callback;

#[before_callback(has_permission(user, Permission::Read).map_err(UseCaseError::from)?)]
fn read(user: &User) -> Result<(), UseCaseError> {
    Ok(())
}

#[before_callback(has_permission(user, Permission::Create).map_err(UseCaseError::from)?)]
fn create(user: &User) -> Result<(), UseCaseError> {
    Ok(())
}

fn main() {
    let general_user = User {
        permissions: vec![Permission::Read],
    };
    let admin_user = User {
        permissions: vec![Permission::Read, Permission::Create],
    };

    // general_user has read permission. So, it should be Ok.
    assert!(read(&general_user).is_ok());
    // admin_user has read permission. So, it should be Ok
    assert!(read(&admin_user).is_ok());

    // general_user does not have permission to create. So, it should be Err.
    assert!(create(&general_user).is_err());
    // admin_user has permission to create. So, it should be Ok.
    assert!(create(&admin_user).is_ok());
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
