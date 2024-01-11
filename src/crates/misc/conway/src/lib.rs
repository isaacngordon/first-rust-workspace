pub mod game_of_life;


//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 0 1 0 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 1 0 1 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 1 0 1 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0   --> 16 steps --> 0 0 0 0 0 0 1 0 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 0 0 0 0 0 0 0 0
//  0 0 0 0 0 0 1 0 0 0 0 0 0                    0 1 1 0 0 0 0 0 0 0 1 1 0
//  0 0 0 0 0 1 1 1 0 0 0 0 0                    1 0 0 1 0 0 0 0 0 1 0 0 1
//  0 0 0 0 0 1 0 1 0 0 0 0 0                    0 1 1 0 0 0 0 0 0 0 1 1 0
//  0 0 0 0 0 0 1 0 0 0 0 0 0                    0 0 0 0 0 0 0 0 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 0 1 0 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 1 0 1 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 1 0 1 0 0 0 0 0
//  0 0 0 0 0 0 0 0 0 0 0 0 0                    0 0 0 0 0 0 1 0 0 0 0 0 0

const TEST_1_START: [[bool; 13]; 13] = [
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, true, false, false, false, false, false, false],
    [false, false, false, false, false, true, true, true, false, false, false, false, false],
    [false, false, false, false, false, true, false, true, false, false, false, false, false],
    [false, false, false, false, false, false, true, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false]
];

const TEST_1_END: [[bool; 13]; 13] = [
    [false, false, false, false, false, false, true, false, false, false, false, false, false],
    [false, false, false, false, false, true, false, true, false, false, false, false, false],
    [false, false, false, false, false, true, false, true, false, false, false, false, false],
    [false, false, false, false, false, false, true, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, true, true, false, false, false, false, false, false, false, true, true, false],
    [true, false, false, true, false, false, false, false, false, true, false, false, true],
    [false, true, true, false, false, false, false, false, false, false, true, true, false],
    [false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, true, false, false, false, false, false, false],
    [false, false, false, false, false, true, false, true, false, false, false, false, false],
    [false, false, false, false, false, true, false, true, false, false, false, false, false],
    [false, false, false, false, false, false, true, false, false, false, false, false, false]
];

const TEST_1_STEPS: i32 = 16;

#[cfg(test)]
mod tests {
    use crate::{TEST_1_START, TEST_1_END, TEST_1_STEPS};

    use super::game_of_life::*;

    fn test_game_of_life(n: usize, start: Vec<Vec<bool>>, end: Vec<Vec<bool>>, steps: i32, algo: fn(&mut Slice)) {
        assert_eq!(start.len(), end.len(), "Start and end states must have the same number of rows");
        assert_eq!(start[0].len(), end[0].len(), "Start and end states must have the same number of columns");
        assert_eq!(start.len(), n, "Start state must have the same number of rows as the game size, {}, but it was {}", n, start.len());
        assert_eq!(end.len(), n, "End state must have the same number of rows as the game size, {}, but it was {}", n, end.len());
        assert_eq!(start[0].len(), n, "Start state must have the same number of columns as the game size, {}, but it was {}", n, start[0].len());
        assert_eq!(end[0].len(), n, "End state must have the same number of columns as the game size, {}, but it was {}", n, end[0].len());

        let mut game_actual = Slice::new(n);
        let mut game_expected = Slice::new(n);
        game_actual.set_cells(start.into_iter().flatten().collect());
        game_expected.set_cells(end.into_iter().flatten().collect());

        println!("Initial Game: {}", game_actual);
        for _ in 0..steps {
            algo(&mut game_actual);
        }
        println!("Final Game Expected:\n===========================================\n{}\n================================================", 
        game_expected.to_hex_string());
        println!("Final Game Actual:\n===========================================\n{}\n\n================================================", 
        game_actual.to_hex_string());

        assert_eq!(game_actual.to_hex_string(), game_expected.to_hex_string(), "Game of Life did not match expected result");
    }

    #[test]
    fn test_1_naive() {
        test_game_of_life(
            13, 
            TEST_1_START.iter().map(|row| row.to_vec()).collect(), 
            TEST_1_END.iter().map(|row| row.to_vec()).collect(), 
            TEST_1_STEPS, 
            Slice::next_generation_naive);
    }

    #[test]
    fn test_1_naive_optimized() {
        test_game_of_life(
            13, 
            TEST_1_START.iter().map(|row| row.to_vec()).collect(), 
            TEST_1_END.iter().map(|row| row.to_vec()).collect(), 
            TEST_1_STEPS, 
            Slice::next_generation_naive_optimized);
    }
}
