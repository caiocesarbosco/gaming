pub mod map {

    use crate::graphical::tile::tile::Tile;
    use crate::constants::constants::windows_resolutions::WORLD_SIZE;

    type Map = Vec<Vec<Tile>>;

    pub fn make_map() -> Map {
        let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
        map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
        map
    }
}