use crate::db::Conn;
use crate::models::{{table-0-singular}}::{{Table-0-singular}};
use crate::schema::{{table-0}};
use std::ops::Deref;

use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "{{table-0}}"]
pub struct New{{Table-0-singular}}<'a> {
    pub name: &'a str,
}

pub enum {{Table-0-singular}}CreationError {
    Duplicated{{Table-0-singular}}Name,
}

impl From<Error> for {{Table-0-singular}}CreationError {
    fn from(err: Error) -> {{Table-0-singular}}CreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("{{table-0-singular}}_name_key") => return {{Table-0-singular}}CreationError::Duplicated{{Table-0-singular}}Name,
                _ => {}
            }
        }
        panic!("Error creating {{table-0-singular}}: {:?}", err)
    }
}

pub fn create(
    conn: &Conn,
    name: &str,
) -> Result<{{Table-0-singular}}, {{Table-0-singular}}CreationError> {

    let new_{{table-0-singular}} = &New{{Table-0-singular}} {
        name,
    };

    diesel::insert_into({{table-0}}::table)
        .values(new_{{table-0-singular}})
        .get_result::<{{Table-0-singular}}>(conn.deref())
        .map_err(Into::into)
}


/// Return a list of all {{Table-0}}
/// TODO: Pagination
pub fn find(conn: &Conn) -> Option<{{Table-0-singular}}List> {

    let {{table-0}} : Vec<{{Table-0-singular}}> = {{table-0}}::table.load::<{{Table-0-singular}}>(conn.deref())
        .map_err(|err| println!("Can not load users!: {}", err))
        .unwrap();

    Some(UserList{
        {{table-0}}
    })
}


pub fn find_one(conn: &Conn, id: i32) -> Option<{{Table-0-singular}}> {
    {{table-0}}::table
        .find(id)
        .get_result(conn.deref())
        .map_err(|err| println!("find_{{table-0-singular}}: {}", err))
        .ok()
}

pub fn delete(conn: &Conn, id: i32) {
    let result = diesel::delete({{table-0}}::table.filter({{table-0}}::id.eq(id))).execute(conn.deref());
    if let Err(err) = result {
        eprintln!("{{table-0}}::delete: {}", err);
    }
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "{{table-0}}"]
pub struct Update{{Table-0-singular}}Data {
    name: Option<String>,
}

pub fn update(conn: &Conn, id: i32, data: &Update{{Table-0-singular}}Data) -> Option<{{Table-0-singular}}> {
    let data = &Update{{Table-0-singular}}Data {
        // Place to set particular fields... ex password: None,
        ..data.clone()
    };
    diesel::update({{table-0}}::table.find(id))
        .set(data)
        .get_result(conn.deref())
        .ok()
}
