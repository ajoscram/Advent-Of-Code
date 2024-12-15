use std::collections::HashSet;

pub const DAY: &str = "6";

struct Guard {
    location: (i32, i32),
    direction_index: usize,
}

struct Map {
    width: usize,
    height: usize,
    terrain: Vec<Vec<char>>,
}

pub fn solve(lines: impl Iterator<Item = String>) {
    let mut map = Map::new(lines);
    let mut guard = Guard::new(map.get_guard_start());
    
    let unique_positions = guard
        .traverse(&map)
        .iter()
        .collect::<HashSet<_>>()
        .len();

    println!("The guard visited {} unique positions", unique_positions);
        
    // 2nd star
    
    let loop_location_count = map
        .get_loop_locations()
        .iter()
        .count();
    
    println!("There are {} obstacle locations that result in a loop", loop_location_count);
}

impl Guard {
    const DIRECTIONS: [fn((i32, i32)) -> (i32, i32); 4] = [
        |(x, y)| (x  , y-1), // north
        |(x, y)| (x+1, y  ), // east
        |(x, y)| (x  , y+1), // south
        |(x, y)| (x-1, y  ), // west
    ];
        
    fn new(location: (i32, i32)) -> Guard {
        return Guard { location, direction_index: 0, }
    }
    
    fn traverse(&mut self, map: &Map) -> Vec<(i32, i32, usize)> {
        let mut path: Vec<(i32, i32, usize)> = vec![];
        let mut visited = HashSet::<(i32, i32, usize)>::new();

        while let Some((x, y)) = self.step(map) {            
            let entry = (x, y, self.direction_index);
            path.push(entry);
            if !visited.insert(entry){
                break;
            }
        }

        return path;
    }

    fn step(&mut self, map: &Map) -> Option<(i32, i32)> {
        let direction = Guard::DIRECTIONS[self.direction_index];
        let new_location = direction(self.location);
        
        if let Some(terrain) = map.get_terrain(new_location) {
            if terrain == Map::OBSTACLE {
                self.direction_index = (self.direction_index + 1) % Guard::DIRECTIONS.len();
                return self.step(map);
            }
            
            self.location = new_location;
            return Some(new_location);
        }
        
        return None;
    }
}
    
impl Map {
    const OBSTACLE: char = '#';
    const GUARD_START: char = '^';
    
    fn new(lines: impl Iterator<Item = String>) -> Map {
        let terrain: Vec<Vec<char>> = lines
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let height = terrain.len();
        let width = if height > 0 { terrain[0].len() } else { 0 };
        return Map { width, height, terrain, };
    }

    fn get_guard_start(&self) -> (i32, i32) {
        return self.terrain
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row
                .iter()
                .enumerate()
                .map(move |(x, character)| (x, y, character)))
            .find(|(_, _, terrain)| **terrain == Map::GUARD_START)
            .map(|(x, y, _)| (x as i32, y as i32))
            .unwrap();
    }

    fn get_terrain(&self, location: (i32,i32)) -> Option<char> {
        let (x, y) = location;

        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        return Some(self.terrain[y as usize][x as usize]);
    }

    fn get_loop_locations(&mut self) -> Vec<(usize, usize)> {
        let mut loop_locations = vec!();
        let guard_start = self.get_guard_start();
        for y in 0..self.height {
            for x in 0..self.width {
                
                let previous = self.terrain[y][x];
                if matches!(previous, Map::OBSTACLE | Map::GUARD_START) {
                    continue;
                }

                self.terrain[y][x] = Map::OBSTACLE;

                let mut guard = Guard::new(guard_start);
                let path = guard.traverse(&self);
                if has_path_loop(&path) {
                    loop_locations.push((x, y));
                }
                
                self.terrain[y][x] = previous;
            }
        }

        return loop_locations;
    }
}

fn has_path_loop(path: &Vec<(i32, i32, usize)>) -> bool {
    let last = path.last().unwrap();
    for (i, location) in path.iter().enumerate() {
        if location == last && i < path.len() - 1 {
            return true;
        }
    }

    return false;
}