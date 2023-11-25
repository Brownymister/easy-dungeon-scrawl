use std::io::{stdin, stdout, Write};

mod map_gen;

fn main() {
    println!("You stand at the mouth of a dark, damp cavern. The air is heavy with the smell of decay, and a chill runs down your spine. Do you dare enter the unknown depths, or retreat to the safety of the light?");
    let map = map_gen::generate_map(
        "|x|x|x|x|x|x|x|x|x|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|x|_|_|x|x|
|x|_|_|x|x|x|_|_|x|x|
|x|_|_|_|_|_|_|_|x|x|
|x|x|x|_|_|_|x|x|x|x|
|x|x|x|x|_|_|x|x|x|x|
|x|x|x|x|_|_|x|x|x|x|"
            .to_string(),
    );

    println!("{:?}", map);

    let game = Game::new(map);
    loop {
        let mut s = String::new();

        print!("Please enter some text: ");
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
    health: String,
    inventory: Vec<InventoryElement>,
    map: map_gen::Map,
    /// i, j
    pos: (usize, usize),
}

impl Movement for Game {
    fn north(&self) {
        let j = self.pos.1;
        println!("{}", j);
        println!("{}", j - 1);
        let block_type_next = self.get_map_block_types((self.pos.0, j.clone() - 1));
        if j == 0 || block_type_next == &MapBlockTypes::NotWalkable {
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
        let row = &self.map[i];
        let map_block = &row[j];
        return &map_block.block_type;
    }
    fn new(map: map_gen::Map) -> Game {
        return Game {
            playername: "".to_string(),
            health: "".to_string(),
            inventory: vec![],
            pos: (5, 0),
            map,
        };
    }
}

pub struct InventoryElement {
    item: String,
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
}

#[derive(Debug, Clone)]
pub struct MapBlock {
    i: usize,
    j: usize,
    block_type: MapBlockTypes,
}
