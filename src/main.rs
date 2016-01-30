mod voodoo;
extern crate hyper;

use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::io::Read;
use hyper::server::{Handler, Server, Request, Response};
use voodoo::{GameManager, VoodooMessage};

fn main() {
    let manager_sender = GameManager::run();
    Server::http("0.0.0.0:9282").unwrap().handle(NetworkHandler::new(manager_sender)).unwrap();
}

struct NetworkHandler {
    manager_sender: Mutex<Sender<VoodooMessage>>
}

impl NetworkHandler {
    fn new(sender: Sender<VoodooMessage>) -> NetworkHandler {
        NetworkHandler {
            manager_sender: Mutex::new(sender)
        }
    }
}

impl Handler for NetworkHandler {
    fn handle(&self, mut request: Request, response: Response) {
        let mut request_buffer = Vec::new();
        let request_body = request.read_to_end(&mut request_buffer);
        if let Ok(body_content) = String::from_utf8(request_buffer) {
            println!("Requested with body {}", &body_content);
        }
        response.send(format!("{}\n", "it's alive").as_bytes()).unwrap();
    }
}