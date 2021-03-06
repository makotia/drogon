use env_logger;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder, middleware::Logger };
use serde::{ Deserialize, Serialize };
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
struct DrogonObj {
    text: String,
}

async fn index() -> impl Responder {
    let mut gen_dragon_arr = [
        "ド",
        "ラ",
        "ゴ",
        "ン",
        "ボ",
    ];
    let mut dragon_vec = vec![];
    for _ in 0..4 {
        let mut rng = rand::thread_rng();
        gen_dragon_arr.shuffle(&mut rng);
        dragon_vec.push(gen_dragon_arr[0]);
    }
    HttpResponse::Ok().json(DrogonObj {
        text: dragon_vec.join("").to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
