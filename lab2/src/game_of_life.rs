use crate::Screen;
use crate::vga_buf::AsciiChar;

const GoL_MAP: [&str; 25] = [
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                    x                                           ",
    "                                  x x                                           ",
    "                        xx      xx            xx                                ",
    "                       x   x    xx            xx                                ",
    "            xx        x     x   xx                                              ",
    "            xx        x   x xx    x x                                           ",
    "                      x     x       x                                           ",
    "                       x   x                                                    ",
    "                        xx                                                      ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                ",
    "                                                                                "
];
const GoL_MAP2: [&str; 25] = [
    "                                                                                ",
    "  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx       ",
    "  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx       ",
    "  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx       ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    " x                                  xxx                                  xxx    ",
    "  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx       ",
    "  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx       ",
    "  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx   xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx       ",
    "                                                                                "
];
pub const BUF_HEIGHT: u32 = 25;
pub const BUF_WIDTH: u32 = 80;

pub fn print_new_step(field:&[[u8;80];25],screen:&mut Screen){
    for i in 0..field.len(){
        for j in 0..field[0].len(){
            screen.write_symbol_s((i as u32)*BUF_WIDTH+(j as u32),field[i][j]);
        }
    }
}

pub fn wait(){
    for i in 0..25555{}
}

pub fn get_all_neighbours(field:[[u8;80];25],row:isize, column:isize)->u32 {
    let mut count = 0;
    for i in row-1..row+2{
        for j in column -1.. column+2{
            if i == row && j == column{
                continue;
            }
            if i >= 0 && i < BUF_HEIGHT as isize && j>=0 && j < BUF_WIDTH as isize{
                if field[i as usize][j as usize] == b'x'{
                    count +=1;
                }
            }
        }
    }

    return count;
}

pub fn game_of_life(screen: &mut Screen) {
    let mut current_gen: [[u8; 80]; 25] = [[0; 80]; 25];
    for i in 0..GoL_MAP.len() {
        for (j, byte) in GoL_MAP[i].bytes().enumerate() {
            current_gen[i][j] = byte;
        }
    }

    print_new_step(&current_gen, screen);

    loop{
        wait();
        let mut next_gen:[[u8;80];25] = [[0;80];25];

        for i in 0..current_gen.len(){
            for j in 0..current_gen[0].len(){
                let mut count_of_neighbours = get_all_neighbours(current_gen,i as isize,j as isize);

                if current_gen[i][j] == b'x' && (count_of_neighbours == 3 || count_of_neighbours == 2){
                    next_gen[i][j] = b'x';
                }
                else if current_gen[i][j] == b' ' && count_of_neighbours == 3{
                    next_gen[i][j] = b'x';
                }
                else{
                    next_gen[i][j] = b' ';
                }
            }
        }
        current_gen = next_gen;
        print_new_step(&current_gen,screen);
    }
}