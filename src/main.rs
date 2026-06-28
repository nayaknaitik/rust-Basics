use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};

use tracing::{error, info, warn};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, EnvFilter};

#[get("/")]
async fn hello() -> impl Responder {
    println!("Hello point called !!");
    info!("GET / endpoint called");
    
    HttpResponse::Ok().body("Hello world !")
}

#[get("/price/{symbol}")]
async fn get_symbol()-> impl Responder {
    HttpResponse::Ok().body("Helgog{symbol}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    info!("POST /echo endpoint called");

    HttpResponse::Ok().body(req_body)
}

async fn auto_hello() -> impl Responder {
    warn!("GET /hey endpoint called");

    HttpResponse::Ok().body("Hello from Naitik")
}
#[get("/get_g")]
async fn my_function_for_greeting() -> impl Responder{
    HttpResponse::Ok().body("Send the response on the new route")
}

#[get("/health")]
async fn get_health()-> impl Responder {
    HttpResponse::Ok().body("server is running good")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("==================================");
    info!("🚀 Actix Server Starting...");
    info!("Listening on http://127.0.0.1:8080");
    info!("==================================");

    HttpServer::new(|| {
        App::new()
            // Automatically log every request
            .wrap(TracingLogger::default())
            .service(hello)
            .service(echo)
            .service(my_function_for_greeting)
            .service(get_health)
            .route("/hey", web::get().to(auto_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
