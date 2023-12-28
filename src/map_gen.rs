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
    let re = regex::Regex::new("^[1-9]*$").unwrap();
    let caps = re.captures(str);
    if caps.is_some() {
        return MapBlockTypes::NewMapTrigger {
            map_id: (&caps.unwrap()[0]).to_string(),
        };
    }
    match str {
        "x" => MapBlockTypes::NotWalkable,
        "_" => MapBlockTypes::Path,
        _ => MapBlockTypes::NotWalkable,
    }
}

pub fn visulize_map(map: Map, player_pos: Option<&crate::Pos>) {
    println!("{:?}", map);
    println!("countertest str:");
    for j in map {
        let mut row_str = "".to_string();
        for (i, item) in j.iter().enumerate() {
            assert_eq!(i, item.i);
            let mut get_symbol = match &item.block_type {
                MapBlockTypes::Path => "_",
                MapBlockTypes::NotWalkable => "x",
                MapBlockTypes::NewMapTrigger { map_id } => map_id.as_str(),
                MapBlockTypes::EnemyTrigger { enemy_id } => enemy_id.as_str(),
                MapBlockTypes::ItemTrigger { time_id } => time_id.as_str(),
                // MapBlockTypes::Trap => "",
                _ => " ",
            };
            if player_pos.is_some()
                && item.j == player_pos.unwrap().j
                && item.i == player_pos.unwrap().i
            {
                get_symbol = "P";
            }
            row_str.push_str(&format!("|{}", get_symbol))
        }
        row_str.push_str("|");
        println!("{}", row_str);
    }
}

#[cfg(test)]
mod tests {
    use crate::map_gen::{generate_map, get_block_type, visulize_map};

    #[test]
    fn test_get_block_types() {
        let b = get_block_type("1");
        let re = regex::Regex::new("^[0-9]*$").unwrap();
        let caps = re.captures("1").unwrap();
        println!("get_block_type {:?}", (&caps[0]).parse::<usize>().unwrap());
    }

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
        visulize_map(map, None)
    }
}
