mod toggle;
mod fire_and_forget;

pub use toggle::ToggleTrack;
use crate::launchpad::{ButtonEvent, Button};
use crate::sound_board::EventHandler;
use crate::settings::TrackConfig;
use crate::launchpad::colors::RGColor;
use crate::tracks::fire_and_forget::FireForgetTrack;

pub type Track<C> = dyn EventHandler<ButtonEvent, C>;


pub fn from_config<'a>(button_name: String, config: TrackConfig, audio_device: &'a rodio::Device) -> (Box<dyn EventHandler<ButtonEvent, RGColor> + 'a> , RGColor) {
    let mode = Mode::from(config.mode.as_str());

    return match mode {
        Mode::Toggle => {
            let (track, initial_color) = ToggleTrack::new(audio_device, button_name, config.path);
             (Box::new(track), initial_color)
        },
        Mode::FireForget => {
            let (track, initial_color) = FireForgetTrack::new(audio_device, button_name, config.path);
            (Box::new(track), initial_color)
        }
    };

}


pub enum Mode {
    Toggle,
    FireForget,
    // Fire,
    // Hold,
    // Loop,

}

impl From<&str> for Mode {
    fn from(str: &str) -> Self {
        match str {
            "toggle" => Mode::Toggle,
            "fireforget" => Mode::FireForget,
            //  "fire" => Mode::Fire,
            //  "hold" => Mode::Hold,
            _ => panic!("Illegal mode '{}'", str)
        }
    }
}
