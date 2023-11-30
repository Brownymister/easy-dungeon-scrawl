use serde::Deserialize;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

mod custom_layer;
mod map_gen;

fn main() {
    println!("You stand at the mouth of a dark, damp cavern. The air is heavy with the smell of decay, and a chill runs down your spine. Do you dare enter the unknown depths, or retreat to the safety of the light?");

    let game_settings_res = custom_layer::parse_game_settings("test.toml");
    let game_settings;
    match game_settings_res {
        Ok(v) => game_settings = v,
        Err(e) => {
            println!("Error while parsing toml file: {:?}", e);
            exit(0)
        }
    }

    let maps: Vec<map_gen::Map> = game_settings
        .maps
        .iter()
        .map(|map_str| map_gen::generate_map(map_str.to_string()))
        .collect();

    let game = Game {
        playername: game_settings.player.name,
        cur_map: maps[0].clone(),
        health: game_settings.player.total_health,
        global_items: game_settings.global_items,
        inventory: vec![],
        pos: (game_settings.start_pos[0], game_settings.start_pos[1]),
        maps,
    };

    loop {
        let mut s = String::new();

        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        println!("You typed: {}", s);
        if s == "go north" {
            game.north();
        }
    }
}

pub struct Game {
    playername: String,
    health: i32,
    global_items: Vec<GameItem>,
    inventory: Vec<InventoryElement>,
    maps: Vec<map_gen::Map>,
    cur_map: map_gen::Map,
    pos: (usize, usize),
}

#[derive(Deserialize, Debug)]
pub struct GameItem {
    pub item_id: String,
    pub at: i32,
}

#[derive(Deserialize, Debug)]
pub struct Enemy {
    name: String,
    pos: (usize, usize),
    health: i32,
    weapon: String,
}

impl Movement for Game {
    fn north(&self) {
        let j = self.pos.1;
        println!("{}", j);
        if j == 0
            || self.get_map_block_types((self.pos.0, j.clone() - 1)) == &MapBlockTypes::NotWalkable
        {
            println!("Dort kannst du nich hin gehen.")
        } else {
            let newpos = (self.pos.0, self.pos.1 - 1);
            println!("{:?}", newpos);
        }
    }
}

impl Game {
    fn get_map_block_types(&self, pos: (usize, usize)) -> &MapBlockTypes {
        let i = pos.0;
        let j = pos.1;
        let row = &self.cur_map[i];
        let map_block = &row[j];
        return &map_block.block_type;
    }
}

pub struct InventoryElement {
    item: GameItem,
    count: usize,
}

pub trait Movement {
    fn north(&self) {}
    fn south(&self) {}
    fn west(&self) {}
    fn east(&self) {}
}

#[derive(Debug, PartialEq, Clone)]
pub enum MapBlockTypes {
    Path,
    NotWalkable,
    Trap,
    NewMapTrigger { map_id: String },
    EnemyTrigger { enemy_id: String },
    ItemTrigger { time_id: String },
}

#[derive(Debug, Clone)]
pub struct MapBlock {
    i: usize,
    j: usize,
    block_type: MapBlockTypes,
}
