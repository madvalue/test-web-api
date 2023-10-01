use serde::Serialize;

use crate::dto::author as AuthorDTO;
use crate::models::{author::Author, post::Post};

#[derive(Serialize)]
pub struct PostResponse {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct PostDetailedResponse {
    pub title: String,
    pub body: String,
    pub author: Option<AuthorDTO::AuthorResponse>,
}

// ======

pub fn detailed_response_factory(post: Post, author: Option<Author>) -> PostDetailedResponse {
    let mut author_response: Option<AuthorDTO::AuthorResponse> = Option::None;
    if author.is_some() {
        author_response = Some(AuthorDTO::response_factory(author.unwrap()));
    }

    PostDetailedResponse {
        title: post.title,
        body: post.body,
        author: author_response
    }
}