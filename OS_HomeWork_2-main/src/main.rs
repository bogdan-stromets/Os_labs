#![no_std] 
#![no_main] 

mod vga_buf;
mod game_of_life;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write;
use crate::game_of_life::game_of_life;
use crate::vga_buf::{Alignment, Color, DEFAULT_COLOR, Screen};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut screen = Screen::new(Color::Black,Color::Pink, Alignment::Center);

    // 1 task
    /*
    for i in 0..100{
        write!(screen, "number {}\n", i);
    }
*/
    // 2 task
    game_of_life(&mut screen);

    loop {}
}