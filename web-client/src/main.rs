use actix_web::{client::Client, web, App, HttpResponse, HttpServer, Error};
use futures::Future;

fn index() -> impl Future<Item = HttpResponse, Error = Error> {
    let client = Client::default();
    client.get("https://linuxfr.org")
        .send()
        .map_err(Error::from)
        .and_then(|mut resp| {
            resp.body()
                .from_err()
                .and_then(|body| {
                    Ok(HttpResponse::Ok().body(body))
                })
        })
}

fn main() {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to_async(index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
