#![no_std]
#![no_main]
use psp::{ self, * };
#[allow(unused_imports)]
use examples::{tui_output, user_inputs, sounds, time};

#[path = "./modules"]
mod examples {
    pub mod tui_output;
    pub mod user_inputs;
    pub mod sounds;
    pub mod time;
}

module!("PSP programming folder", 1, 0);

fn psp_main() {
    enable_home_button();
    tui_output::output();

    unsafe {
        // let _ = user_inputs::inputs_listener();
        // let _ = sounds::play_sound();
        let _ = time::base();
    }
}
