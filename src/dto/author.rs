use serde::Serialize;
use crate::models::author::Author;

#[derive(Serialize)]
pub struct AuthorResponse {
    pub id: i32,
    pub nickname: String,
}

// ======

pub fn response_factory(author: Author) -> AuthorResponse {
    AuthorResponse { 
        id: author.id, 
        nickname: author.nickname
    }
}