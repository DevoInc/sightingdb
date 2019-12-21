extern crate daemonize;
extern crate ansi_term;


mod sighting_writer;
mod sighting_reader;
mod sighting_configure;
mod attribute;
mod db;

use std::sync::Arc;
use std::sync::Mutex;

use daemonize::Daemonize;
use ansi_term::Color::Red;
use ini::Ini;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::attribute::Attribute;

use serde::{Deserialize, Serialize};

use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

pub struct SharedState {
    pub db: db::Database,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            db: db::Database::new(),
        }
    }
}

#[derive(Serialize)]
pub struct Message {
    message: String
}

#[derive(Serialize)]
pub struct InfoData {
    implementation: String,
    version: String,
    vendor: String,
    author: String
}

#[derive(Serialize,Deserialize)]
pub struct PostData {
    items: Vec<BulkSighting>
}

#[derive(Serialize,Deserialize)]
pub struct BulkSighting {
    namespace: String,
    value: String,
}

fn help(_req: HttpRequest) -> impl Responder {
    "Sighting Daemon, written by Sebastien Tricaud, (C) Devo Inc. 2019
REST Endpoints:
\t/w: write
\t/r: read
\t/c: configure
\t/i: info
"
}

fn read(data: web::Data<Arc<Mutex<SharedState>>>, _req: HttpRequest) -> impl Responder {
    let mut sharedstate = &mut *data.lock().unwrap();
    
    let (_, path) = _req.path().split_at(3);
    if _req.query_string().starts_with("val=") {
        let (_, val) = _req.query_string().split_at(4);
        let ans = sighting_reader::read(&mut sharedstate.db, path, val);
        return HttpResponse::Ok().body(ans);
    }

    return HttpResponse::Ok().json(Message{message: String::from("Error: val= not found!")})
}

// fn write(db: web::Data<Mutex<db::Database>>, _req: HttpRequest) -> impl Responder {
fn write(data: web::Data<Arc<Mutex<SharedState>>>, _req: HttpRequest) -> HttpResponse {
    let mut sharedstate = &mut *data.lock().unwrap();
    let mut could_write = false;

    // println!("{:?}", _req.path());
    
    let (_, path) = _req.path().split_at(3);
    if _req.query_string().starts_with("val=") {
        let (_, val) = _req.query_string().split_at(4);
        could_write = sighting_writer::write(&mut sharedstate.db, path, val);
    }

    if could_write {
        return HttpResponse::Ok().json(Message{message: String::from("ok")});
    }
    return HttpResponse::Ok().json(Message{message: String::from("Invalid base64 encoding (base64 url with non padding) value")});
    
}
fn configure(_req: HttpRequest) -> impl Responder {
    "configure"
}


fn read_bulk(data: web::Data<Arc<Mutex<SharedState>>>, postdata: web::Json<PostData>, _req: HttpRequest) -> impl Responder {
    let mut sharedstate = &mut *data.lock().unwrap();

    let mut json_response = String::from("{\n\t\"items\": [\n");

    for v in &postdata.items {
        let ans = sighting_reader::read(&mut sharedstate.db, v.namespace.as_str(), v.value.as_str());

        json_response.push_str("\t\t");
        json_response.push_str(&ans);
        json_response.push_str(",\n");                
    }
    json_response.pop();
    json_response.pop(); // We don't need the last ,
    json_response.push_str("\n"); // however we need the line return :)
    
    json_response.push_str("\t]\n}\n");
    return HttpResponse::Ok().body(json_response);        
}

fn write_bulk(data: web::Data<Arc<Mutex<SharedState>>>, postdata: web::Json<PostData>, _req: HttpRequest) -> impl Responder {
    let mut sharedstate = &mut *data.lock().unwrap();
    let mut could_write = false;

    for i in &postdata.items {
        could_write = sighting_writer::write(&mut sharedstate.db, i.namespace.as_str(), i.value.as_str());
    }

    if could_write {
        return HttpResponse::Ok().json(Message{message: String::from("ok")});
    }
    return HttpResponse::Ok().json(Message{message: String::from("Invalid base64 encoding (base64 url with non padding) value")});
}

fn main() {

    let mut sharedstate = Arc::new(Mutex::new(SharedState::new()));

    let config = Ini::load_from_file("sighting-daemon.ini").unwrap();

    let daemon_config = config.section(Some("daemon")).unwrap();

    let listen_ip = daemon_config.get("listen_ip").unwrap();
    let listen_port = daemon_config.get("listen_port").unwrap();

    let server_address = format!("{}:{}", listen_ip, listen_port);
    
    let welcome_string = Red.paint("Starting Sighting Daemon").to_string();
    println!("{}", welcome_string);

    let use_ssl;
    match daemon_config.get("ssl").unwrap().as_ref() {
        "false" => use_ssl = false,
        _ => use_ssl = true, // no mistake, only false can start the unsecure server.
    }
    
    let ssl_cert = daemon_config.get("ssl_cert").unwrap();
    let ssl_key = daemon_config.get("ssl_key").unwrap();

    let mut pid_file = "./sightingdb.ini";
    match daemon_config.get("daemonize").unwrap().as_ref() {
        "true" => {
            let stdout = File::create("/tmp/daemon.out").unwrap();
            let stderr = File::create("/tmp/daemon.err").unwrap();

            // Where can we write the pid?
            let can_create_file = File::create("/var/run/sightingdb.pid");
            match can_create_file {
                Ok(_) => {
                    pid_file = "/var/run/sightingdb.pid";
                    Daemonize::new().pid_file(pid_file).stdout(stdout).stderr(stderr).start();
                },
                Err(..) => {
                    let mut sightingdb_home = env::home_dir().unwrap();
                    sightingdb_home.push(".sightingdb");
                    let mut sightingdb_home_pid = Path::new(&sightingdb_home).to_path_buf();
                    sightingdb_home_pid.push("sightingdb.pid");
                    pid_file = sightingdb_home_pid.to_str().unwrap();
                    let can_create_home_pid_file = File::create(pid_file);
                    match can_create_home_pid_file {
                        Ok(_) => {},
                        Err(..) => {
                            println!("Cannot write pid to /var/run not ~/.sightingdb/, using current dir: sightingdb.pid");
                            pid_file = "./sightingdb.pid";                            
                        }
                    }
                    Daemonize::new().pid_file(pid_file).start();
//                    println!("We write the pid there: {:?}", pid_file);
                }
            }

            
        },
        "false" => println!("This daemon is not daemonized. To run in background, set 'daemonize = true' in sigthing-daemon.ini"),
        _ => println!("Unknown daemon setting. Starting in foreground."),
    }
    
    if use_ssl {
        let mut builder =
            SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(ssl_key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(ssl_cert).unwrap();

        // routes:
        // w -> write
        // r -> read
        // c -> config (push all to disk, alway in memory, both)
        // i -> info
        // untyped -> things that have an incorrect type match
        HttpServer::new(move || { App::new().data(sharedstate.clone())
                        .route("/r/*", web::get().to(read))
                        .route("/rb", web::post().to(read_bulk))
                        .route("/w/*", web::get().to(write))
                        .route("/wb", web::post().to(write_bulk))
                        .route("/c/*", web::get().to(configure))
                        .route("/i", web::get().to(info))
                        .default_service(web::to(help))
        })
            .bind_ssl(server_address, builder)
            .unwrap()
            .run()
            .unwrap();        
    }
    
}

fn info(_req: HttpRequest) -> impl Responder {
    let infoData = InfoData {
        implementation: String::from("SightingDB"),
        version: String::from("0.0.1"),
        vendor: String::from("Devo"),
        author: String::from("Sebastien Tricaud")
    };
    return HttpResponse::Ok().json(&infoData);
}
