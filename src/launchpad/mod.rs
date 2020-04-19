use crate::launchpad::error::Error;
use crate::launchpad::colors::Color;
use std::hash::Hash;

pub mod launchpad_mini;
// pub mod lauchpad_mk2;

pub mod error;
pub mod colors;

pub trait Button: Sized + Copy + Clone + Eq + Hash {
    /**
     * Return the name of the button
     */
    fn to_str(&self) -> &str;

    fn from_str(name: &str) -> Result<Self, Error>;

    /*
    /**
     * Return the id of the button. All Ids between the lowest and the highest are in use.
     */
    fn to_id(&self) -> usize;

    /**
     * Create a button from its id.
     * Throws a InvalidButtonIdError if the Id is unknown.
     */
    fn from_id(id: usize) -> Result<Self, Error>;
    */
    /**
     * The highest possible Id for a button
     */
    const TOTAL: usize;
}

pub enum ButtonEvent {
    Pressed,
    Released,
}

pub trait Launchpad
{
    type But: Button;
    type Col: Color;
    /**
     * Set the [button] on this launchpad to a given [color].
     */
    fn set_led(&mut self, button: &Self::But, color: Self::Col) -> Result<(), Error>;

    /**
     * Turn of the led of [button] on this launchpad.
     */
    fn reset_led(&mut self, button: &Self::But) -> Result<(), Error>;

    fn set_all(&mut self, color: Self::Col) -> Result<(), Error>;
    fn reset_all(&mut self) -> Result<(), Error>;

    fn poll(&mut self) -> Result<Vec<(Self::But, ButtonEvent)>, Error>;
}

