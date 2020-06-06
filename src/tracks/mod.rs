mod toggle;
mod fire_and_forget;
mod fire;

pub use toggle::ToggleTrack;
use crate::sound_board::EventHandler;
use crate::settings::TrackConfig;
use crate::tracks::fire_and_forget::FireForgetTrack;
use crate::tracks::fire::FireTrack;
use launchpad_rs::ButtonEvent;
use launchpad_rs::colors::RGColor;
use rodio::{decoder, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn from_config<'a>(button_name: String, config: TrackConfig, audio_device: &'a rodio::Device) -> (Box<dyn EventHandler<ButtonEvent, RGColor> + 'a>, RGColor) {
    let mode = Mode::from(config.mode.as_str());
    return match mode {
        Mode::Toggle => {
            let (track, initial_color) = ToggleTrack::new(audio_device, button_name, config.path, false); // TODO: use loop parameter
            (Box::new(track), initial_color)
        }
        Mode::FireForget => {
            let (track, initial_color) = FireForgetTrack::new(audio_device, button_name, config.path);
            (Box::new(track), initial_color)
        }
        Mode::Fire => {
            let (track, initial_color) = FireTrack::new(audio_device, button_name, config.path);
            (Box::new(track), initial_color)
        }
    };
}

pub enum Mode {
    Toggle,
    FireForget,
    Fire,
    // Hold,
    // Loop,
}

impl From<&str> for Mode {
    fn from(str: &str) -> Self {
        match str {
            "toggle" => Mode::Toggle,
            "fireforget" => Mode::FireForget,
            "fire" => Mode::Fire,
            //  "hold" => Mode::Hold,
            _ => panic!("Illegal mode '{}'", str)
        }
    }
}

const FILE_SIZE_THRESHOLD: u64 = 10 * 1024 * 1024;

fn auto_buffered(file: &String, buffered_flag: Option<bool>, sink: &mut Sink) {
    let f = File::open(file).expect("Failed to open file");
    let meta = f.metadata().expect("Failed to read metadata");
    if meta.len() > FILE_SIZE_THRESHOLD || buffered_flag == Some(true){
        let decoder = decoder::Decoder::new(BufReader::new(f)).unwrap();
        sink.append(decoder)
    } else {
        let decoder = decoder::Decoder::new(f).unwrap();
        sink.append(decoder)
    };

}
