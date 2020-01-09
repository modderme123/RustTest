//use std::io; // provides useful inputting features
//use std::cmp::Ordering; // provides the ordering enums
//use std::process; // for exiting
//use rand::Rng; // provides random number generation

struct Constants {

    width: usize,
    height: usize,

    dead: u8,
    live: u8,

}

const CONSTANTS : Constants = Constants {
    width: 50,
    height: 50,

    dead: 0b0000000,
    live: 0b0000001,
};

fn main() {
    let mut board = [[CONSTANTS.dead; CONSTANTS.width]; CONSTANTS.height];
}

fn update(board: &[[u8; CONSTANTS.width]; CONSTANTS.height]){
    //
}
