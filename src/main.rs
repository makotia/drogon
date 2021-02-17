use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use serde::{ Deserialize, Serialize };
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
struct DrogonObj {
    text: String,
}

async fn index() -> impl Responder {
    let mut gen_dragon_vec = vec![
        "ド",
        "ラ",
        "ゴ",
        "ン",
        "ボ",
    ];
    let mut dragon_vec = vec![];
    for _ in 0..4 {
        let mut rng = rand::thread_rng();
        gen_dragon_vec.shuffle(&mut rng);
        dragon_vec.push(gen_dragon_vec[0]);
    }
    HttpResponse::Ok().json(DrogonObj {
        text: dragon_vec.join("").to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
