use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

use crate::auth::{ApiKey, Auth};
use crate::config::AppState;
use crate::db::{self, {{table-0}}::{{Table-0-singular}}CreationError};
use crate::errors::{Errors, FieldValidator};

#[derive(Deserialize)]
pub struct New{{Table-0-singular}} {
    {{table-0-singular}}: New{{Table-0-singular}}Data,
}

#[derive(Deserialize, Validate)]
struct New{{Table-0-singular}}Data {
    #[validate(length(min = 1))]
    name: Option<String>,
}

#[post("/{{table-0}}", format = "json", data = "<new_{{table-0-singular}}>")]
pub fn post_{{table-0-singular}}(
    new_{{table-0-singular}}: Json<New{{Table-0-singular}}>,
    conn: db::Conn,
    state: State<AppState>,
) -> Result<JsonValue, Errors> {
    let new_{{table-0-singular}} = new_{{table-0-singular}}.into_inner().{{table-0-singular}};

    let mut extractor = FieldValidator::validate(&new_{{table-0-singular}});
    let name = extractor.extract("name", new_{{table-0-singular}}.name);

    extractor.check()?;

    db::{{table-0}}::create(&conn, &name)
        .map(|{{table-0-singular}}| json!({ "{{table-0-singular}}": {{table-0-singular}}.before_insert() }))
        .map_err(|error| {
            let field = match error {
                {{Table-0-singular}}CreationError::Duplicated{{Table-0-singular}}Name => "name",
            };
            Errors::new(&[(field, "has already been taken")])
        })
}

#[get("/{{table-0}}")]
pub fn get_{{table-0}}(_key: ApiKey, conn: db::Conn) -> Option<JsonValue> {
    db::{{table-0}}::find(&conn).map(|{{table-0-singular}}| json!({{table-0-singular}}))
}

#[get("/{{table-0}}/<id>")]
pub fn get_{{table-0-singular}}(_key: ApiKey, id: i32, conn: db::Conn) -> Option<JsonValue> {
    db::{{table-0}}::find_one(&conn, id).map(|{{table-0-singular}}| json!({ "{{table-0-singular}}": {{table-0-singular}} }))
}

#[derive(Deserialize)]
pub struct Update{{Table-0-singular}} {
    {{table-0-singular}}: db::{{table-0}}::Update{{Table-0-singular}}Data,
}

#[put("/{{table-0}}", format = "json", data = "<{{table-0-singular}}>")]
pub fn put_{{table-0-singular}}(
    {{table-0-singular}}: Json<Update{{Table-0-singular}}>,
    auth: Auth,
    conn: db::Conn,
    state: State<AppState>,
) -> Option<JsonValue> {
    db::{{table-0}}::update(&conn, auth.id, &{{table-0-singular}}.{{table-0-singular}})
        .map(|{{table-0-singular}}| json!({ "{{table-0-singular}}": {{table-0-singular}}.before_insert() }))
}

#[delete("/{{table-0}}/<id>")]
pub fn delete_{{table-0-singular}}(id: i32, _auth: Auth, conn: db::Conn) {
    db::{{table-0}}::delete(&conn, id);
}
