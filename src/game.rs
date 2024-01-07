use crate::info_manager::*;
use crate::map_gen;
use serde::Deserialize;

#[derive(Debug)]
pub struct Game {
    pub playername: String,
    pub health: i32,
    pub global_items: Vec<ItemProps>,
    pub inventory: Inventory,
    pub maps: Vec<map_gen::Map>,
    pub cur_map: usize,
    pub pos: Pos,
    pub info_queue: InfoQueue,
}

#[derive(Debug, Clone)]
pub struct Pos {
    pub i: usize,
    pub j: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemProps {
    pub id: usize,
    pub name: String,
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
//     pub strength: i32,
//     pub intelligence: i32,
//     pub intuition: i32,
// }

fn block_is_new_map(map_block: &MapBlockTypes) -> Option<usize> {
    if let &MapBlockTypes::NewMapTrigger(new_map_id) = map_block {
        Some(new_map_id)
    } else {
        None
    }
}

fn block_is_teleport_trigger(map_block: &MapBlockTypes) -> Option<(usize, usize, usize)> {
    if let &MapBlockTypes::TeleportTrigger(teleport_id, j, i) = map_block {
        Some((teleport_id, j, i))
    } else {
        None
    }
}

impl Movement for Game {
    fn movement(&mut self, incoming_block: Pos, new_map_pos: Pos, is_edge: bool) {
        let new_map_block_id = block_is_new_map(self.get_map_block_type(&self.pos.clone()));
        if is_edge && new_map_block_id.is_some() {
            self.cur_map = new_map_block_id.unwrap();
            self.pos = new_map_pos;
            return;
        }

        let teleport_id = block_is_teleport_trigger(self.get_map_block_type(&incoming_block));
        if teleport_id.is_some() {
            let (teleport_id, j, i) = teleport_id.unwrap();
            self.cur_map = teleport_id;
            let newpos = Pos { i, j };
            self.pos = newpos;
            return;
        }
        if !is_edge
            && self.get_map_block_type(&incoming_block.clone()) != &MapBlockTypes::NotWalkable
        {
            if let &MapBlockTypes::ItemTrigger(item_id) =
                self.get_map_block_type(&incoming_block.clone())
            {
                self.inventory
                    .add_item(item_id, &self.global_items)
                    .unwrap();

                self.info_queue
                    .queue("Item".to_string(), self.global_items[item_id].name.clone());

                self.remove_item_from_map(&incoming_block);
            }
            self.pos = incoming_block;
        }
    }

    fn north(&mut self) {
        let mut incoming_block = Pos {
            j: self.pos.j,
            i: self.pos.i,
        };
        if (self.pos.j as isize - 1) >= 0 {
            incoming_block.j -= 1;
        }
        self.movement(
            incoming_block,
            Pos {
                i: self.pos.i,
                j: self.maps[self.cur_map].len() - 1,
            },
            self.pos.j == 0,
        );
    }

    fn south(&mut self) {
        let mut incoming_block = Pos {
            j: self.pos.j,
            i: self.pos.i,
        };
        if (self.pos.j as isize + 1) < self.maps[self.cur_map].len() as isize {
            incoming_block.j += 1;
        }
        self.movement(
            incoming_block,
            Pos {
                i: self.pos.i,
                j: 0,
            },
            self.pos.j == self.maps[self.cur_map].len() - 1,
        );
    }

    fn west(&mut self) {
        let mut incoming_block = Pos {
            j: self.pos.j,
            i: self.pos.i,
        };
        if (self.pos.i as isize - 1) >= 0 {
            incoming_block.i -= 1;
        }
        self.movement(
            incoming_block,
            Pos {
                i: self.maps[self.cur_map][self.pos.j].len() - 1,
                j: self.pos.j,
            },
            self.pos.i == 0,
        );
    }

    fn east(&mut self) {
        let mut incoming_block = Pos {
            j: self.pos.j,
            i: self.pos.i,
        };
        if (self.pos.i as isize + 1) < self.maps[self.cur_map][self.pos.j].len() as isize {
            incoming_block.i += 1;
        }
        self.movement(
            incoming_block,
            Pos {
                i: 0,
                j: self.pos.j,
            },
            self.pos.i == self.maps[self.cur_map][self.pos.j].len() - 1,
        );
    }
}

impl Game {
    fn get_map_block_type(&self, pos: &Pos) -> &MapBlockTypes {
        let row = &self.maps[self.cur_map][pos.j];
        let map_block = &row[pos.i];
        return &map_block;
    }

    pub fn new() -> Game {
        let game_settings = match crate::custom_layer::parse_game_settings("test.yaml") {
            Ok(v) => v,
            Err(e) => {
                println!("Error while parsing toml file: {:?}", e);
                std::process::exit(0);
            }
        };

        let maps: Vec<map_gen::Map> = game_settings
            .maps
            .iter()
            .map(|map_str| map_gen::generate_map(map_str.to_string()))
            .collect();

        return Game {
            playername: game_settings.player.name,
            cur_map: 0,
            health: game_settings.player.total_health,
            global_items: game_settings.global_items,
            inventory: Inventory::new(),
            pos: Pos {
                i: game_settings.start_pos[0],
                j: game_settings.start_pos[1],
            },
            info_queue: InfoQueue::new(),
            maps,
        };
    }

    pub fn remove_item_from_map(&mut self, pos: &Pos) {
        for j in 0..self.maps[self.cur_map].len() {
            let row = &self.maps[self.cur_map][j];
            for i in 0..row.len() {
                if i == pos.i && j == pos.j {
                    self.maps[self.cur_map][j][i] = MapBlockTypes::Path;
                }
            }
        }
    }

    pub fn add_item_to_map(&mut self, pos: &Pos, item_id: usize) {
        todo!("impl add item to map");
    }
}

#[derive(Debug)]
pub struct InventoryElement {
    pub acquisition_time: u128,
    pub props: ItemProps,
}

#[derive(Debug)]
pub struct Inventory {
    pub inventory: Vec<InventoryElement>,
}

impl Inventory {
    fn new() -> Inventory {
        return Inventory { inventory: vec![] };
    }

    fn get_item_props(&self, item_id: &usize, global_items: &Vec<ItemProps>) -> Option<ItemProps> {
        for item_prop in global_items {
            if item_prop.id == *item_id {
                return Some(item_prop.clone());
            }
        }

        return None;
    }

    fn add_item(&mut self, id: usize, global_items: &Vec<ItemProps>) -> Result<(), String> {
        let props = self
            .get_item_props(&id, global_items)
            .ok_or("Item not found".to_string())?;

        let item = InventoryElement {
            acquisition_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            props,
        };
        self.inventory.push(item);
        return Ok(());
    }

    pub fn to_string(&self) -> String {
        let mut str = "".to_string();
        for item in &self.inventory {
            str += &item.props.name;
            str += "\n";
        }
        return str;
    }
}

pub trait Movement {
    fn north(&mut self) {}
    fn south(&mut self) {}
    fn west(&mut self) {}
    fn east(&mut self) {}
    fn movement(&mut self, new_pos: Pos, new_map_pos: Pos, is_edge: bool) {}
}

#[derive(Debug, PartialEq, Clone)]
pub enum MapBlockTypes {
    Path,
    NotWalkable,
    Trap,
    NewMapTrigger(usize),
    TeleportTrigger(usize, usize, usize),
    EnemyTrigger(usize),
    ItemTrigger(usize),
}
