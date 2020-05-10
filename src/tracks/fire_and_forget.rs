use rodio::{Sink, decoder, Device};
use std::fs::File;
use crate::sound_board::EventHandler;
use launchpad_rs::colors::{RGColor, rg_color_code};
use launchpad_rs::ButtonEvent;

static DEFAULT: RGColor = rg_color_code(2, 0);
static PRESSED: RGColor = rg_color_code(3, 3);

pub struct FireForgetTrack<'a> {
    button_name: String,
    audio_device: &'a Device,
    file: String
}


impl FireForgetTrack<'_> {
    pub fn new(audio_device: &Device, button_name: String, file: String) -> (FireForgetTrack, RGColor) {
        let track = FireForgetTrack {
            button_name,
            audio_device,
            file,
        };
        return (track, DEFAULT);
    }

    fn fire(&mut self) {
        println!("short");
        let sink = Sink::new(self.audio_device);
        println!("{}: loading file: {}", self.button_name, self.file);
        let f = File::open(&self.file).expect("Failed to open file");
        let decoder = decoder::Decoder::new(f).unwrap();
        sink.append(decoder);
        sink.detach();
    }
}

impl EventHandler<ButtonEvent, RGColor> for FireForgetTrack<'_> {
    fn on_event(&mut self, event: ButtonEvent) -> Option<RGColor> {
        match event {
            ButtonEvent::Pressed => {
                self.fire();
                Some(PRESSED)
            }
            ButtonEvent::Released => {
                Some(DEFAULT)
            }
        }
    }

    fn on_update(&mut self) -> Option<RGColor> {
        // We forgot about this one ;)
        return None;
    }
}
