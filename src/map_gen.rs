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
pub fn generate_map() -> Vec<Vec<MapBlock>> {
    let mut map = vec![];
    let mut first_row = vec![];
    for i in 0..10 {
        first_row.push(MapBlock {
            i,
            j: 0,
            BlockType: MapBlockTypes::NotWalkable,
        })
    }

    for i in 0..10 {
        first_row.push(MapBlock {
            i,
            j: 0,
            BlockType: MapBlockTypes::NotWalkable,
        })
    }

    map.push(first_row);
    return map;
}
