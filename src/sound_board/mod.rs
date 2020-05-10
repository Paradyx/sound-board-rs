use std::collections::HashMap;
use launchpad_rs::{Launchpad, ButtonEvent, Button};
use launchpad_rs::error::Error;

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

    pub fn register_track_to_button(&mut self, button: L::Button, track: Box<dyn EventHandler<ButtonEvent, L::Color> + 'a>, initial_color: L::Color) -> Result<(), Error>{
        self.launchpad.set_led(&button, initial_color)?;
        self.tracks.insert(button, track);
        Ok(())
    }

    pub fn register_track(&mut self, name: &str, track: Box<dyn EventHandler<ButtonEvent, L::Color> + 'a>, initial_color: L::Color) -> Result<(), Error>{
        self.register_track_to_button(L::Button::from_str(name)?, track, initial_color)
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
