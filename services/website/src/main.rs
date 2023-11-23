use std::sync::{Arc, Mutex};
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use mysql::Pool;
use crate::ai::main::{AIModel, load_model};

mod routes;
mod ai;

struct AppState {
    db_pool: Pool,
    model: Arc<Mutex<AIModel>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // load the model. if it is found in the filesystem, load it. otherwise, train it and save it.
    let model = match load_model("./src/ai/model.bin", "smartcore") {
        Ok(model) => Arc::new(Mutex::new(model)),
        Err(_) => {
            let model = ai::main::train().unwrap();
            ai::main::save_model(&model, "./src/ai/model.bin").unwrap();
            Arc::new(Mutex::new(model))
        }
    };

    // ai::main::train().unwrap();
    // let model = Arc::new(Mutex::new(ai::main::train().unwrap()));

    HttpServer::new(move || {
        // create app state
        let url= std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let db_pool = Pool::new(url.as_str()).expect("db pool");


        let app_state = AppState {
            db_pool,
            model: model.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .service(web::scope("/api")
                .service(routes::api::latest)
                .service(routes::api::temperature)
                .service(routes::api::pressure)
                .service(routes::api::humidity)
            )
            .service(web::scope("/ai")
                .service(routes::ai::train)
            )
            .service(routes::client::index)
            .service(routes::client::history)
            .service(routes::client::static_files)
            .service(routes::client::config)
    })

        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
