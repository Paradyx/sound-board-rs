use chrono::{DateTime, Utc, Duration};
use rodio::{Sink, decoder, Device};
use std::fs::File;
use crate::sound_board::EventHandler;
use crate::launchpad::{ButtonEvent, Button};
use crate::launchpad::colors::{RGColor, rg_color_code};

static PAUSED: RGColor = rg_color_code(2, 0);
static PLAYING: RGColor = rg_color_code(0, 2);
static PRESSED: RGColor = rg_color_code(3, 3);
static PRESSED_RESET: RGColor = rg_color_code(3, 0);

pub struct ToggleTrack {
    button_name: String,
    pressed_on: Option<DateTime<Utc>>,
    sink: Sink,
    file: String,
}


impl ToggleTrack {
    pub fn new(audio_device: &Device, button_name: String, file: String) -> (ToggleTrack, RGColor) {
        let sink = Sink::new(audio_device);
        sink.pause();

        let track = ToggleTrack {
            button_name,
            pressed_on: None,
            sink,
            file,
        };
        return (track, PAUSED);
    }

    fn reset(&mut self) {
        println!("{}: loading file: {}", self.button_name, self.file);
        let f = File::open(&self.file).expect("Failed to open file");
        let decoder = decoder::Decoder::new(f).unwrap();
        self.sink.append(decoder)
    }

    fn short_button_press(&mut self) -> u8 {
        println!("short");
        if self.sink.empty() {
            self.reset();
        }

        if self.sink.is_paused() {
            println!("{}: playing", self.button_name);
            self.sink.play();
            PLAYING
        } else {
            println!("{}: pausing", self.button_name);
            self.sink.pause();
            PAUSED
        }
    }

    fn long_button_press(&mut self) -> u8 {
        println!("{}: restart", self.button_name);
        self.sink.pause();
        self.reset();

        PRESSED_RESET
    }
}

impl EventHandler<ButtonEvent, RGColor> for ToggleTrack {
    fn on_event(&mut self, event: ButtonEvent) -> Option<RGColor> {
        match event {
            ButtonEvent::Pressed => {
                self.pressed_on = Some(Utc::now());
                Some(PRESSED)
            }
            ButtonEvent::Released => {
                return if let Some(pressed_on) = self.pressed_on.clone() {
                    let time_since_pressed_on = Utc::now().signed_duration_since(pressed_on);
                    if time_since_pressed_on > Duration::milliseconds(1000) {
                        self.pressed_on = None; // Ignore next release event
                        Some(self.long_button_press())
                    } else {
                        self.pressed_on = None; // Button is not pressed anymore
                        Some(self.short_button_press())
                    }
                } else { None }
            }
        }
    }

    fn on_update(&mut self) -> Option<RGColor> {

        if let Some(pressed_on) = self.pressed_on { // Detect long button press
            let time_since_pressed_on = Utc::now().signed_duration_since(pressed_on);
            if time_since_pressed_on > Duration::milliseconds(1000) {
                self.pressed_on = None;
                return Some(self.long_button_press());
            }
        } else if self.sink.empty() { // Detect end of file
            return Some(PAUSED);
        }

        return None;
    }
}
