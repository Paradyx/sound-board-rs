use chrono::{DateTime, Utc, Duration};
use rodio::{Sink, Device};
use crate::sound_board::EventHandler;
use launchpad_rs::colors::{RGColor, rg_color_code};
use launchpad_rs::ButtonEvent;
use crate::tracks::auto_buffered;

static PAUSED: RGColor = rg_color_code(2, 0);
static PLAYING: RGColor = rg_color_code(0, 2);
static PRESSED: RGColor = rg_color_code(3, 3);
static PRESSED_RESET: RGColor = rg_color_code(3, 0);

pub struct FireTrack<'a> {
    button_name: String,
    pressed_on: Option<DateTime<Utc>>,
    audio_device: &'a Device,
    sink: Sink,
    file: String,
}


impl FireTrack<'_> {
    pub fn new(audio_device: &Device, button_name: String, file: String) -> (FireTrack, RGColor) {
        let sink = Sink::new(audio_device);

        let mut track = FireTrack {
            button_name,
            pressed_on: None,
            audio_device,
            sink,
            file,
        };

        track.action_reset();

        return (track, PAUSED);
    }

    /// Restarts and pauses the sink. Reuses the existing sink if possible.
    fn reset(&mut self) {
        if !self.sink.empty() {
            println!("{}: recreating sink", self.button_name);
            self.sink.stop();
            self.sink = Sink::new(self.audio_device);
        }
        self.sink.pause();

        println!("{}: loading file: {}", self.button_name, self.file);
        auto_buffered(&self.file, None, &mut self.sink); // TODO: use buffered flag from config
    }

    /// Returns true if the button was pressed longer than a threshold
    fn detect_long_button_press(&mut self) -> bool {
        if let Some(pressed_on) = self.pressed_on { // Detect long button press
            let time_since_pressed_on = Utc::now().signed_duration_since(pressed_on);
            return time_since_pressed_on > Duration::milliseconds(1000)
        } else { false }
    }

    /// Toggle play pause
    fn action_fire(&mut self) {
        self.reset();
        println!("{}: playing", self.button_name);
        self.sink.play();
    }

    /// Restarts the track and pause
    fn action_reset(&mut self) {
        self.reset();
    }
}

impl EventHandler<ButtonEvent, RGColor> for FireTrack<'_> {
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
                        self.action_fire();
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
        } else if self.sink.empty()  {
            self.action_reset();
            Some(PAUSED)
        } else { None }
    }
}