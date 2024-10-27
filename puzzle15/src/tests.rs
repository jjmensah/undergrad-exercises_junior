use super::*;
#[test]

#[cfg(test)]
fn my_tests() {
    

    //#[test]
    fn test_default_game_state() {
        let state = GameState::default();
        assert_eq!(state.get(0, 0), Some(1));
        assert_eq!(state.get(1, 0), Some(2));
        assert_eq!(state.get(2, 0), Some(3));
        assert_eq!(state.get(3, 0), Some(4));
        assert_eq!(state.get(0, 1), Some(5));
        assert_eq!(state.get(1, 1), Some(6));
        assert_eq!(state.get(2, 1), Some(7));
        assert_eq!(state.get(3, 1), Some(8));
        assert_eq!(state.get(0, 2), Some(9));
        assert_eq!(state.get(1, 2), Some(10));
        assert_eq!(state.get(2, 2), Some(11));
        assert_eq!(state.get(3, 2), Some(12));
        assert_eq!(state.get(0, 3), Some(13));
        assert_eq!(state.get(1, 3), Some(14));
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
    }

    test_default_game_state();

//     //#[test]
    fn test_set_game_state() {
        let mut state = GameState::default();
        state.set(0, 2, Some(3));
        assert_eq!(state.get(0, 2), Some(3));
        // TODO: add more tests
        state.set(0, 1 , Some(6)); //[0][1] = 5 --> 6
        assert_eq!(state.get(0, 1), Some(6));

        state.set(0, 3 , Some(14)); //[0][3] = 13 --> 14
        assert_eq!(state.get(0, 3), Some(14));

        state.set(1, 0, Some(3)); //[1][0] = 2 --> 3
        assert_eq!(state.get(1, 0), Some(3));

        state.set(1, 1, Some(7)); //[1][1] = 6 --> 7
        assert_eq!(state.get(1, 1), Some(7));

        state.set(1, 2 , Some(11)); //[1][2] = 10 --> 11
        assert_eq!(state.get(1, 2), Some(11));

        state.set(1, 3 , Some(15)); //[1][3] = 14 --> 15
        assert_eq!(state.get(1, 3), Some(15));

        state.set(2, 0 , Some(4)); //[2][0] = 3 --> 4
        assert_eq!(state.get(2, 0), Some(4));

        state.set(2, 1 , Some(8)); //[2][1] = 7 --> 8
        assert_eq!(state.get(2, 1), Some(8));

        state.set(2, 2 , Some(12)); //[2][2] = 11 --> 12
        assert_eq!(state.get(2, 2), Some(12));

        state.set(2, 3 , None); //[2][3] = 15 --> None
        assert_eq!(state.get(2, 3), None);

        state.set(3, 0 , Some(5)); //[3][0] = 4 --> 5
        assert_eq!(state.get(3, 0), Some(5));
        
        state.set(3, 1 , Some(9)); //[3][1] = 8 --> 9
        assert_eq!(state.get(3, 1), Some(9));
        
        state.set(3, 2 , Some(13)); //[3][2] = 12 --> 13
        assert_eq!(state.get(3, 2), Some(13));

        state.set(3, 3 , Some(15)); //[3][3] = None --> 15
        assert_eq!(state.get(3, 3), Some(15));
    }

    test_set_game_state();

    const DEFAULT_STATE_STR: &'static str = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 | 15 |    |
";

    const TILE_15_MOVED_STATE_STR: &'static str = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 |    | 15 |
";

//Case where None has been moved towards the extreme end of the matrix
        const TILE_1_MOVED_STATE_STR: &'static str = "\
|    |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 | 15 |  1 |
";

    const SCATTERED_TILES_STATE_STR: &'static str = "\
|  3 |    |  8 | 14 |
| 13 |  1 |  4 |  2 |
|  5 | 12 |  6 | 15 |
| 11 | 10 |  9 |  7 |
";

//     //#[test]
    fn test_display_game_state() {
        let state = GameState::default();
        assert_eq!(DEFAULT_STATE_STR, format!("{state}"));

        // TODO: add more tests
        let mut tile_15_moved_state = GameState::default();
        tile_15_moved_state.set(3, 3, Some(15));
        tile_15_moved_state.set(2, 3, None);
        assert_eq!(TILE_15_MOVED_STATE_STR, format!("{tile_15_moved_state}"));

        let mut tile_1_moved_state = GameState::default();
        tile_1_moved_state.set(0, 0, None);
        tile_1_moved_state.set(3, 3, Some(1));
        assert_eq!(TILE_1_MOVED_STATE_STR, format!("{tile_1_moved_state}"));

        let mut scattered_tiles_state = GameState::default();
        scattered_tiles_state.set(0, 0, Some(3));
        scattered_tiles_state.set(1, 0, None);
        scattered_tiles_state.set(2, 0, Some(8));
        scattered_tiles_state.set(3, 0, Some(14));
        scattered_tiles_state.set(0, 1, Some(13));
        scattered_tiles_state.set(1, 1, Some(1));
        scattered_tiles_state.set(2, 1, Some(4));
        scattered_tiles_state.set(3, 1, Some(2));
        scattered_tiles_state.set(0, 2, Some(5));
        scattered_tiles_state.set(1, 2, Some(12));
        scattered_tiles_state.set(2, 2, Some(6));
        scattered_tiles_state.set(3, 2, Some(15));
        scattered_tiles_state.set(0, 3, Some(11));
        scattered_tiles_state.set(1, 3, Some(10));
        scattered_tiles_state.set(2, 3, Some(9));
        scattered_tiles_state.set(3, 3, Some(7));
        assert_eq!(SCATTERED_TILES_STATE_STR, format!("{scattered_tiles_state}"));

    }

    test_display_game_state();

//     //#[test]
    fn test_validate_game_state() {
        let mut state = GameState::default();
        assert!(state.all_tiles_unique());
        state.set(3, 0, Some(1));
        assert!(!state.all_tiles_unique());
        state.set(0, 0, Some(4));
        assert!(state.all_tiles_unique());

        // TODO: add more tests
        state.set(1, 0, Some(3));
        assert!(!state.all_tiles_unique());
        state.set(2, 0, Some(2));
        assert!(state.all_tiles_unique());

        state.set(0, 1, Some(8));
        assert!(!state.all_tiles_unique());
        state.set(3, 1, Some(5));
        assert!(state.all_tiles_unique());
        
        state.set(1, 1, Some(7));
        assert!(!state.all_tiles_unique());
        state.set(2, 1, Some(6));
        assert!(state.all_tiles_unique());

        state.set(0, 2, Some(12));
        assert!(!state.all_tiles_unique());
        state.set(3, 2, Some(9));
        assert!(state.all_tiles_unique());

        state.set(1, 2, Some(11));
        assert!(!state.all_tiles_unique());
        state.set(2, 2, Some(10));
        assert!(state.all_tiles_unique());

        state.set(0, 3, None);
        assert!(!state.all_tiles_unique());
        state.set(3, 3, Some(13));
        assert!(state.all_tiles_unique());

        state.set(1, 3, Some(15));
        assert!(!state.all_tiles_unique());
        state.set(2, 3, Some(14));
        assert!(state.all_tiles_unique());
    }

    test_validate_game_state();

//     //#[test]
    fn test_swap() {
        let mut state = GameState::default();
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
        state.swap(2, 3, 3, 3);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(2, 3), None);
        assert_eq!(state.get(3, 3), Some(15));

        //
        state.swap(0, 0, 2, 2);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 0), Some(11));
        assert_eq!(state.get(2, 2), Some(1));

        // TODO: add more tests

        state.swap(0, 1, 1, 1);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 1), Some(6));
        assert_eq!(state.get(1, 1), Some(5));

        state.swap(0, 2, 1, 2);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 2), Some(10));
        assert_eq!(state.get(1, 2), Some(9));

        state.swap(0, 3, 1, 3);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 3), Some(14));
        assert_eq!(state.get(1, 3), Some(13));

        state.swap(2, 1, 3, 1);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(2, 1), Some(8));
        assert_eq!(state.get(3, 1), Some(7));

        state.swap(3, 2, 3, 0);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(3, 2), Some(4));
        assert_eq!(state.get(3, 0), Some(12));

        state.swap(1, 0, 2, 0);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(1, 0), Some(3));
        assert_eq!(state.get(2, 0), Some(2));

    }

    test_swap();

//     //#[test]
    fn test_perform_move() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::RightToLeft));
        assert!(!state.perform_move(Move::BottomToTop));
        assert!(state.perform_move(Move::TopToBottom));
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(3, 3), Some(12));
        assert_eq!(state.get(3, 2), None);
        assert!(state.perform_move(Move::LeftToRight));
        assert_eq!(state.get(3, 2), Some(11));
        assert_eq!(state.get(2, 2), None);

        // TODO: add more tests
        
        assert!(state.perform_move(Move::BottomToTop));
        assert_eq!(state.get(2, 2), Some(15));
        assert_eq!(state.get(2, 3), None);

        assert!(state.perform_move(Move::RightToLeft));
        assert_eq!(state.get(2, 3), Some(12));
        assert_eq!(state.get(3, 3), None);

        assert!(state.all_tiles_unique());
    }

    test_perform_move();

//     //#[test]
    fn test_game_state_equality() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::BottomToTop));
        assert_eq!(state, GameState::default());
        assert!(state.perform_move(Move::TopToBottom));
        let mut state_2 = GameState::default();
        state_2.set(3, 3, Some(12));
        state_2.set(3, 2, None);
        assert_eq!(state, state_2);

        // TODO: add more tests

        assert!(state.perform_move(Move::BottomToTop));
        state_2.set(3, 3, None);
        state_2.set(3, 2, Some(12));
        assert_eq!(state, state_2);

        assert!(state.perform_move(Move::LeftToRight));
        state_2.set(2, 3, None);
        state_2.set(3, 3, Some(15));
        assert_eq!(state, state_2);

        assert!(state.perform_move(Move::RightToLeft));
        state_2.set(2, 3, Some(15));
        state_2.set(3, 3, None);
        assert_eq!(state, state_2);

    }

    test_game_state_equality();

//     //#[test]
    fn test_perform_moves() {
        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[Move::RightToLeft, Move::BottomToTop, Move::TopToBottom]),
            1
        );

        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[Move::TopToBottom, Move::TopToBottom, Move::TopToBottom]),
            3
        );
        let expected = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        assert_eq!(expected, format!("{state}"));

        // TODO: add more tests

        let mut state_2 = GameState::default();

        assert_eq!(state_2.perform_moves(&[Move::TopToBottom, Move::RightToLeft,
            Move::LeftToRight, Move::BottomToTop, Move::BottomToTop]) , 3);
        
        let expected_2 = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 15 | 11 |
| 13 | 14 |    | 12 |
";

        assert_eq!(expected_2, format!("{state_2}"));


        let mut state_3 = GameState::default();

        assert_eq!(state_3.perform_moves(&[Move::BottomToTop, Move::RightToLeft,
            Move::LeftToRight, Move::TopToBottom, Move::RightToLeft,
            Move::RightToLeft, Move::TopToBottom]) , 4);
        
        let expected_3 = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |    |
|  9 | 10 | 12 |  8 |
| 13 | 14 | 11 | 15 |
";

        assert_eq!(expected_3, format!("{state_3}"));

    }

    test_perform_moves();

//     //#[test]
    fn test_parse_state() {
        assert_eq!(
            GameState::from_str(DEFAULT_STATE_STR).unwrap(),
            GameState::default()
        );

        let wrong0 = "\
|  1 | 22 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong1 = "\
|  1 |  2 ,  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong2 = "\
|  1 |  2 |  3 |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong3 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
| 13 | 14 | 15 | 12 |
";
        let wrong4 = "\
|  1 |  2 |  3 |    | 1 |
|  5 |  6 |  7 |  4 | 1 |
|  9 | 10 | 11 |  8 | 1 |
| 13 | 14 | 15 | 12 | 1 |
";
        let wrong5 = "\
|  1 |  2 |  3 |    |
|  5 |  2 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";

        assert!(GameState::from_str(wrong0).is_none());
        assert!(GameState::from_str(wrong1).is_none());
        assert!(GameState::from_str(wrong2).is_none());
        assert!(GameState::from_str(wrong3).is_none());
        assert!(GameState::from_str(wrong4).is_none());
        assert!(GameState::from_str(wrong5).is_none());

        // TODO: add more tests
        let incorrect_6 = "\
|  1 |  2 |  3 | 16 |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";//Parsing not possible

        let incorrect_7 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 |  4 | 15 | 12 |
";//GameState contains duplicate tiles

        let incorrect_8 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |    |
| 13 | 14 | 15 | 12 |
";//GameState contains two None (empty) tiles making it invalid

        let incorrect_9 = "\
|| 2 |  3 |    |  1 |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";//Parsing not possible

        let incorrect_10 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 18 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";//GameState contains an invalid tile (Tile 18)

        let incorrect_11 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
| 13 | 14 | 15 | 12 |
";//Parsing not possible

        assert!(GameState::from_str(incorrect_6).is_none());
        assert!(GameState::from_str(incorrect_7).is_none());
        assert!(GameState::from_str(incorrect_8).is_none());
        assert!(GameState::from_str(incorrect_9).is_none());
        assert!(GameState::from_str(incorrect_10).is_none());
        assert!(GameState::from_str(incorrect_11).is_none());

    }

    test_parse_state();
//     //#[test]
//     fn test_find_shortest_path() {
//         let expected_moves = [Move::TopToBottom, Move::TopToBottom, Move::TopToBottom];
//         let mut state = GameState::default();
//         assert_eq!(state.perform_moves(&expected_moves), 3);

//         let actual_moves = find_shortest_path(GameState::default(), state);
//         assert_eq!(actual_moves.len(), 3);
//         assert_eq!(actual_moves, expected_moves);

//         // TODO: add more tests
//     }
}
