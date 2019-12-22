use model::{
    Tile,
};
use aicup2019::{
    examples::example_level,
    my_strategy::{
        Level,
        Location,
        dump_level,
    }
};

#[test]
fn test_dump_level_1x1() {
    assert_eq!(dump_level(&Level::from_model(&model::Level {tiles: vec![vec![Tile::Wall]]})), "#\n".to_string())
}

#[test]
fn test_dump_level_5x1() {
    assert_eq!(
        dump_level(&Level::from_model(&model::Level {tiles: vec![vec![Tile::Empty], vec![Tile::Wall], vec![Tile::Ladder], vec![Tile::Platform], vec![Tile::JumpPad]]})),
        ".#H^T\n".to_string()
    );
}

#[test]
fn test_dump_level_example_level() {
    assert_eq!(
        dump_level(&Level::from_model(&example_level())),
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
    assert_eq!(Level::from_model(&example_level()).get_tile_index(Location::new(0, 0)), 0);
    assert_eq!(Level::from_model(&example_level()).get_tile_index(Location::new(39, 0)), 1170);
    assert_eq!(Level::from_model(&example_level()).get_tile_index(Location::new(0, 29)), 29);
    assert_eq!(Level::from_model(&example_level()).get_tile_index(Location::new(39, 29)), 1199);
    assert_eq!(Level::from_model(&example_level()).get_tile_index(Location::new(10, 10)), 310);
}

#[test]
fn test_get_tile_location() {
    assert_eq!(Level::from_model(&example_level()).get_tile_location(0), Location::new(0, 0));
    assert_eq!(Level::from_model(&example_level()).get_tile_location(1170), Location::new(39, 0));
    assert_eq!(Level::from_model(&example_level()).get_tile_location(29), Location::new(0, 29));
    assert_eq!(Level::from_model(&example_level()).get_tile_location(1199), Location::new(39, 29));
    assert_eq!(Level::from_model(&example_level()).get_tile_location(310), Location::new(10, 10));
}
