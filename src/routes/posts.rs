use actix_web::{ web, Responder, HttpResponse, HttpRequest };
use serde::Deserialize;
use crate::database::DbPool;
use crate::dto::post as PostDTO;
use crate::dto::shared::ErrorResponse;
use crate::models::post;
use crate::enums::errors::ModelError;

#[derive(Deserialize)]
struct ListOptions {
    limit: Option<i64>,
    page: Option<i64>,
}

pub async fn route_list(
    pool: web::Data<DbPool>,
    request: HttpRequest
) -> impl Responder {
    let params = web::Query::<ListOptions>::from_query(request.query_string()).unwrap();
    
    let options = post::PostListOptions {
        limit: Option::Some(params.limit.unwrap_or(5)),
        page: Option::Some(params.page.unwrap_or(1)),
    };

    let connection = pool.get();

    if connection.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: String::from("An unexpected error occurred!")
        });
    }

    match post::find_list_paginated(connection.unwrap(), options) {
        Ok(results) => HttpResponse::Ok().json(
                remap_list_of_results(results)
            ),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                message: String::from("An unexpected error occurred!")
            }),
    }
}

pub async fn route_get(
    pool: web::Data<DbPool>,
    path: web::Path<String>
) -> impl Responder {
    let post_id = path.parse::<i32>();

    if post_id.is_err() {
        return HttpResponse::BadRequest().json(
            ErrorResponse {
                message: String::from("Entered ID is invalid!")
            }
        );
    }

    let connection = pool.get();
    
    if connection.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse {
            message: String::from("An unexpected error occurred!")
        });
    }

    match post::find_by_id(connection.unwrap(), post_id.unwrap()) {
        Ok(result) => HttpResponse::Ok().json(
                PostDTO::detailed_response_factory(result.post, result.author)
            ),
        Err(ModelError::NotFound) => HttpResponse::NotFound().json(ErrorResponse {
                message: String::from("Requested post could not be found!")
            }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
                message: String::from("An unexpected error occurred!")
            })
    }
}

// ======

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .route("",      web::get().to(route_list))
            .route("/{id}", web::get().to(route_get))
    );
}

// ======

fn remap_list_of_results(results: Vec<post::PostResult>) -> Vec<PostDTO::PostDetailedResponse> {
    let mut remapped: Vec<PostDTO::PostDetailedResponse> = Vec::new();

    for item in results {
        remapped.push(
            PostDTO::detailed_response_factory(item.post, item.author)
        )
    }

    remapped
}
