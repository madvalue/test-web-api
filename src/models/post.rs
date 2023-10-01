use std::cmp;
use diesel::prelude::*; 
use diesel::result::Error::NotFound;
use crate::database::DbPooledConnection;
use crate::dto::shared::ModelResponse;
use crate::enums::errors::ModelError;
use crate::models::author::Author;
use crate::schema::authors;
use crate::schema::posts;

#[derive(Debug, Identifiable, Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author_id: Option<i32>,
}

// ======

pub struct PostListOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub struct PostResult {
    pub post: Post,
    pub author: Option<Author>,
}

// ======

pub fn find_list_paginated(mut connection: DbPooledConnection, options: PostListOptions) -> ModelResponse<Vec<PostResult>> {
    let page = cmp::max(options.page.unwrap_or(1) - 1, 0);
    let limit = options.limit.unwrap_or(5);

    let results = posts::table
        .left_join(authors::table)
        .offset(limit * page)
        .limit(limit)
        .select((Post::as_select(), Option::<Author>::as_select()))
        .load::<(Post,Option<Author>)>(&mut connection);

    if results.is_err() {
        return Err(ModelError::Unexpected);
    }

    let mut mapped_results: Vec<PostResult> = Vec::new();

    for result in results.unwrap() {
        mapped_results.push(
            PostResult {
                post: result.0,
                author: result.1
            }
        )
    }

    Ok(mapped_results)
}

pub fn find_by_id(mut connection: DbPooledConnection, id: i32) -> ModelResponse<PostResult> {
    let results = posts::table
        .left_join(authors::table)
        .filter(posts::id.eq(id))
        .select((Post::as_select(), Option::<Author>::as_select()))
        .get_result::<(Post,Option<Author>)>(&mut connection);

    match results {
        Ok(post)        => Ok(PostResult { post: post.0, author: post.1 }),
        Err(NotFound)   => Err(ModelError::NotFound),
        Err(_)          => Err(ModelError::Unexpected),
    }
}