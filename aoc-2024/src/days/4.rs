use std::slice::Iter;

pub const DAY: &str = "4";

struct Search {
    contents: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

pub fn solve(lines: impl Iterator<Item = String>) {
    let search = Search::new(lines);
    
    let mut xmas_count = 0;
    for x in 0..search.width {
        for y in 0..search.height {
            xmas_count += search.count_xmas(x, y);
        }
    }

    println!("Found {} XMASes", xmas_count);

    // 2nd star

    let mut mas_x_count = 0;
    for x in 0..search.width {
        for y in 0..search.height {
            mas_x_count += if search.is_mas_x(x, y) { 1 } else { 0 };
        }
    }
    
    println!("Found {} MAS crosses", mas_x_count);
}

impl Search {
    fn new(lines: impl Iterator<Item = String>) -> Search {
        let contents: Vec<Vec<char>> = lines
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let height = contents.len();
        let width = if height > 0 { contents[0].len() } else { 0 };
        return Search { contents, height, width };
    }

    fn count_xmas(&self, x: usize, y: usize) -> i32 {
        const LETTERS: [char; 4] = [ 'X', 'M', 'A', 'S' ];
        const DIRECTIONS: [fn(i32, i32) -> (i32, i32); 8] = [
            |x, y| (x-1, y+1), // north-west
            |x, y| (x  , y+1), // north
            |x, y| (x+1, y+1), // north-east
            |x, y| (x+1, y  ), // east
            |x, y| (x+1, y-1), // south-east
            |x, y| (x  , y-1), // south
            |x, y| (x-1, y-1), // south-west
            |x, y| (x-1, y  ), // west
        ];

        return DIRECTIONS
            .iter()
            .map(|dir| self.has_word(x as i32, y as i32, *dir, LETTERS.iter()))
            .map(|is_word| if is_word { 1 } else { 0 })
            .sum();
    }

    fn is_mas_x(&self, x: usize, y: usize) -> bool {
        const LETTERS: [char; 3] = [ 'M', 'A', 'S' ];
        const DIRECTIONS: [fn(i32, i32) -> (i32, i32); 4] = [
            |x, y| (x-1, y+1), // north-west
            |x, y| (x+1, y+1), // north-east
            |x, y| (x+1, y-1), // south-east
            |x, y| (x-1, y-1), // south-west
        ];

        let x: i32 = x as i32;
        let y: i32 = y as i32;
        let points = [
            (x+1, y-1), // south-east
            (x-1, y-1), // south-west
            (x-1, y+1), // north-west
            (x+1, y+1), // north-east
        ];
        
        return points
            .iter()
            .zip(DIRECTIONS)
            .map(|((x, y), dir)| self.has_word(*x, *y, dir, LETTERS.iter()))
            .map(|is_word| if is_word { 1 } else { 0 })
            .sum::<i32>() == 2;
    }

    fn has_word(&self, x: i32, y: i32, direction: fn(i32, i32) -> (i32, i32), mut letters: Iter<char>) -> bool {
        let character = match letters.next() {
            Some(c) => *c,
            None => return true,
        };

        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }

        if self.contents[y as usize][x as usize] != character {
            return false;
        }

        let (new_x, new_y) = direction(x, y);
        return self.has_word(new_x, new_y, direction, letters);
    }
}