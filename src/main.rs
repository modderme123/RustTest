struct Constants {

    width: usize,
    height: usize,

    wrap: bool,
    ticks:u32,

    dead: u8,
    live: u8,

}

const CONSTANTS : Constants = Constants {
    width: 50,
    height: 50,

    wrap: true,
    ticks: 10,

    dead: 0b0000000,
    live: 0b0000001,
};

fn main() {
    let mut board: [[u8; CONSTANTS.width]; CONSTANTS.height] =
        [[CONSTANTS.dead; CONSTANTS.width]; CONSTANTS.height];

    let mut index = 0;
    loop {
        update(&board);

        // print the board

        index += 1;
        if index > CONSTANTS.ticks {
            break;
        }
    }
}

fn update(board: &[[u8; CONSTANTS.width]; CONSTANTS.height]){
    //
}
