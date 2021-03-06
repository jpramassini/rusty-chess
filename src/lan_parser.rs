use std::str;

use crate::{rank_and_file_to_index, Board, Space};

pub fn letter_to_file(letter: char) -> u8 {
    match letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => u8::MAX,
    }
}

pub fn parse_lan_string_to_coords(lan_string: &str) -> ((u8, u8), (u8, u8)) {
    let origin_file = letter_to_file(lan_string.as_bytes()[0] as char);
    let origin_rank: u8 = (lan_string.as_bytes()[1] as char)
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();
    let dest_file = letter_to_file(lan_string.as_bytes()[2] as char);
    let dest_rank: u8 = (lan_string.as_bytes()[3] as char)
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();

    return ((origin_file, origin_rank), (dest_file, dest_rank));
}

pub fn get_lan_spaces_from_board(lan_string: &str, board: &Board) -> (Space, Space) {
    let (start, end) = parse_lan_string_to_coords(lan_string);

    let start_space_index = rank_and_file_to_index(start.0, start.1);
    let end_space_index = rank_and_file_to_index(end.0, end.1);

    return (
        board.squares[start_space_index],
        board.squares[end_space_index],
    );
}
