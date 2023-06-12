#![no_std]
#![no_main]
use core::ffi::c_void;

use psp::{ self, * };
#[allow(unused_imports)]
use examples::{tui_output, user_inputs, sounds, time, file_system, graphics};

#[path = "./modules"]
mod examples {
    pub mod tui_output;
    pub mod user_inputs;
    pub mod sounds;
    pub mod time;
    pub mod file_system;
    pub mod graphics;
}

module!("PSP programming folder", 1, 0);

fn psp_main() {
    enable_home_button();
    tui_output::output();

    unsafe {
        // let _ = user_inputs::inputs_listener();
        // let _ = sounds::play_sound();
        // let _ = time::base();
        // let _ = file_system::file_sys();
        // let _ = file_system::dir_sys();
        // let _ = graphics::background();
        let _ = graphics::draw_shapes();
    }
}