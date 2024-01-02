use crate::MapBlockTypes;

pub type Map = Vec<Vec<MapBlockTypes>>;

pub fn generate_map(map_str: String) -> Map {
    let mut map = vec![];

    for (_, row_str) in map_str.lines().collect::<Vec<&str>>().iter().enumerate() {
        let mut row: Vec<MapBlockTypes> = vec![];
        for (_, item) in row_str
            .split("|")
            .filter(|x| x != &" " && x != &"")
            .enumerate()
        {
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
        return MapBlockTypes::NewMapTrigger(extract_first_match(new_maps_trigger_caps.unwrap()));
    }

    let item_trigger_re = regex::Regex::new(r"I(\d+)").unwrap();
    let item_trigger_caps = item_trigger_re.captures(str);
    if item_trigger_caps.is_some() {
        return MapBlockTypes::ItemTrigger(extract_first_match(item_trigger_caps.unwrap()));
    }

    let tp_trigger_re = regex::Regex::new(r"T(.+)").unwrap();
    let tp_trigger_caps = tp_trigger_re.captures(str);
    if tp_trigger_caps.is_some() {
        let input = tp_trigger_caps.unwrap().get(1).unwrap().as_str();

        let values: Vec<&str> = input
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();

        return MapBlockTypes::TeleportTrigger(
            values[0].parse().unwrap(),
            values[1].parse().unwrap(),
            values[2].parse().unwrap(),
        );
    }

    match str {
        "x" => MapBlockTypes::NotWalkable,
        "_" => MapBlockTypes::Path,
        _ => MapBlockTypes::NotWalkable,
    }
}

fn extract_first_match(caps: regex::Captures) -> usize {
    return caps.get(1).unwrap().as_str().to_string().parse().unwrap();
}

pub fn visulize_map(map: &Map, player_pos: Option<&crate::Pos>) -> String {
    let mut map_str = "".to_string();
    for (j, row) in map.iter().enumerate() {
        let mut row_str = "".to_string();
        for (i, item) in row.iter().enumerate() {
            let mut get_symbol = match &item {
                MapBlockTypes::Path => "  ",
                MapBlockTypes::NotWalkable => "XX",
                _ => "  ",
            };
            if player_pos.is_some() && j == player_pos.unwrap().j && i == player_pos.unwrap().i {
                get_symbol = "<>";
            }
            row_str.push_str(&format!("{}", get_symbol))
        }
        map_str.push_str(&format!("{}\n", row_str));
    }
    return map_str;
}

#[cfg(test)]
mod tests {
    use crate::map_gen::{extract_first_match, generate_map, get_block_type, visulize_map};

    #[test]
    fn test_get_block_types() {
        let b = get_block_type("M1");
        println!("get_block_type {:?}", b);
        assert_eq!(b, crate::MapBlockTypes::NewMapTrigger(1));
        let b = get_block_type("I0");
        assert_eq!(b, crate::MapBlockTypes::ItemTrigger(0));
    }

    #[test]
    fn test_extract_first_match() {
        let tp_trigger_re = regex::Regex::new(r"T(.+)").unwrap();
        let tp_trigger_caps = tp_trigger_re.captures("T(1,2,3)");
        println!("tp_trigger_caps {:?}", tp_trigger_caps);
        let input = tp_trigger_caps.unwrap().get(1).unwrap().as_str();

        let values: Vec<&str> = input
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();

        // Parse each value and collect into a tuple
        let tuple: (i32, i32, i32) = (
            values[0].parse().unwrap(),
            values[1].parse().unwrap(),
            values[2].parse().unwrap(),
        );

        assert_eq!(tuple, (1, 2, 3));
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
            "XX    XXXXXXXXXXXXXX
XX    XXXX        XX
XX    XXXX        XX
XX    XXXX        XX
XX    XXXXXX    XXXX
XX    XXXXXX    XXXX
XX              XXXX
XXXXXX      XXXXXXXX
XXXXXXXX    XXXXXXXX
XXXXXXXX    XXXXXXXX\n"
                .to_string()
        )
    }
}
