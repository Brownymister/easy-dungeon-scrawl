use crate::map_gen;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Game {
    pub playername: String,
    pub health: i32,
    pub global_items: Vec<GameItem>,
    pub inventory: Vec<InventoryElement>,
    pub maps: Vec<map_gen::Map>,
    pub cur_map: map_gen::Map,
    pub pos: Pos,
    pub info_queue: InfoQueue,
}

#[derive(Debug)]
pub struct InfoMessage {
    pub title: String,
    pub message: String,
    // pub time: u128,
}

impl InfoMessage {
    fn new(title: String, message: String) -> InfoMessage {
        let time = 0;
        return InfoMessage {
            title,
            message,
            // time,
        };
    }
}

#[derive(Debug)]
pub struct InfoQueue {
    pub queue: VecDeque<InfoMessage>,
    pub timer: usize,
}

impl InfoQueue {
    pub fn new() -> InfoQueue {
        return InfoQueue {
            queue: VecDeque::new(),
            timer: 30,
        };
    }

    pub fn queue(&mut self, title: String, message: String) {
        let info = InfoMessage::new(title, message);
        self.queue.push_back(info);
        log::info!("{:?}", self.queue);
    }

    pub fn dequeue(&mut self) {
        self.timer = 30;
        self.queue.pop_front();
    }

    pub fn head(&self) -> Option<&InfoMessage> {
        self.queue.front()
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub i: usize,
    pub j: usize,
}

#[derive(Deserialize, Debug)]
pub struct GameItem {
    pub item_id: String,
    pub at: i32,
}

#[derive(Deserialize, Debug)]
pub struct Enemy {
    pub name: String,
    pub pos: (usize, usize),
    pub health: i32,
    pub weapon: String,
}

// pub struct charakteristiks {
//     pub courage: i32,
//     pub Strength: i32,
//     pub Intelligence: i32,
//     pub Intuition: i32,
// }

impl Movement for Game {
    fn north(&mut self) {
        let j = self.pos.j;
        if j == 0
            || self.get_map_block_type(Pos {
                j: j.clone() - 1,
                i: self.pos.i,
            }) == &MapBlockTypes::NotWalkable
        {
            self.info_queue.queue(
                "Error".to_string(),
                "Dort kannst du nicht hin gehen".to_string(),
            );
        } else if let &MapBlockTypes::NewMapTrigger(new_map_id) =
            self.get_map_block_type(self.pos.clone())
        {
            self.cur_map = self.maps[new_map_id as usize].clone();
            let newpos = Pos {
                i: self.pos.i,
                j: self.cur_map.len() - 1,
            };
            self.pos = newpos;
        } else {
            let newpos = Pos {
                i: self.pos.i,
                j: self.pos.j - 1,
            };
            self.pos = newpos;
        }
    }

    fn south(&mut self) {
        if self.pos.j == self.cur_map.len() - 1
            || self.get_map_block_type(Pos {
                j: self.pos.j + 1,
                i: self.pos.i,
            }) == &MapBlockTypes::NotWalkable
        {
            self.info_queue.queue(
                "Error".to_string(),
                "Dort kannst du nicht hin gehen".to_string(),
            );
        } else {
            let newpos = Pos {
                i: self.pos.i,
                j: self.pos.j + 1,
            };
            self.pos = newpos;
        }
    }

    fn west(&mut self) {
        if self.pos.i == 0
            || self.get_map_block_type(Pos {
                j: self.pos.j,
                i: self.pos.i - 1,
            }) == &MapBlockTypes::NotWalkable
        {
            self.info_queue.queue(
                "Error".to_string(),
                "Dort kannst du nicht hin gehen".to_string(),
            );
        } else {
            let newpos = Pos {
                i: self.pos.i - 1,
                j: self.pos.j,
            };
            self.pos = newpos;
        }
    }

    fn east(&mut self) {
        if self.pos.i == self.cur_map[0].len() - 1
            || self.get_map_block_type(Pos {
                j: self.pos.j,
                i: self.pos.i + 1,
            }) == &MapBlockTypes::NotWalkable
        {
            self.info_queue.queue(
                "Error".to_string(),
                "Dort kannst du nicht hin gehen".to_string(),
            );
        } else {
            let newpos = Pos {
                i: self.pos.i + 1,
                j: self.pos.j,
            };
            self.pos = newpos;
        }
    }
}

impl Game {
    fn get_map_block_type(&self, pos: Pos) -> &MapBlockTypes {
        let row = &self.cur_map[pos.j];
        let map_block = &row[pos.i];
        return &map_block;
    }

    pub fn new() -> Game {
        let game_settings_res = crate::custom_layer::parse_game_settings("test.toml");
        let game_settings;
        match game_settings_res {
            Ok(v) => game_settings = v,
            Err(e) => {
                println!("Error while parsing toml file: {:?}", e);
                std::process::exit(0);
            }
        }

        let maps: Vec<map_gen::Map> = game_settings
            .maps
            .iter()
            .map(|map_str| map_gen::generate_map(map_str.to_string()))
            .collect();

        return Game {
            playername: game_settings.player.name,
            cur_map: maps[0].clone(),
            health: game_settings.player.total_health,
            global_items: game_settings.global_items,
            inventory: vec![],
            pos: Pos {
                i: game_settings.start_pos[0],
                j: game_settings.start_pos[1],
            },
            info_queue: InfoQueue::new(),
            maps,
        };
    }
}

#[derive(Debug)]
pub struct InventoryElement {
    pub item: GameItem,
    pub count: usize,
}

pub trait Movement {
    fn north(&mut self) {}
    fn south(&mut self) {}
    fn west(&mut self) {}
    fn east(&mut self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub enum MapBlockTypes {
    Path,
    NotWalkable,
    Trap,
    NewMapTrigger(usize),
    EnemyTrigger(usize),
    ItemTrigger(usize),
}
