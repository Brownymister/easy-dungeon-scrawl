use std::io::{stdin, stdout, Write};

mod map_gen;

fn main() {
    println!("You stand at the mouth of a dark, damp cavern. The air is heavy with the smell of decay, and a chill runs down your spine. Do you dare enter the unknown depths, or retreat to the safety of the light?");
    let map = map_gen::generate_map();
    println!("{:?}", map);

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
    }
}

struct Game {
    player: String,
    health: String,
    inventory: Vec<InventoryElement>,
}

struct InventoryElement {
    name: String,
}

trait Movement {
    fn north(game: Game) {}
    fn south(game: Game) {}
    fn west(game: Game) {}
    fn east(game: Game) {}
}

#[derive(Debug)]
enum MapBlockTypes {
    Path,
    NotWalkable,
    Trap,
}

#[derive(Debug)]
struct MapBlock {
    i: i32,
    j: i32,
    block_type: MapBlockTypes,
}
