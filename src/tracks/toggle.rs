use chrono::{DateTime, Utc, Duration};
use rodio::{Sink, decoder, Device};
use std::fs::File;
use crate::sound_board::EventHandler;
use launchpad_rs::colors::{RGColor, rg_color_code};
use launchpad_rs::ButtonEvent;

static PAUSED: RGColor = rg_color_code(2, 0);
static PLAYING: RGColor = rg_color_code(0, 2);
static PRESSED: RGColor = rg_color_code(3, 3);
static PRESSED_RESET: RGColor = rg_color_code(3, 0);

pub struct ToggleTrack<'a> {
    button_name: String,
    pressed_on: Option<DateTime<Utc>>,
    audio_device: &'a Device,
    sink: Sink,
    file: String,
}


impl ToggleTrack<'_> {
    pub fn new(audio_device: &Device, button_name: String, file: String) -> (ToggleTrack, RGColor) {
        let sink = Sink::new(audio_device);
        sink.pause();

        let track = ToggleTrack {
            button_name,
            pressed_on: None,
            audio_device,
            sink,
            file,
        };
        return (track, PAUSED);
    }

    fn reset(&mut self) {
        if !self.sink.empty() {
            println!("{}: recreating sink", self.button_name);
            let is_paused = self.sink.is_paused();
            self.sink.stop();
            self.sink = Sink::new(self.audio_device);

            if self.sink.is_paused() && !is_paused {
                self.sink.play()
            } else if !self.sink.is_paused() && is_paused {
                self.sink.pause()
            }
        }

        println!("{}: loading file: {}", self.button_name, self.file);
        let f = File::open(&self.file).expect("Failed to open file");
        let decoder = decoder::Decoder::new(f).unwrap();
        self.sink.append(decoder)
    }

    fn action_exhausted(&mut self) -> u8 {
        // TODO: add loop parameter
        self.sink.pause();
        self.reset();
        PAUSED
    }

    fn action_toggle_play(&mut self) -> u8 {
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

    fn action_reset(&mut self) -> u8 {
        self.reset();
        PRESSED_RESET
    }
}

impl EventHandler<ButtonEvent, RGColor> for ToggleTrack<'_> {
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
                        Some(self.action_reset())
                    } else {
                        self.pressed_on = None; // Button is not pressed anymore
                        Some(self.action_toggle_play())
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
                return Some(self.action_reset());
            }
        } else if self.sink.empty() { // Detect end of file
            return Some(self.action_exhausted());
        }

        return None;
    }
}
