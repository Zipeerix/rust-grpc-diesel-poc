use crate::account_service::proto;
use crate::database::Validatable;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub country: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub name: &'a str,
    pub surname: &'a str,
    pub country: &'a str,
}

impl Validatable for NewUser<'_> {
    fn validate(&self) -> Result<(), String> {
        if self.email.is_empty()
            || self.password.is_empty()
            || self.name.is_empty()
            || self.surname.is_empty()
            || self.country.is_empty()
        {
            return Err("All fields must be not empty".to_string());
        }

        if !self.email.contains('@') {
            return Err("Invalid e-mail format".to_string());
        }

        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }

        Ok(())
    }
}

impl<'a> From<&'a proto::User> for NewUser<'a> {
    fn from(proto_user: &'a proto::User) -> NewUser<'a> {
        NewUser {
            email: &proto_user.email,
            password: &proto_user.password,
            name: &proto_user.name,
            surname: &proto_user.surname,
            country: &proto_user.country,
        }
    }
}
