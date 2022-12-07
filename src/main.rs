//==================================================================================================
//
//
//
//
//
//==================================================================================================
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use actix_files::NamedFile;
use actix_web_actors::ws;

mod server; // Import the server module
use self::server::MyWebSocket;

// load TLS keys
// to create a self-signed temporary cert for testing:
// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
fn enable_tls() -> SslAcceptorBuilder {

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("./ssl/key.pem", SslFiletype::PEM).unwrap();

    builder.set_certificate_chain_file("./ssl/cert.pem").unwrap();

    return builder;
}

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

// WebSocket handshake and start `MyWebSocket` actor.
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
        .service(web::resource("/").to(index))
        .service(web::resource("/ws").route(web::get().to(websocket)))// Add the WebSocket route
        .wrap(middleware::Logger::default())
    })
    .workers(3)
    .bind(("0.0.0.0:8080", enable_tls()))?
    .run()
    .await
}