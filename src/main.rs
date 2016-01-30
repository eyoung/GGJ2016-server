mod voodoo;
extern crate hyper;

use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};
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
    fn handle(&self, request: Request, response: Response) {
        response.send("It's alive".as_bytes()).unwrap();
    }
}