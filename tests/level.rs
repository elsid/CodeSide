use model::{
    Level,
    Tile,
};
use aicup2019::{
    examples::example_level,
    my_strategy::{
        Location,
        dump_level,
        get_tile_index,
        get_tile_location,
    }
};

#[test]
fn test_dump_level_1x1() {
    assert_eq!(dump_level(&Level {tiles: vec![vec![Tile::Wall]]}), "#\n".to_string())
}

#[test]
fn test_dump_level_5x1() {
    assert_eq!(
        dump_level(&Level {tiles: vec![vec![Tile::Empty], vec![Tile::Wall], vec![Tile::Ladder], vec![Tile::Platform], vec![Tile::JumpPad]]}),
        ".#H^T\n".to_string()
    );
}

#[test]
fn test_dump_level_example_level() {
    assert_eq!(
        dump_level(&example_level()),
        "########################################\n\
         #.................#....................#\n\
         #......................................#\n\
         #.................#....................#\n\
         #...........H#############.............#\n\
         #...........H..........................#\n\
         #...........H..........................#\n\
         #...........H..........................#\n\
         #.....########^^^^^^^^^^########.......#\n\
         #..........#.............#.............#\n\
         #..........#.............#.............#\n\
         #..........#............##.............#\n\
         #..........##..........................#\n\
         #.................^^#..................#\n\
         #...................#..................#\n\
         #...................#..................#\n\
         #.....T..................^^^###H###....#\n\
         #....########..................H.......#\n\
         #....#......#..................H.......#\n\
         #...........#..................H.......#\n\
         #..#...............T...................#\n\
         #..#............#######^^^^^...........#\n\
         #..####^^H^............................#\n\
         #........H.............................#\n\
         #........H.............................#\n\
         #....H#######^^^^#^^^^^^^^######.......#\n\
         #....H.................H...............#\n\
         #....H.................H...............#\n\
         #....H.......T...#.#...H.........T.....#\n\
         ########################################\n".to_string()
    );
}

#[test]
fn test_get_tile_index() {
    assert_eq!(get_tile_index(&example_level(), Location::new(0, 0)), 0);
    assert_eq!(get_tile_index(&example_level(), Location::new(39, 0)), 1170);
    assert_eq!(get_tile_index(&example_level(), Location::new(0, 29)), 29);
    assert_eq!(get_tile_index(&example_level(), Location::new(39, 29)), 1199);
    assert_eq!(get_tile_index(&example_level(), Location::new(10, 10)), 310);
}

#[test]
fn test_get_tile_location() {
    assert_eq!(get_tile_location(&example_level(), 0), Location::new(0, 0));
    assert_eq!(get_tile_location(&example_level(), 1170), Location::new(39, 0));
    assert_eq!(get_tile_location(&example_level(), 29), Location::new(0, 29));
    assert_eq!(get_tile_location(&example_level(), 1199), Location::new(39, 29));
    assert_eq!(get_tile_location(&example_level(), 310), Location::new(10, 10));
}
