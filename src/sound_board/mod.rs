use crate::launchpad::{Button, Launchpad, ButtonEvent};
use std::thread::sleep;
use std::time::Duration;
use crate::launchpad::colors::Color;
use crate::launchpad::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;
use std::collections::btree_map::Entry::Vacant;

pub trait EventHandler<E, R>
    where E: Sized, // Event
          R: Sized, // Reaction
{
    fn on_event(&mut self, event: E) -> Option<R>;
    fn on_update(&mut self) -> Option<R>;
}

pub struct SoundBoard<'a, L>
    where L: Launchpad,
{
    launchpad: L,
    tracks: HashMap<L::Button, Box<dyn EventHandler<ButtonEvent, L::Color> + 'a>>,
}

impl<'a, L> SoundBoard<'a, L>
    where L: Launchpad,
{
    pub fn new(launchpad: L) -> Self {
        return SoundBoard {
            launchpad,
            tracks: HashMap::with_capacity(L::Button::TOTAL),
        };
    }

    pub fn register_track_to_button(&mut self, button: L::Button, track: Box<dyn EventHandler<ButtonEvent, L::Color> + 'a>, initial_color: L::Color) {
        self.launchpad.set_led(&button, initial_color);
        self.tracks.insert(button, track);
    }

    pub fn register_track(&mut self, name: &str, track: Box<dyn EventHandler<ButtonEvent, L::Color> + 'a>, initial_color: L::Color) {
        match L::Button::from_str(name) {
            Ok(button) => self.register_track_to_button(button, track, initial_color),
            Err(e) => println!("An error occurred when registering: {}", e),
        }
    }

    pub fn mix(&mut self) -> Result<(), Error> {
        let event_queue = self.launchpad.poll().expect("Polling launchpad events failed");

        for (button, event) in event_queue {
            match self.tracks.get_mut(&button) {
                Some(track) => {
                    if let Some(new_color) = track.on_event(event) {
                        self.launchpad.set_led(&button, new_color)?;
                    }
                },
                None => println!("received event for empty track {}", button.to_str()),
            };
        }

        for (button, track) in self.tracks.iter_mut() {
            if let Some(new_color) = track.on_update() {
                self.launchpad.set_led(button, new_color)?;
            }
        };
        Ok(())
    }
}
