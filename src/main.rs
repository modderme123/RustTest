//use std::io; // provides useful inputting features
//use std::cmp::Ordering; // provides the ordering enums
//use std::process; // for exiting
//use rand::Rng; // provides random number generation

struct Constants {

    // bitwise combinable constants
    empty_tile: u8,
    live_tile: u8,

    // other constants
    width: usize,
    height: usize,

}

fn main() {
    const CONSTANTS : Constants = Constants {
        empty_tile: 0b0000000,
        live_tile:  0b0000001,

        width: 100,
        height: 100,
    };

    let mut board = [[CONSTANTS.empty_tile; CONSTANTS.width]; CONSTANTS.height];
}

// TODO fix the 2d array input
fn update(mut board: &[[u8; 4]; 4]){
    //
}
