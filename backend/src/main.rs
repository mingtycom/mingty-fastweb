use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use sqlx::mysql::{ MySqlPool, MySqlPoolOptions };

#[derive(Clone)]
struct AppState {
    pool: MySqlPool
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("")
}

#[path="./api/v1/user"]
pub mod user;

/*
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DB_URL: &str = "mysql://root:mariadb@www.mingty.com:3306/mingty";
    
    let pool: MySqlPool = MySqlPoolOptions:new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .route("/get/{user_id}", web::get().to(user))
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello)) 
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}