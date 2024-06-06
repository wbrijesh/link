use diesel::{prelude::*, r2d2};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

//pub struct ShortUrl {
//    id: i32,
//    url: String,
//    slug: String,
//}
