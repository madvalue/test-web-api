use diesel::prelude::*;
use crate::schema::authors;

#[derive(Debug, Identifiable, Queryable, Selectable, PartialEq)]
#[diesel(belongs_to(crate::models::post::Post))]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Author {
    pub id: i32,
    pub nickname: String,
}