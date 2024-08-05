/* 
This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>. 
 */


use actix_web::{
    http::header::ContentEncoding,
    get, App, HttpResponse,
    HttpServer, Responder,
    middleware, web::Data
};
use std::fs;
use clap::{arg, command, value_parser};
use actix_web::middleware::Logger;
use env_logger::Env;



// This struct represents state
struct AppState {
    test_file_path: String
}

#[get("/download-blob-non-compressed")]
async fn download_blob_non_compressed(data: Data<AppState>) -> impl Responder {

    let contents = fs::read_to_string(&data.test_file_path);
    match contents {
        Ok(s) =>  { 
                HttpResponse::Ok()
                .insert_header(ContentEncoding::Identity)
                .body(s) 
        },
        Err(e) => { HttpResponse::NotFound().body(e.to_string()) }
    }
}

#[get("/download-blob")]
async fn download_blob(data: Data<AppState>) -> impl Responder {
    let contents = fs::read_to_string(&data.test_file_path);
    match contents {
        Ok(s) =>  { 
            HttpResponse::Ok()
                .insert_header(ContentEncoding::Identity)
                .body(s) 
        }
        Err(e) => { HttpResponse::NotFound().body(e.to_string()) }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let matches = command!()
    .arg(
        arg!(
            -f --filepath <FILE> "Sets path to test file"
        )
        .required(true)
        .value_parser(value_parser!(String)),
    )
    // .arg(arg!(
    //     -d --debug ... "Turn debugging information on"
    // ))
    .get_matches();
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let app_state = AppState {
            test_file_path: matches.get_one::<String>("filepath").expect("filepath is required").to_string()
        };
        App::new()
            .app_data(Data::new(app_state))
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .service(download_blob)
            .service(download_blob_non_compressed)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}