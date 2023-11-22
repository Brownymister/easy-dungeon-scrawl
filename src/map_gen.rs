use crate::{MapBlock, MapBlockTypes};

type Map = Vec<Vec<MapBlock>>;

/*
 * |x|x|x|x|x|x|x|x|x|x|
 * |x|_|_|x|x|_|_|_|_|x|
 * |x|_|_|x|x|_|_|_|_|x|
 * |x|_|_|x|x|_|_|_|_|x|
 * |x|_|_|x|x|x|_|_|x|x|
 * |x|_|_|x|x|x|_|_|x|x|
 * |x|_|_|_|_|_|_|_|x|x|
 * |x|x|x|_|_|_|x|x|x|x|
 * |x|x|x|x|_|_|x|x|x|x|
 * |x|x|x|x|_|_|x|x|x|x|
*/
pub fn generate_map() -> Map {
    let mut map = vec![];
    let mut first_row = vec![];
    for i in 0..10 {
        first_row.push(MapBlock {
            i,
            j: 0,
            block_type: MapBlockTypes::NotWalkable,
        })
    }

    map.push(first_row);
    for _ in 0..3 {
        let second_row = generate_second_row();
        map.push(second_row);
    }

    return map;
}

fn generate_second_row() -> Vec<MapBlock> {
    let mut second_row = vec![];
    second_row = push_not_walkable(second_row, 0, 1);
    for i in 1..3 {
        second_row.push(MapBlock {
            i,
            j: 1,
            block_type: MapBlockTypes::Path,
        });
    }

    for i in 3..5 {
        second_row = push_not_walkable(second_row, i, 1);
    }

    for i in 5..9 {
        second_row.push(MapBlock {
            i,
            j: 1,
            block_type: MapBlockTypes::Path,
        });
    }
    second_row = push_not_walkable(second_row, 9, 1);
    return second_row;
}
fn push_not_walkable(mut row: Vec<MapBlock>, i: usize, j: usize) -> Vec<MapBlock> {
    row.push(MapBlock {
        i,
        j,
        block_type: MapBlockTypes::NotWalkable,
    });
    return row;
}

#[cfg(test)]
mod tests {
    use crate::map_gen::{generate_map, Map, MapBlockTypes};

    #[test]
    fn test_map_gen() {
        let map = generate_map();
        visulize_map(map)
    }

    fn visulize_map(map: Map) {
        println!("{:?}", map);
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
