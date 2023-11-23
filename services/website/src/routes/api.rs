use actix_web::{get, web};
use mysql::PooledConn;
use mysql::prelude::Queryable;
use base64::{Engine as _, engine::general_purpose};

use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
    pressure: f32,
    humidity: f32,
    rain_prediction: f32,
}


#[get("/latest/{hour}")]
pub(crate) async fn latest(
    hour: web::Path<u8>,
    data: web::Data<AppState>) -> String {
    let mut conn = data.db_pool.get_conn().unwrap();

    let hour = hour.into_inner() as f64;

    let latest: (f32, f32, f32) = conn.query("
select
    humid,
    press,
    temp
from (
         (select value as temp from measurements where type = 'TEMPERATURE' order by timestamp desc limit 1) as a,
         (select value as humid from measurements where type = 'HUMIDITY' order by timestamp desc limit 1) as b,
         (select value as press from measurements where type = 'PRESSURE' order by timestamp desc limit 1) as c
);",
    ).unwrap()[0];
    let inps = vec![hour, latest.0 as f64, latest.1 as f64, latest.2 as f64];
    let predict = data
        .model
        .lock()
        .unwrap()
        .predict(inps);

    let measurement = Measurement {
        temperature: latest.2,
        pressure: latest.1 / 10.,
        humidity: latest.0,
        rain_prediction: predict as f32,
    };


    serde_json::to_string(&measurement).unwrap()
}

#[derive(Serialize)]
struct History {
    pub value: f64,
    pub timestamp: u32
}

impl From<(f64, u32)> for History {
    fn from(value: (f64, u32)) -> Self {
        Self {
            value: value.0,
            timestamp: value.1,
        }
    }
}


fn get_history(mut conn: PooledConn, type_: &str, daterange: &str) -> Vec<History> {
    // parse daterange from b64
    let daterange = String::from_utf8_lossy(
        &general_purpose::STANDARD.decode(daterange).unwrap()
        ).to_string();

    // split the daterange around "-"
    let dates: Vec<&str> = daterange.split("-").collect();

    // convert them to unix timestamps
    let start = dates[0].parse::<u64>().unwrap();
    let end = dates[1].parse::<u64>().unwrap();

    println!("{} {}", start, end);
    let query = format!("SELECT value, UNIX_TIMESTAMP(timestamp) FROM \
    measurements \
    WHERE type = '{}' \
    AND timestamp between FROM_UNIXTIME({}) AND FROM_UNIXTIME({}) \
    LIMIT 500", type_, start, end);

    println!("{}", &query);

    conn.query::<(f64, u32), String>(query)
        .unwrap()
        .iter()
        .map(|r: &(f64, u32)| (*r).into())
        .collect()
}

#[get("/temperature/{daterange}")]
pub(crate) async fn temperature(
    daterange: web::Path<String>,
    data: web::Data<AppState>) -> String {
    let daterange = daterange.into_inner();

    let conn = data.db_pool.get_conn().unwrap();

    serde_json::to_string(&get_history(conn, "TEMPERATURE", &daterange)).unwrap()
}

#[get("/humidity/{daterange}")]
pub(crate) async fn humidity(
    daterange: web::Path<String>,
    data: web::Data<AppState>) -> String {
    let conn = data.db_pool.get_conn().unwrap();

    serde_json::to_string(&get_history(conn, "HUMIDITY", &daterange)).unwrap()
}

#[get("/pressure/{daterange}")]
pub(crate) async fn pressure(
    daterange: web::Path<String>,
    data: web::Data<AppState>) -> String {
    let conn = data.db_pool.get_conn().unwrap();

    serde_json::to_string(&get_history(conn, "PRESSURE", &daterange)).unwrap()
}



