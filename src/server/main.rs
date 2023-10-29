use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[allow(dead_code)]
enum RequestAction {
    GET,
    POST,
}
#[allow(dead_code)]
struct WebView {
    action: RequestAction,
    path: String,
    body: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let mut app = App::new();
        // TODO: load urls from the config file
        //
        // [ ] - Add support for config file
        // [ ] - serialize the url from the config file
        let urls = vec![
            WebView {
                action: RequestAction::GET,
                path: "/".to_owned(),
                body: "Welcome".to_owned(),
            },
            WebView {
                action: RequestAction::GET,
                path: "/hello".to_owned(),
                body: "Hello There".to_owned(),
            },
        ];
        for url in urls.iter() {
            app = app.service(web::resource(url.path.as_str()).route(match url.action {
                RequestAction::GET => web::get().to(HttpResponse::Ok),
                RequestAction::POST => web::post().to(HttpResponse::Ok),
            }))
        }
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
