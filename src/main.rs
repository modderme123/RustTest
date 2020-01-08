//use std::io; // provides useful inputting features
//use std::cmp::Ordering; // provides the ordering enums
//use std::process; // for exiting
//use rand::Rng; // provides random number generation

struct Constants {

    width: usize,
    height: usize,

}

const CONSTANTS : Constants = Constants {
    width: 50,
    height: 50,
};

enum Tiles {

    Empty,
    Live,

}

fn main() {
    let mut board = [[CONSTANTS.empty_tile; CONSTANTS.width]; CONSTANTS.height];
}

fn update(board: &[[u8; CONSTANTS.width]; CONSTANTS.height]){
    //
}
