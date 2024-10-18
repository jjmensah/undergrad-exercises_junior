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

//     const DEFAULT_STATE_STR: &'static str = "\
// |  1 |  2 |  3 |  4 |
// |  5 |  6 |  7 |  8 |
// |  9 | 10 | 11 | 12 |
// | 13 | 14 | 15 |    |
// ";

//     //#[test]
//     fn test_display_game_state() {
//         let state = GameState::default();
//         assert_eq!(DEFAULT_STATE_STR, format!("{state}"));

//         // TODO: add more tests
//     }

//     //#[test]
//     fn test_validate_game_state() {
//         let mut state = GameState::default();
//         assert!(state.all_tiles_unique());
//         state.set(3, 0, Some(1));
//         assert!(!state.all_tiles_unique());
//         state.set(0, 0, Some(4));
//         assert!(state.all_tiles_unique());

//         // TODO: add more tests
//     }

//     //#[test]
//     fn test_swap() {
//         let mut state = GameState::default();
//         assert_eq!(state.get(2, 3), Some(15));
//         assert_eq!(state.get(3, 3), None);
//         state.swap(2, 3, 3, 3);
//         assert!(state.all_tiles_unique());
//         assert_eq!(state.get(2, 3), None);
//         assert_eq!(state.get(3, 3), Some(15));

//         //
//         state.swap(0, 0, 2, 2);
//         assert!(state.all_tiles_unique());
//         assert_eq!(state.get(0, 0), Some(11));

//         // TODO: add more tests
//     }

//     //#[test]
//     fn test_perform_move() {
//         let mut state = GameState::default();
//         assert!(!state.perform_move(Move::RightToLeft));
//         assert!(!state.perform_move(Move::BottomToTop));
//         assert!(state.perform_move(Move::TopToBottom));
//         assert!(state.all_tiles_unique());
//         assert_eq!(state.get(3, 3), Some(12));
//         assert_eq!(state.get(3, 2), None);
//         assert!(state.perform_move(Move::LeftToRight));
//         assert_eq!(state.get(3, 2), Some(11));
//         assert_eq!(state.get(2, 2), None);

//         // TODO: add more tests
//     }

//     //#[test]
//     fn test_game_state_equality() {
//         let mut state = GameState::default();
//         assert!(!state.perform_move(Move::BottomToTop));
//         assert_eq!(state, GameState::default());
//         assert!(state.perform_move(Move::TopToBottom));
//         let mut state_2 = GameState::default();
//         state_2.set(3, 3, Some(12));
//         state_2.set(3, 2, None);
//         assert_eq!(state, state_2);

//         // TODO: add more tests
//     }

//     //#[test]
//     fn test_perform_moves() {
//         let mut state = GameState::default();
//         assert_eq!(
//             state.perform_moves(&[Move::RightToLeft, Move::BottomToTop, Move::TopToBottom]),
//             1
//         );

//         let mut state = GameState::default();
//         assert_eq!(
//             state.perform_moves(&[Move::TopToBottom, Move::TopToBottom, Move::TopToBottom]),
//             3
//         );
//         let expected = "\
// |  1 |  2 |  3 |    |
// |  5 |  6 |  7 |  4 |
// |  9 | 10 | 11 |  8 |
// | 13 | 14 | 15 | 12 |
// ";
//         assert_eq!(expected, format!("{state}"));

//         // TODO: add more tests
//     }

//     //#[test]
//     fn test_parse_state() {
//         assert_eq!(
//             GameState::from_str(DEFAULT_STATE_STR).unwrap(),
//             GameState::default()
//         );

//         let wrong0 = "\
// |  1 | 22 |  3 |    |
// |  5 |  6 |  7 |  4 |
// |  9 | 10 | 11 |  8 |
// | 13 | 14 | 15 | 12 |
// ";
//         let wrong1 = "\
// |  1 |  2 ,  3 |    |
// |  5 |  6 |  7 |  4 |
// |  9 | 10 | 11 |  8 |
// | 13 | 14 | 15 | 12 |
// ";
//         let wrong2 = "\
// |  1 |  2 |  3 |
// |  5 |  6 |  7 |  4 |
// |  9 | 10 | 11 |  8 |
// | 13 | 14 | 15 | 12 |
// ";
//         let wrong3 = "\
// |  1 |  2 |  3 |    |
// |  5 |  6 |  7 |  4 |
// |  9 | 10 | 11 |  8 |
// | 13 | 14 | 15 | 12 |
// | 13 | 14 | 15 | 12 |
// ";
//         let wrong4 = "\
// |  1 |  2 |  3 |    | 1 |
// |  5 |  6 |  7 |  4 | 1 |
// |  9 | 10 | 11 |  8 | 1 |
// | 13 | 14 | 15 | 12 | 1 |
// ";
//         let wrong5 = "\
// |  1 |  2 |  3 |    |
// |  5 |  2 |  7 |  4 |
// |  9 | 10 | 11 |  8 |
// | 13 | 14 | 15 | 12 |
// ";
//         assert!(GameState::from_str(wrong0).is_none());
//         assert!(GameState::from_str(wrong1).is_none());
//         assert!(GameState::from_str(wrong2).is_none());
//         assert!(GameState::from_str(wrong3).is_none());
//         assert!(GameState::from_str(wrong4).is_none());
//         assert!(GameState::from_str(wrong5).is_none());

//         // TODO: add more tests
//     }

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
