use crate::map_gen::visulize_map;
use serde::{Deserialize, Serialize};
use crate::map_gen;

#[derive(Debug)]
pub struct Game {
    pub playername: String,
    pub health: i32,
    pub global_items: Vec<GameItem>,
    pub inventory: Vec<InventoryElement>,
    pub maps: Vec<map_gen::Map>,
    pub cur_map: map_gen::Map,
    pub pos: Pos,
}

#[derive(Debug)]
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

pub struct charakteristiks {
    pub courage: i32,
    pub Strength: i32,
    pub Intelligence: i32,
    pub Intuition: i32,
}


impl Movement for Game {
    fn north(&mut self) {
        let j = self.pos.j;
        println!("{}", j);
        println!(
            "{:?}",
            self.get_map_block_type(Pos {
                i: self.pos.i,
                j: j.clone() - 1
            })
        );
        if j == 0
            || self.get_map_block_type(Pos {
                j: j.clone() - 1,
                i: self.pos.i,
            }) == &MapBlockTypes::NotWalkable
        {
            println!("Dort kannst du nich hin gehen.")
        } else {
            let newpos = Pos {
                i: self.pos.i,
                j: self.pos.j - 1,
            };
            self.pos = newpos;
            println!("{:?}", visulize_map(self.cur_map.clone(), Some(&self.pos)));
        }
    }
}

impl Game {
    fn get_map_block_type(&self, pos: Pos) -> &MapBlockTypes {
        let row = &self.cur_map[pos.j];
        println!("{:?}", row);
        let map_block = &row[pos.i];
        println!("{:?}", map_block);
        return &map_block.block_type;
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
    NewMapTrigger { map_id: String },
    EnemyTrigger { enemy_id: String },
    ItemTrigger { time_id: String },
}

#[derive(Debug, Clone)]
pub struct MapBlock {
    pub i: usize,
    pub j: usize,
    pub block_type: MapBlockTypes,
}

