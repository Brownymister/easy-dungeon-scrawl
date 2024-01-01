use crate::MapBlockTypes;

pub type Map = Vec<Vec<MapBlockTypes>>;

pub fn generate_map(map_str: String) -> Map {
    let mut map = vec![];

    for (_, row_str) in map_str.lines().collect::<Vec<&str>>().iter().enumerate() {
        let mut row: Vec<MapBlockTypes> = vec![];
        for (_, item) in row_str.split("|").filter(|x| x != &" " && x != &"" ).enumerate() {
            row.push(get_block_type(item))
        }
        map.push(row);
    }
    return map;
}

fn get_block_type(str: &str) -> MapBlockTypes {
    let new_maps_trigger_re = regex::Regex::new(r"M(\d+)").unwrap();
    let new_maps_trigger_caps = new_maps_trigger_re.captures(str);
    if new_maps_trigger_caps.is_some() {
        return MapBlockTypes::NewMapTrigger(
            new_maps_trigger_caps
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
                .parse()
                .unwrap(),
        );
    }

    let item_trigger_re = regex::Regex::new(r"I(\d+)").unwrap();
    let item_trigger_caps = item_trigger_re.captures(str);
    if item_trigger_caps.is_some() {
        return MapBlockTypes::ItemTrigger(
            item_trigger_caps
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
                .parse()
                .unwrap(),
        );
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
    for (j, row) in map.iter().enumerate() {
        let mut row_str = "".to_string();
        for (i, item) in row.iter().enumerate() {
            let mut get_symbol = match &item {
                MapBlockTypes::Path => "_",
                MapBlockTypes::NotWalkable => "x",
                _ => "_",
            };
            if player_pos.is_some() && j == player_pos.unwrap().j && i == player_pos.unwrap().i {
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
        assert_eq!(b, crate::MapBlockTypes::NewMapTrigger(1));
        let b = get_block_type("I0");
        assert_eq!(b, crate::MapBlockTypes::ItemTrigger(0));
    }

    #[test]
    fn test_map_gen() {
        let map = generate_map(
"|x|M1|M1|x|x|x|x|x|x|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|_|I0|_|_|x|
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
        visulize_map(&map, None)
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|x| println!("{}", x));
        assert_eq!(
            visulize_map(&map, None),
            "|x|_|_|x|x|x|x|x|x|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|_|_|_|_|x|
|x|_|_|x|x|x|_|_|x|x|
|x|_|_|x|x|x|_|_|x|x|
|x|_|_|_|_|_|_|_|x|x|
|x|x|x|_|_|_|x|x|x|x|
|x|x|x|x|_|_|x|x|x|x|
|x|x|x|x|_|_|x|x|x|x|
"
                .to_string()
        )
    }
}
