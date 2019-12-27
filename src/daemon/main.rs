extern crate daemonize;
extern crate ansi_term;
extern crate clap;
extern crate dirs;

mod sighting_writer;
mod sighting_reader;
mod sighting_configure;
mod attribute;
mod db;

use clap::Arg;
use std::sync::Arc;
use std::sync::Mutex;

use daemonize::Daemonize;
use ansi_term::Color::Red;
use ini::Ini;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use serde::{Deserialize, Serialize};

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

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
    let sharedstate = &mut *data.lock().unwrap();
    
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
    let sharedstate = &mut *data.lock().unwrap();
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


#[derive(Serialize,Deserialize)]
pub struct PostData {
    items: Vec<BulkSighting>
}

#[derive(Serialize,Deserialize)]
pub struct BulkSighting {
    namespace: String,
    value: String,
}

fn read_bulk(data: web::Data<Arc<Mutex<SharedState>>>, postdata: web::Json<PostData>, _req: HttpRequest) -> impl Responder {
    let sharedstate = &mut *data.lock().unwrap();

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
    let sharedstate = &mut *data.lock().unwrap();
    let mut could_write = false;

    for i in &postdata.items {
        could_write = sighting_writer::write(&mut sharedstate.db, i.namespace.as_str(), i.value.as_str());
    }

    if could_write {
        return HttpResponse::Ok().json(Message{message: String::from("ok")});
    }
    return HttpResponse::Ok().json(Message{message: String::from("Invalid base64 encoding (base64 url with non padding) value")});
}


fn create_home_config() {
    let mut home_config = PathBuf::from(dirs::home_dir().unwrap());
    home_config.push(".sightingdb");
    fs::create_dir_all(home_config);
}

fn sightingdb_get_config() -> Result<String, &'static str> {
    let ini_file = PathBuf::from("/etc/sightingdb/sighting-daemon.ini");
    let mut home_ini_file = PathBuf::from(dirs::home_dir().unwrap());
    
    let can_open = Path::new(&ini_file).exists();
    if can_open {
        return Ok(String::from(ini_file.to_str().unwrap()));
    }
    
    home_ini_file.push(".sightingdb");
    home_ini_file.push("sighting-daemon.ini");

    let can_open = Path::new(&home_ini_file).exists();
    if can_open {
        return Ok(String::from(home_ini_file.to_str().unwrap()));
    }
    
    return Err("Cannot locate sighting-daemon.ini in neither from the -c flag, /etc/sightingdb or ~/.sightingdb/");
}

fn sightingdb_get_pid() -> String {
    let can_create_file = File::create("/var/run/sightingdb.pid");
    match can_create_file {
        Ok(_) => {
            return String::from("/var/run/sightingdb.pid");
        },
        Err(..) => {
            let mut home_pid = PathBuf::from(dirs::home_dir().unwrap());
            home_pid.push(".sightingdb");
            home_pid.push("sighting-daemon.pid");
            let pid_file = home_pid.to_str().unwrap();
            let can_create_home_pid_file = File::create(pid_file);
            match can_create_home_pid_file {
                Ok(_) => { return String::from(pid_file); },
                Err(..) => {
                    println!("Cannot write pid to /var/run not ~/.sightingdb/, using current dir: sightingdb.pid");
                    return String::from("./sightingdb.pid");
                }
            }
        }
    }    
    return String::from("./sightingdb.pid");
}

fn main() {
    create_home_config();
    
    let sharedstate = Arc::new(Mutex::new(SharedState::new()));

    let matches = clap::App::new("SightingDB")
                          .version("0.2")
                          .author("Sebastien Tricaud <sebastien.tricaud@devo.com>")
                          .about("Sightings Database")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    let configarg = matches.value_of("config");
    let mut configstr = String::from("");
    match configarg {
        Some(_configstr) => { configstr = _configstr.to_string(); },
        None => {
            let sightingdb_ini_file = sightingdb_get_config().unwrap();
            configstr = sightingdb_ini_file;
        }
    }

    println!("Using configuration file: {}", configstr);
    let configpath = Path::new(&configstr);
    let config = Ini::load_from_file(&configstr).unwrap();
    println!("Config path:{}", configpath.parent().unwrap().display());
    
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

    let mut ssl_cert: PathBuf;
    let ssl_cert_config = daemon_config.get("ssl_cert").unwrap();
    if ssl_cert_config.starts_with("/") {
        ssl_cert = PathBuf::from(ssl_cert_config);
    } else {
        ssl_cert = PathBuf::from(configpath.parent().unwrap());
        ssl_cert.push(&ssl_cert_config);
    }

    let mut ssl_key: PathBuf;
    let ssl_key_config = daemon_config.get("ssl_key").unwrap();
    if ssl_key_config.starts_with("/") {
        ssl_key = PathBuf::from(ssl_key_config);
    } else {
        ssl_key = PathBuf::from(configpath.parent().unwrap());
        ssl_key.push(&ssl_key_config);
    }
    
    match daemon_config.get("daemonize").unwrap().as_ref() {
        "true" => {
            let stdout = File::create("/tmp/daemon.out").unwrap();
            let stderr = File::create("/tmp/daemon.err").unwrap();

            let pid_file = sightingdb_get_pid();
            Daemonize::new().pid_file(pid_file).stdout(stdout).stderr(stderr).start();
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
        builder.set_certificate_chain_file(ssl_cert.to_str().unwrap()).unwrap();

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
