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
    let new_maps_trigger_re = regex::Regex::new(r"M(\d+)").unwrap();
    let new_maps_trigger_caps = new_maps_trigger_re.captures(str);
    if new_maps_trigger_caps.is_some() {
        return MapBlockTypes::NewMapTrigger {
            map_id: (&new_maps_trigger_caps.unwrap().get(1).unwrap().as_str()).to_string(),
        };
    }

    let item_trigger_re = regex::Regex::new(r"I(\d+)").unwrap();
    let item_trigger_caps = item_trigger_re.captures(str);
    if item_trigger_caps.is_some() {
        return MapBlockTypes::NewMapTrigger {
            map_id: (&item_trigger_caps.unwrap().get(1).unwrap().as_str()).to_string(),
        };
    }
    match str {
        "x" => MapBlockTypes::NotWalkable,
        "_" => MapBlockTypes::Path,
        "T" => MapBlockTypes::Trap,
        _ => MapBlockTypes::NotWalkable,
    }
}

pub fn visulize_map(map: &Map, player_pos: Option<&crate::Pos>) -> String {
    let mut map_str = "".to_string();
    for j in map {
        let mut row_str = "".to_string();
        for (i, item) in j.iter().enumerate() {
            assert_eq!(i, item.i);
            let mut get_symbol = match &item.block_type {
                MapBlockTypes::Path => "_",
                MapBlockTypes::NotWalkable => "x",
                MapBlockTypes::NewMapTrigger { map_id } => map_id.as_str(),
                MapBlockTypes::EnemyTrigger { enemy_id } => enemy_id.as_str(),
                MapBlockTypes::ItemTrigger { itme_id } => itme_id.as_str(),
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
        map_str.push_str(&format!("{}\n", row_str));
    }
    return map_str;
}

#[cfg(test)]
mod tests {
    use crate::map_gen::{generate_map, get_block_type, visulize_map};

    #[test]
    fn test_get_block_types() {
        let b = get_block_type("M1");
        println!("get_block_type {:?}", b);
        assert_eq!(b, crate::MapBlockTypes::NewMapTrigger { map_id: "1".to_string() });
    }

    #[test]
    fn test_map_gen() {
        let map = generate_map(
            "|x|1|1|x|x|x|x|x|x|x|
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
        println!("{:?}", visulize_map(&map, None));
    }
}
