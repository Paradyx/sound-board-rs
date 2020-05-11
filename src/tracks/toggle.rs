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
    r#loop: bool,
}


impl ToggleTrack<'_> {
    pub fn new(audio_device: &Device, button_name: String, file: String, r#loop: bool) -> (ToggleTrack, RGColor) {
        let sink = Sink::new(audio_device);
        sink.pause();

        let track = ToggleTrack {
            button_name,
            pressed_on: None,
            audio_device,
            sink,
            file,
            r#loop,
        };
        return (track, PAUSED);
    }

    /// Restarts the sink. Reuses the existing sink if possible. Does not change start or pause
    /// the sink.
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

    /// Returns true if the button was pressed longer than a threshold
    fn detect_long_button_press(&mut self) -> bool {
        if let Some(pressed_on) = self.pressed_on { // Detect long button press
            let time_since_pressed_on = Utc::now().signed_duration_since(pressed_on);
            return time_since_pressed_on > Duration::milliseconds(1000)
        } else { false }
    }

    /// Toggle play pause
    fn action_toggle_play(&mut self) {
        if self.sink.is_paused() {
            println!("{}: playing", self.button_name);
            self.sink.play();
        } else {
            println!("{}: pausing", self.button_name);
            self.sink.pause();
        }
    }

    /// Restarts the track and starts playing
    fn action_restart(&mut self) {
        self.reset();
        self.sink.play();
    }

    /// Restarts the track and pause
    fn action_reset(&mut self) {
        self.sink.pause();
        self.reset();
    }
}

impl EventHandler<ButtonEvent, RGColor> for ToggleTrack<'_> {
    fn on_event(&mut self, event: ButtonEvent) -> Option<RGColor> {
        match event {
            ButtonEvent::Pressed => { // Just store the press_on time. The action takes place when released
                self.pressed_on = Some(Utc::now());
                Some(PRESSED)
            }
            ButtonEvent::Released => {
                if self.pressed_on.clone().is_some() { // Ignore release event
                    if self.detect_long_button_press() {
                        self.pressed_on = None;
                        self.action_reset();
                    } else {
                        self.pressed_on = None;
                        self.action_toggle_play();
                    }
                }
                // Update the button color
                match self.sink.is_paused() {
                    true => Some(PAUSED),
                    false => Some(PLAYING)
                }
            }
        }
    }

    fn on_update(&mut self) -> Option<RGColor> {
        return if self.detect_long_button_press() {
            self.pressed_on = None; // Ingores next release event
            self.action_reset();
            Some(PRESSED_RESET)
        } else if self.sink.empty() && self.r#loop {
            self.action_restart();
            Some(PLAYING)
        } else if self.sink.empty() && !self.r#loop {
            self.action_reset();
            Some(PAUSED)
        } else { None }
    }
}
