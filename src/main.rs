mod settings;
mod launchpad;
mod sound_board;
mod tracks;
mod midi;

use std::path::Path;
use std::thread::sleep;
use std::process::exit;
use std::time::{Duration, SystemTime, SystemTimeError};
use portmidi::PortMidi;
use crate::settings::{Settings, TrackConfig};
use crate::launchpad::launchpad_mini::{LaunchpadMini, LaunchpadMiniButton};
use crate::sound_board::{SoundBoard, EventHandler};
use crate::launchpad::{ButtonEvent, Button};
use crate::tracks::{Track, from_config};
use crate::launchpad::colors::RGColor;
use core::fmt;
use crate::launchpad::error::Error;
use std::collections::HashMap;


// Usage: boards-rs CONFIG
// Launch pad must be in session mode.
fn main() {
    let settings = Settings::new().expect("Failed to read settings");
    println!("Loaded settings.");

    println!("Initializing midi context");
    let midi_context = PortMidi::new().expect("Failed to get midi context");

    println!("Initializing launchpad");
    let launchpad = LaunchpadMini::new(&midi_context);

    println!("Initializing audio devices");
    let audio_device = rodio::default_output_device().expect("Initializing default audio device failed");

    println!("Initializing boards");
    let mut board: SoundBoard<LaunchpadMini> = SoundBoard::new(launchpad);

    for (button_name, track_config) in settings.tracks {
        let (track, initial_color) = tracks::from_config(button_name.clone(), track_config, &audio_device);
        board.register_track( button_name.as_str(), track , initial_color );

    }

    println!("Finished loading");
    main_loop(settings.fps , move |time: SystemTime| {
        match time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {} //println!("{}, Event handler here!", n.as_secs()),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        Ok(board.mix()?)
    });
}

fn main_loop<F>(fps: u8, mut f: F) -> !
    where F: FnMut(SystemTime) -> Result<(), Error>
{
    let frame_duration = Duration::from_millis(1000 / fps as u64);
    let mut now = SystemTime::now();
    loop {
        let elapsed = now.elapsed().expect("Ending main loop due to error");
        if elapsed > frame_duration {
            f(now).expect("Ending main loop due to error");
            now = SystemTime::now();
        } else {
            sleep(frame_duration - elapsed);
        }
    }
}

