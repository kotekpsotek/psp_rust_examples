use psp::{sys::{AUDIO_NEXT_CHANNEL, AUDIO_SAMPLE_MAX, AudioFormat}, dprintln};

pub unsafe fn play_sound() {
    let channel = psp::sys::sceAudioChReserve(AUDIO_NEXT_CHANNEL, AUDIO_SAMPLE_MAX as i32, AudioFormat::Stereo);

    /* TODO: .... */
    // let _ = psp::sys::sceAudioOutput(channel, 100, );

    let released = psp::sys::sceAudioChRelease(channel);
    match released {
        0 => (),
        _ => dprintln!("Couldn't play audio")
    }
}
