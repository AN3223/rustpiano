use glutin::ElementState::Released;
use rodio::Device;
use std::io::BufReader;
use std::io;
use std::fs;

// Takes a scan code for a key and returns the corresponding note (if any)
pub fn match_key(key: u32) -> Option<String> {
    let maybe_note = match key {
        11 => Some("48"),
        2 => Some("49"),
        3 => Some("50"),
        4 => Some("51"),
        5 => Some("52"),
        6 => Some("53"),
        7 => Some("54"),
        8 => Some("55"),
        9 => Some("56"),
        10 => Some("57"),
        16 => Some("81"),
        17 => Some("87"),
        18 => Some("69"),
        19 => Some("82"),
        20 => Some("84"),
        21 => Some("89"),
        22 => Some("85"),
        23 => Some("73"),
        24 => Some("79"),
        25 => Some("80"),
        30 => Some("65"),
        31 => Some("83"),
        32 => Some("68"),
        33 => Some("70"),
        34 => Some("71"),
        35 => Some("72"),
        36 => Some("74"),
        37 => Some("75"),
        38 => Some("76"),
        44 => Some("90"),
        45 => Some("88"),
        46 => Some("67"),
        47 => Some("86"),
        48 => Some("66"),
        49 => Some("78"),
        50 => Some("77"),
        _ => None
    };

    if let Some(note) = maybe_note {
        Some(note.to_owned())
    } else {
        None
    }
}


// Takes a keypress and an audio device and plays the corresponding
// key (if any) into the audio device.
pub fn handle_keypress(audio_device: &Device, input: glutin::KeyboardInput) {
    if input.state == Released {
        return;
    }

    let is_capital = input.modifiers.shift;

    if let Some(note) = match_key(input.scancode) {
        let final_note = if is_capital {
            "b".to_owned() + &note
        } else {
            "a".to_owned() + &note
        };

        play_audio(audio_device, &final_note).ok();
    }
}

// Takes a reference to an audio device, and the name of a note, and 
// it will attempt to play the note into the audio device.
pub fn play_audio(audio_device: &Device, note: &str) -> Result<(), io::Error> {
    let file = fs::File::open(format!("sounds/{}.mp3", note))?;

    rodio::play_once(audio_device, BufReader::new(file))
        .unwrap()
        .detach();

    Ok(())
}

// Plays a constant silence in the background, can prevent audio glitches
pub fn play_silence(audio_device: &Device) {
    use rodio::source::{Zero, Source};

    let format = audio_device.default_output_format().unwrap();
    
    rodio::play_raw(
        &audio_device,
        Zero::new(format.channels, format.sample_rate.0).repeat_infinite()
    );
}