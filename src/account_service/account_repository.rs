use crate::account_service::models::{NewUser, User};
use crate::database::{PooledDbConnection, Validatable};
use crate::schema::users;
use crate::schema::users::dsl;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use log::info;
use sha2::{Digest, Sha256};
use std::fmt::Formatter;
use std::fmt::Write;
use std::{error, fmt};

#[derive(Debug)]
pub enum AccountRepositoryError {
    DatabaseError(String),
    EntryValidationError(String),
    LoginValidationError(String),
}

impl fmt::Display for AccountRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AccountRepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AccountRepositoryError::EntryValidationError(e) => write!(f, "Validation error: {}", e),
            AccountRepositoryError::LoginValidationError(e) => write!(f, "Login error: {}", e),
        }
    }
}

impl error::Error for AccountRepositoryError {}

pub fn create_new_user(
    new_user: NewUser,
    db_connection: &mut PooledDbConnection,
) -> Result<i32, AccountRepositoryError> {
    new_user
        .validate()
        .map_err(AccountRepositoryError::EntryValidationError)?;

    let found_user = dsl::users
        .filter(users::email.eq(&new_user.email))
        .select(User::as_select())
        .first(db_connection)
        .optional()
        .map_err(|e| AccountRepositoryError::DatabaseError(e.to_string()))?;

    if found_user.is_some() {
        return Err(AccountRepositoryError::EntryValidationError(
            "User with this email already exists".to_string(),
        ));
    }

    let hashed_password = hash_password(new_user.password);

    let mut insertable_new_user = new_user;
    insertable_new_user.password = &hashed_password;

    let created_user = diesel::insert_into(dsl::users)
        .values(&insertable_new_user)
        .returning(User::as_returning())
        .get_result(db_connection)
        .map_err(|e| AccountRepositoryError::DatabaseError(e.to_string()))?;

    info!(
        "Created user with user id: '{}' and email '{}'",
        created_user.id, created_user.email
    );

    Ok(created_user.id)
}

pub fn delete_user(
    user_id: i32,
    db_connection: &mut PooledDbConnection,
) -> Result<(), AccountRepositoryError> {
    let found_user = dsl::users
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first(db_connection)
        .optional()
        .map_err(|e| AccountRepositoryError::DatabaseError(e.to_string()))?;

    if found_user.is_none() {
        return Err(AccountRepositoryError::EntryValidationError(
            "User with this id does not exist".to_string(),
        ));
    }

    diesel::delete(dsl::users.filter(users::id.eq(user_id)))
        .execute(db_connection)
        .map_err(|e| AccountRepositoryError::DatabaseError(e.to_string()))?;

    info!("Deleted user with user id: '{}'", user_id);

    Ok(())
}

pub fn validate_login(
    email: &str,
    password: &str,
    db_connection: &mut PooledDbConnection,
) -> Result<(), AccountRepositoryError> {
    let found_user = dsl::users
        .filter(users::email.eq(email))
        .select(User::as_select())
        .first(db_connection)
        .optional()
        .map_err(|e| AccountRepositoryError::DatabaseError(e.to_string()))?
        .ok_or_else(|| {
            AccountRepositoryError::LoginValidationError("User not found".to_string())
        })?;

    if hash_password(password) != found_user.password {
        return Err(AccountRepositoryError::LoginValidationError(
            "User not found".to_string(),
        ));
    }

    Ok(())
}

fn hash_password(plaintext: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plaintext);

    hasher
        .finalize()
        .iter()
        .fold(String::new(), |mut acc, byte| {
            let _ = write!(&mut acc, "{:02x}", byte);
            acc
        })
}
