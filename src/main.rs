// TODO: executable start [[config.toml]]; executable key [[custom key]]

use awc;
use actix_web::{get, App, web, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::fs;
use toml::Table;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Clone,Debug)]
struct Config {
    token: String,
	channel_id: String,
	port: u16
}

#[derive(Deserialize,Serialize)]
struct Info {
    title: String,
	content: String
}

#[get("/")]
async fn post(config: web::Data<Config>,info: web::Query<Info>) -> impl Responder {
	let client = awc::Client::default();
	let data = json!({
        "title": info.title,
        "content": info.content
    });
	let req = client.post(format!("https://www.guilded.gg/api/v1/channels/{}/announcements",config.channel_id))
		.insert_header(("Authorization", format!("Bearer {}",config.token)))
		.insert_header(("Accept", "application/json"))
		.insert_header(("Content-type","application/json"));

	let res = req.send_json(&data).await;
	let body = res.unwrap().body().await;
	HttpResponse::Ok().body(body.unwrap())
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let map = &config_str.parse::<Table>().unwrap();
	let config = Config{
		token: map["config"]["token"].as_str().expect("Failed to parse config.token").to_string(),
		channel_id: map["config"]["channel_id"].as_str().expect("Failed to parse config.channel_id").to_string(),
		port: map["config"]["port"].as_integer().expect("Failed to parse config.port") as u16
	};
    Ok(config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let config = load_config().expect("Failed to load config");
	let port = config.port;
    HttpServer::new(move || {
        App::new()
			.app_data(web::Data::new(config.clone()))
            .service(post)
    })
    .bind(("localhost", port))?
    .run()
    .await
}
