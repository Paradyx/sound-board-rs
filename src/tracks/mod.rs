mod toggle;

pub use toggle::ToggleTrack;
use crate::launchpad::{ButtonEvent, Button};
use crate::sound_board::EventHandler;
use crate::settings::TrackConfig;
use crate::launchpad::colors::RGColor;

pub type Track<C> = dyn EventHandler<ButtonEvent, C>;


pub fn from_config(button_name: String, config: TrackConfig, audio_device: &rodio::Device) -> (Box<Track<RGColor>>, RGColor) {
    let mode = Mode::from(config.mode.as_str());

    let (track, initial_color) = match mode {
        Mode::Toggle => ToggleTrack::new(audio_device, button_name, config.path),
    };
    return (Box::new(track), initial_color);
}


pub enum Mode {
    Toggle,
    // FireForget,
    // Fire,
    // Hold,
}

impl From<&str> for Mode {
    fn from(str: &str) -> Self {
        match str {
            "toggle" => Mode::Toggle,
            //  "fireforget" => Mode::FireForget,
            //  "fire" => Mode::Fire,
            //  "hold" => Mode::Hold,
            _ => panic!("Illegal mode '{}'", str)
        }
    }
}
