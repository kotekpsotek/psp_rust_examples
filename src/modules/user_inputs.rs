use psp::{ sys::{ self, SceCtrlData, CtrlButtons }, dprintln };

/// Listens for inputs into TUI
pub unsafe fn inputs_listener() {
    // Setup listening for inputs buttons
    sys::sceCtrlSetSamplingCycle(0);
    sys::sceCtrlSetSamplingMode(sys::CtrlMode::Analog);

    // Storage for recived inputs datas
    let ctrl_datas = &mut SceCtrlData::default();

    // Start listening continuosly for inputs
    loop {
        // Assign potentialy recived datas to variable
        sys::sceCtrlReadBufferPositive(ctrl_datas, 1);

        // Destructurize to simpre read
        let SceCtrlData { buttons, .. } = ctrl_datas;

        // Perform action sutable for specific clicked button
        match *buttons {
            CtrlButtons::CIRCLE => dprintln!("Circle was clicked"),
            CtrlButtons::CROSS => dprintln!("Cross was clicked"),
            _ => ()
        }
    }
}
