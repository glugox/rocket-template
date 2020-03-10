use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::Serialize;

type Url = String;

#[derive(Queryable, Serialize)]
pub struct {{Table-0-singular}} {
    pub id: i32,
    pub name: String,
}




impl {{Table-0-singular}} {
  pub fn before_insert(&self) -> {{Table-0-singular}} {
       
     {{Table-0-singular}} {
          id: self.id,
          name: self.name.clone(),
     }
   }
}

