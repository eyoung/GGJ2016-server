use std::sync::mpsc::{Sender, channel};
use std::thread;
use rustc_serialize::json;
use voodoo::Scene;
use voodoo::VoodooError;

pub struct GameManager {
    current_scene: Scene,
    client_queue: Vec<Sender<String>>,
    num_clients: usize
}

impl GameManager {
    fn new() -> GameManager {
        GameManager {
            current_scene: Scene::new(),
            client_queue: Vec::new(),
            num_clients: 2
        }
    }
}

impl GameManager {
    pub fn run() -> Sender<VoodooMessage> {
        let (spawn_sender, spawn_receiver) = channel();
        let spawn_sender = spawn_sender.clone();
        thread::spawn(move || {
            let (event_sender, event_receiver) = channel();
            if let Ok(_) = spawn_sender.send(event_sender.clone()) {
                let mut manager = GameManager::new();
                while let Ok(message) = event_receiver.recv() {
                    match message {
                        VoodooMessage::Magic(region, value, responce_channel) => {
                            match region {
                                Region::Head => {
                                    manager.current_scene.head += value;
                                }
                                Region::Body => {
                                    manager.current_scene.body += value;
                                }
                                Region::Arms => {
                                    manager.current_scene.arms += value;
                                }
                                Region::Legs => {
                                    manager.current_scene.legs += value;
                                }
                            }
                            
                            manager.client_queue.push(responce_channel);
                            if manager.client_queue.len() == manager.num_clients {
                                let response = VoodooResponse::new(&manager.current_scene);
                                let body_content = json::encode(&response).unwrap();
                                for client in &manager.client_queue {
                                    client.send(body_content.to_string()).unwrap();
                                }
                                manager.current_scene.next();
                            }
                        }

                        VoodooMessage::TurnAction(action) => {

                        }
                    }
                }
            }
        });
        spawn_receiver.recv().unwrap()
    }
}

pub enum VoodooMessage {
    Magic(Region, isize, Sender<String>),
    TurnAction(ActionContent, Sender<String>)
}

pub enum Region {
    Head,
    Arms,
    Body,
    Legs
}

impl Region {
    pub fn new(text: &str) -> Result<Region, VoodooError> {
        match text {
            "head" => Ok(Region::Head),
            "arms" => Ok(Region::Arms),
            "body" => Ok(Region::Body),
            "legs" => Ok(Region::Legs),
            _ => Err(VoodooError::InvalidRegionError)
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct VoodooResponse {
    next_level: isize,
    arm_score: isize,
    head_score: isize,
    legs_score: isize,
    body_score: isize,
    total_score: isize,
    current_level: isize
}

impl VoodooResponse {
    fn new(scene: &Scene) -> VoodooResponse {
        VoodooResponse {
            next_level: scene.scene_number+1,
            arm_score: scene.arms,
            head_score: scene.head,
            legs_score: scene.legs,
            body_score: scene.body,
            total_score: scene.arms + scene.head + scene.legs + scene.body,
            current_level: scene.scene_number
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ActionContent {
    head: isize,
    body: isize,
    arm_left: isize,
    arm_right: isize,
    leg_left: isize,
    leg_right: isize
}