pub mod game_of_life;

#[cfg(test)]
mod tests {
    use super::game_of_life::*;

    #[test]
    fn test_game_of_life() {
        
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
         

        // create a 13x13 grid game, with the above pattern as the initial state
        let start_shape_1 = vec![
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false, false, false, false],
            vec![false, false, false, false, false, true, true, true, false, false, false, false, false],
            vec![false, false, false, false, false, true, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false]
        ];

        let end_state_1 = vec![
            vec![false, false, false, false, false, false, true, false, false, false, false, false, false],
            vec![false, false, false, false, false, true, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, true, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, true, true, false, false, false, false, false, false, false, true, true, false],
            vec![true, false, false, true, false, false, false, false, false, true, false, false, true],
            vec![false, true, true, false, false, false, false, false, false, false, true, true, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false, false, false, false],
            vec![false, false, false, false, false, true, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, true, false, true, false, false, false, false, false],
            vec![false, false, false, false, false, false, true, false, false, false, false, false, false]
        
        ];
        
        let mut game_actual = Slice::new(13);
        let mut game_expected = Slice::new(13);
        game_actual.set_cells(start_shape_1.into_iter().flatten().collect());
        game_expected.set_cells(end_state_1.into_iter().flatten().collect());

        println!("Initial Game: {}", game_actual);
        for i in 0..16 {
            game_actual.next_generation_naive();
            println!("Frame: {}\n{}", i+1, game_actual);
        }
        println!("Final Game Expected:\n===========================================\n{}\n\n================================================", game_expected);
        println!("Final Game Actual:\n===========================================\n{}\n\n================================================", game_actual);

        assert_eq!(game_actual.to_hex_string(), game_expected.to_hex_string());

    }
}
