use crate::{MapBlock, MapBlockTypes};

pub type Map = Vec<Vec<MapBlock>>;

pub fn generate_map(map_str: String) -> Map {
    let mut map = vec![];

    for (j, row_str) in map_str.split("\n").enumerate() {
        let mut row: Vec<MapBlock> = vec![];

        for (i, item) in row_str.split("|").filter(|x| x != &"").enumerate() {
            row.push(MapBlock {
                i: i.try_into().unwrap(),
                j: j.try_into().unwrap(),
                block_type: get_block_type(item),
            });
        }
        map.push(row);
    }
    return map;
}

fn get_block_type(str: &str) -> MapBlockTypes {
    match str {
        "x" => MapBlockTypes::NotWalkable,
        "_" => MapBlockTypes::Path,
        _ => MapBlockTypes::NotWalkable,
    }
}

#[cfg(test)]
mod tests {
    use crate::map_gen::{generate_map, Map, MapBlockTypes};

    #[test]
    fn test_map_gen() {
        let map = generate_map(
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
        visulize_map(map)
    }

    fn visulize_map(map: Map) {
        println!("{:?}", map);
        println!("countertest str:");
        for j in map {
            let mut row_str = "".to_string();
            for (i, item) in j.iter().enumerate() {
                assert_eq!(i, item.i);
                let get_symbol = match item.block_type {
                    MapBlockTypes::Path => "_",
                    MapBlockTypes::NotWalkable => "x",
                    // MapBlockTypes::Trap => "",
                    _ => "",
                };
                row_str.push_str(&format!("|{}", get_symbol))
            }
            row_str.push_str("|");
            println!("{}", row_str);
        }
    }
}
