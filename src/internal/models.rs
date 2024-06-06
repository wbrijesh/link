use crate::schema::links_backup;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct ShortUrl {
    pub id: i32,
    pub url: String,
    pub slug: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "links_backup"]
pub struct NewShortUrl {
    pub url: String,
    pub slug: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "links_backup"]
pub struct UpdateShortUrl {
    pub url: Option<String>,
    pub slug: Option<String>,
}
