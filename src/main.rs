#[macro_use]
extern crate diesel;

extern crate dotenv;

mod db;
use db::*;

#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
     let posts = get_posts();
     HttpResponse::Ok().json(posts)
}

#[derive(Debug, Serialize, Deserialize)]
struct CreatePost {
     title: String,
     body: String,
}

fn create(post: web::Json<CreatePost>, req: HttpRequest) -> impl Responder {
     println!("request: {:?}", req);
     println!("model: {:?}", post);

     let result = create_post(post.0.title.as_ref(), post.0.body.as_ref());

     HttpResponse::Ok().json(result)
}

fn publish(path: web::Path<String>) -> impl Responder {
     let result = publish_post(path.to_string());

     HttpResponse::Ok().json(result)
}

fn main() {
     HttpServer::new(|| {
          App::new()
               .data(web::JsonConfig::default().limit(4096))
               .route("/", web::get().to(index))
               .route("/create", web::post().to(create))
               .route("/publish/{id}", web::put().to(publish))
     })
     .bind("127.0.0.1:8088")
     .unwrap()
     .run()
     .unwrap();
}
