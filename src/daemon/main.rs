extern crate rouille;
extern crate daemonize;
extern crate ansi_term;

use rouille::Request;
use rouille::Response;
use daemonize::Daemonize;
use ansi_term::Color::Red;
use ini::Ini;


fn main() {

    let config = Ini::load_from_file("sighting-daemon.ini").unwrap();

    let daemon_config = config.section(Some("daemon")).unwrap();
    match daemon_config.get("daemonize").unwrap().as_ref() {
        "true" => { Daemonize::new().pid_file("/var/run/sightingdb.pid").start().unwrap(); },
        "false" => println!("nope"),
        _ => println!("unknown"),
    }

    let listen_ip = daemon_config.get("listen_ip").unwrap();
    let listen_port = daemon_config.get("listen_port").unwrap();

    let server_address = format!("{}:{}", listen_ip, listen_port);
    
    let welcome_string = Red.paint("Starting Sighting Daemon").to_string();
    println!("{}", welcome_string);
    rouille::start_server(server_address, move |request| {
        Response::text("hello world")
    });    


}
