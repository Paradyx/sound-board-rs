mod settings;
mod sound_board;
mod tracks;

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use crate::settings::{Settings};
use crate::sound_board::{SoundBoard};
use launchpad_rs::error::Error;
use launchpad_rs::launchpad_mini_mk_ii::LaunchpadMiniMk2;
use portmidi::PortMidi;


// Usage: boards-rs CONFIG
// Launch pad must be in session mode.
fn main() {
    let settings = Settings::new().expect("Failed to read settings");
    println!("Loaded settings.");

    println!("Initializing midi context");
    let midi_context = PortMidi::new().expect("Failed to get midi context");

    println!("Initializing launchpad");
    let launchpad = LaunchpadMiniMk2::new(&midi_context);

    println!("Initializing audio devices");
    let audio_device = rodio::default_output_device().expect("Initializing default audio device failed");

    println!("Initializing boards");
    let mut board: SoundBoard<LaunchpadMiniMk2> = SoundBoard::new(launchpad);

    for (button_name, track_config) in settings.tracks {
        let (track, initial_color) = tracks::from_config(button_name.clone(), track_config, &audio_device);
        board.register_track(button_name.as_str(), track, initial_color).expect("Failed to register track to button");
    }

    println!("Finished loading");
    main_loop(settings.fps, move |time: SystemTime| {
        match time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(_t) => {} //println!("{}, Event handler here!", n.as_secs()),
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

