pub trait Color : Sized + Copy {}

pub type RGColor = u8;

impl Color for RGColor {}

/**
 * Allowed values 0..=3
 */
pub const fn rg_color_code(red: u8, green: u8) -> RGColor {
    ((green & 0b00000011) << 4) + (red & 0b00000011)
}

