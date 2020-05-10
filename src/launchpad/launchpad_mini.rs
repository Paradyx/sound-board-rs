use portmidi::{InputPort, OutputPort, PortMidi, MidiMessage, MidiEvent};
use crate::launchpad::{Launchpad, ButtonEvent, Button};
use crate::launchpad::error::Error;
use crate::midi;
use crate::launchpad::colors::{RGColor, rg_color_code};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LaunchpadMiniButton {
    NoteButton(u8),
    ControlButton(u8),
}

impl Button for LaunchpadMiniButton {
    fn to_str(&self) -> &str {
        match self {
            LaunchpadMiniButton::ControlButton(104) => "o1",
            LaunchpadMiniButton::ControlButton(105) => "o2",
            LaunchpadMiniButton::ControlButton(106) => "o3",
            LaunchpadMiniButton::ControlButton(107) => "o4",
            LaunchpadMiniButton::ControlButton(108) => "o5",
            LaunchpadMiniButton::ControlButton(109) => "o6",
            LaunchpadMiniButton::ControlButton(110) => "o7",
            LaunchpadMiniButton::ControlButton(111) => "o8",
            LaunchpadMiniButton::NoteButton(0x00) => "a1",
            LaunchpadMiniButton::NoteButton(0x01) => "a2",
            LaunchpadMiniButton::NoteButton(0x02) => "a3",
            LaunchpadMiniButton::NoteButton(0x03) => "a4",
            LaunchpadMiniButton::NoteButton(0x04) => "a5",
            LaunchpadMiniButton::NoteButton(0x05) => "a6",
            LaunchpadMiniButton::NoteButton(0x06) => "a7",
            LaunchpadMiniButton::NoteButton(0x07) => "a8",
            LaunchpadMiniButton::NoteButton(0x08) => "oA",
            LaunchpadMiniButton::NoteButton(0x10) => "b1",
            LaunchpadMiniButton::NoteButton(0x11) => "b2",
            LaunchpadMiniButton::NoteButton(0x12) => "b3",
            LaunchpadMiniButton::NoteButton(0x13) => "b4",
            LaunchpadMiniButton::NoteButton(0x14) => "b5",
            LaunchpadMiniButton::NoteButton(0x15) => "b6",
            LaunchpadMiniButton::NoteButton(0x16) => "b7",
            LaunchpadMiniButton::NoteButton(0x17) => "b8",
            LaunchpadMiniButton::NoteButton(0x18) => "oB",
            LaunchpadMiniButton::NoteButton(0x20) => "c1",
            LaunchpadMiniButton::NoteButton(0x21) => "c2",
            LaunchpadMiniButton::NoteButton(0x22) => "c3",
            LaunchpadMiniButton::NoteButton(0x23) => "c4",
            LaunchpadMiniButton::NoteButton(0x24) => "c5",
            LaunchpadMiniButton::NoteButton(0x25) => "c6",
            LaunchpadMiniButton::NoteButton(0x26) => "c7",
            LaunchpadMiniButton::NoteButton(0x27) => "c8",
            LaunchpadMiniButton::NoteButton(0x28) => "oC",
            LaunchpadMiniButton::NoteButton(0x30) => "d1",
            LaunchpadMiniButton::NoteButton(0x31) => "d2",
            LaunchpadMiniButton::NoteButton(0x32) => "d3",
            LaunchpadMiniButton::NoteButton(0x33) => "d4",
            LaunchpadMiniButton::NoteButton(0x34) => "d5",
            LaunchpadMiniButton::NoteButton(0x35) => "d6",
            LaunchpadMiniButton::NoteButton(0x36) => "d7",
            LaunchpadMiniButton::NoteButton(0x37) => "oD",
            LaunchpadMiniButton::NoteButton(0x38) => "d8",
            LaunchpadMiniButton::NoteButton(0x40) => "e1",
            LaunchpadMiniButton::NoteButton(0x41) => "e2",
            LaunchpadMiniButton::NoteButton(0x42) => "e3",
            LaunchpadMiniButton::NoteButton(0x43) => "e4",
            LaunchpadMiniButton::NoteButton(0x44) => "e5",
            LaunchpadMiniButton::NoteButton(0x45) => "e6",
            LaunchpadMiniButton::NoteButton(0x46) => "e7",
            LaunchpadMiniButton::NoteButton(0x47) => "e8",
            LaunchpadMiniButton::NoteButton(0x48) => "oE",
            LaunchpadMiniButton::NoteButton(0x50) => "f1",
            LaunchpadMiniButton::NoteButton(0x51) => "f2",
            LaunchpadMiniButton::NoteButton(0x52) => "f3",
            LaunchpadMiniButton::NoteButton(0x53) => "f4",
            LaunchpadMiniButton::NoteButton(0x54) => "f5",
            LaunchpadMiniButton::NoteButton(0x55) => "f6",
            LaunchpadMiniButton::NoteButton(0x56) => "f7",
            LaunchpadMiniButton::NoteButton(0x57) => "f8",
            LaunchpadMiniButton::NoteButton(0x58) => "oF",
            LaunchpadMiniButton::NoteButton(0x60) => "g1",
            LaunchpadMiniButton::NoteButton(0x61) => "g2",
            LaunchpadMiniButton::NoteButton(0x62) => "g3",
            LaunchpadMiniButton::NoteButton(0x63) => "g4",
            LaunchpadMiniButton::NoteButton(0x64) => "g5",
            LaunchpadMiniButton::NoteButton(0x65) => "g6",
            LaunchpadMiniButton::NoteButton(0x66) => "g7",
            LaunchpadMiniButton::NoteButton(0x67) => "g8",
            LaunchpadMiniButton::NoteButton(0x68) => "oG",
            LaunchpadMiniButton::NoteButton(0x70) => "h1",
            LaunchpadMiniButton::NoteButton(0x71) => "h2",
            LaunchpadMiniButton::NoteButton(0x72) => "h3",
            LaunchpadMiniButton::NoteButton(0x73) => "h4",
            LaunchpadMiniButton::NoteButton(0x74) => "h5",
            LaunchpadMiniButton::NoteButton(0x75) => "h6",
            LaunchpadMiniButton::NoteButton(0x76) => "h7",
            LaunchpadMiniButton::NoteButton(0x77) => "h8",
            LaunchpadMiniButton::NoteButton(0x78) => "oH",
            LaunchpadMiniButton::NoteButton(code) => panic!("Unexpected note button code: {}", code),
            LaunchpadMiniButton::ControlButton(code) => panic!("Unexpected control button code: {}", code),
        }
    }

    fn from_str(name: &str) -> Result<Self, Error> {
        match name {
            "o1" => Ok(LaunchpadMiniButton::ControlButton(104)),
            "o2" => Ok(LaunchpadMiniButton::ControlButton(105)),
            "o3" => Ok(LaunchpadMiniButton::ControlButton(106)),
            "o4" => Ok(LaunchpadMiniButton::ControlButton(107)),
            "o5" => Ok(LaunchpadMiniButton::ControlButton(108)),
            "o6" => Ok(LaunchpadMiniButton::ControlButton(109)),
            "o7" => Ok(LaunchpadMiniButton::ControlButton(110)),
            "o8" => Ok(LaunchpadMiniButton::ControlButton(111)),
            "a1" => Ok(LaunchpadMiniButton::NoteButton(0x00)),
            "a2" => Ok(LaunchpadMiniButton::NoteButton(0x01)),
            "a3" => Ok(LaunchpadMiniButton::NoteButton(0x02)),
            "a4" => Ok(LaunchpadMiniButton::NoteButton(0x03)),
            "a5" => Ok(LaunchpadMiniButton::NoteButton(0x04)),
            "a6" => Ok(LaunchpadMiniButton::NoteButton(0x05)),
            "a7" => Ok(LaunchpadMiniButton::NoteButton(0x06)),
            "a8" => Ok(LaunchpadMiniButton::NoteButton(0x07)),
            "oA" => Ok(LaunchpadMiniButton::NoteButton(0x08)),
            "b1" => Ok(LaunchpadMiniButton::NoteButton(0x10)),
            "b2" => Ok(LaunchpadMiniButton::NoteButton(0x11)),
            "b3" => Ok(LaunchpadMiniButton::NoteButton(0x12)),
            "b4" => Ok(LaunchpadMiniButton::NoteButton(0x13)),
            "b5" => Ok(LaunchpadMiniButton::NoteButton(0x14)),
            "b6" => Ok(LaunchpadMiniButton::NoteButton(0x15)),
            "b7" => Ok(LaunchpadMiniButton::NoteButton(0x16)),
            "b8" => Ok(LaunchpadMiniButton::NoteButton(0x17)),
            "oB" => Ok(LaunchpadMiniButton::NoteButton(0x18)),
            "c1" => Ok(LaunchpadMiniButton::NoteButton(0x20)),
            "c2" => Ok(LaunchpadMiniButton::NoteButton(0x21)),
            "c3" => Ok(LaunchpadMiniButton::NoteButton(0x22)),
            "c4" => Ok(LaunchpadMiniButton::NoteButton(0x23)),
            "c5" => Ok(LaunchpadMiniButton::NoteButton(0x24)),
            "c6" => Ok(LaunchpadMiniButton::NoteButton(0x25)),
            "c7" => Ok(LaunchpadMiniButton::NoteButton(0x26)),
            "c8" => Ok(LaunchpadMiniButton::NoteButton(0x27)),
            "oC" => Ok(LaunchpadMiniButton::NoteButton(0x28)),
            "d1" => Ok(LaunchpadMiniButton::NoteButton(0x30)),
            "d2" => Ok(LaunchpadMiniButton::NoteButton(0x31)),
            "d3" => Ok(LaunchpadMiniButton::NoteButton(0x32)),
            "d4" => Ok(LaunchpadMiniButton::NoteButton(0x33)),
            "d5" => Ok(LaunchpadMiniButton::NoteButton(0x34)),
            "d6" => Ok(LaunchpadMiniButton::NoteButton(0x35)),
            "d7" => Ok(LaunchpadMiniButton::NoteButton(0x36)),
            "oD" => Ok(LaunchpadMiniButton::NoteButton(0x37)),
            "d8" => Ok(LaunchpadMiniButton::NoteButton(0x38)),
            "e1" => Ok(LaunchpadMiniButton::NoteButton(0x40)),
            "e2" => Ok(LaunchpadMiniButton::NoteButton(0x41)),
            "e3" => Ok(LaunchpadMiniButton::NoteButton(0x42)),
            "e4" => Ok(LaunchpadMiniButton::NoteButton(0x43)),
            "e5" => Ok(LaunchpadMiniButton::NoteButton(0x44)),
            "e6" => Ok(LaunchpadMiniButton::NoteButton(0x45)),
            "e7" => Ok(LaunchpadMiniButton::NoteButton(0x46)),
            "e8" => Ok(LaunchpadMiniButton::NoteButton(0x47)),
            "oE" => Ok(LaunchpadMiniButton::NoteButton(0x48)),
            "f1" => Ok(LaunchpadMiniButton::NoteButton(0x50)),
            "f2" => Ok(LaunchpadMiniButton::NoteButton(0x51)),
            "f3" => Ok(LaunchpadMiniButton::NoteButton(0x52)),
            "f4" => Ok(LaunchpadMiniButton::NoteButton(0x53)),
            "f5" => Ok(LaunchpadMiniButton::NoteButton(0x54)),
            "f6" => Ok(LaunchpadMiniButton::NoteButton(0x55)),
            "f7" => Ok(LaunchpadMiniButton::NoteButton(0x56)),
            "f8" => Ok(LaunchpadMiniButton::NoteButton(0x57)),
            "oF" => Ok(LaunchpadMiniButton::NoteButton(0x58)),
            "g1" => Ok(LaunchpadMiniButton::NoteButton(0x60)),
            "g2" => Ok(LaunchpadMiniButton::NoteButton(0x61)),
            "g3" => Ok(LaunchpadMiniButton::NoteButton(0x62)),
            "g4" => Ok(LaunchpadMiniButton::NoteButton(0x63)),
            "g5" => Ok(LaunchpadMiniButton::NoteButton(0x64)),
            "g6" => Ok(LaunchpadMiniButton::NoteButton(0x65)),
            "g7" => Ok(LaunchpadMiniButton::NoteButton(0x66)),
            "g8" => Ok(LaunchpadMiniButton::NoteButton(0x67)),
            "oG" => Ok(LaunchpadMiniButton::NoteButton(0x68)),
            "h1" => Ok(LaunchpadMiniButton::NoteButton(0x70)),
            "h2" => Ok(LaunchpadMiniButton::NoteButton(0x71)),
            "h3" => Ok(LaunchpadMiniButton::NoteButton(0x72)),
            "h4" => Ok(LaunchpadMiniButton::NoteButton(0x73)),
            "h5" => Ok(LaunchpadMiniButton::NoteButton(0x74)),
            "h6" => Ok(LaunchpadMiniButton::NoteButton(0x75)),
            "h7" => Ok(LaunchpadMiniButton::NoteButton(0x76)),
            "h8" => Ok(LaunchpadMiniButton::NoteButton(0x77)),
            "oH" => Ok(LaunchpadMiniButton::NoteButton(0x78)),
            name => Err(Error::UnknownButtonError(String::from(name))),
        }
    }

    /*
    fn to_id(&self) -> usize {
        let id = match self {
            LaunchpadMiniButton::NoteButton(code) => { (code / 16) * 9 + (code % 16) }
            LaunchpadMiniButton::ControlButton(code) => { 70 + (code % 104) }
        };
        assert!(id < 80);
        return id as usize;
    }

    fn from_id(id: usize) -> Result<Self, Error> {
        match id {
            0..=70 => Ok(LaunchpadMiniButton::NoteButton(((id / 9) * 16 + (id % 9)) as u8)),
            71..=79 => Ok(LaunchpadMiniButton::ControlButton((104 + (id % 9)) as u8)),
            _ => Err(Error::IllegalButtonIdError(id))
        }
    }
    */

    const TOTAL: usize = 80;
}

impl LaunchpadMiniButton {
    fn from(msg: &MidiMessage) -> Self {
        match msg.status {
            0xb0 => LaunchpadMiniButton::ControlButton(msg.data1),
            0x90 => LaunchpadMiniButton::NoteButton(msg.data1),
            _ => panic!("Received unexpted midi status!"),
        }
    }
}


pub struct LaunchpadMini<'a> {
    pub rx: InputPort<'a>,
    pub tx: OutputPort<'a>,
}

const MIDI_BUFFER_SIZE: usize = 1024;

impl LaunchpadMini<'_> {
    pub fn new(midi_context: &PortMidi) -> LaunchpadMini {
        let (rx_id, tx_id) = Self::find_midi_device(midi_context);

        let rx_device = midi_context.device(rx_id).expect("Loading RX device failed.");
        let tx_device = midi_context.device(tx_id).expect("Loading TX device failed.");

        let rx = midi_context.input_port(rx_device.clone(), MIDI_BUFFER_SIZE).expect("Could not connect to input port.");
        println!("Connected to rx device with id {}: {}", rx_device.id(), rx_device.name());

        let tx = midi_context.output_port(tx_device.clone(), MIDI_BUFFER_SIZE).expect("Could not connect to output port.");
        println!("Connected to tx device with id {}: {}", tx_device.id(), tx_device.name());

        let mut launchpad = LaunchpadMini {
            rx,
            tx,
        };

        launchpad.set_all(rg_color_code(0, 3));
        // Flush buffer
        launchpad.poll();

        sleep(Duration::from_millis(500));
        launchpad.reset_all();
        return launchpad;
    }

    fn find_midi_device(midi_context: &PortMidi) -> (i32, i32) {
        let devices = midi_context.devices().expect("Failed to load midi devices");

        for x in &devices {
            println!("Available device: {}, input: {}, output: {}", x.name(), x.is_input(), x.is_output());
        }

        let (rx_id, tx_id) = midi::find_device_with_name(&devices, "Launchpad Mini");

        let rx_id = rx_id.expect("Could not find midi device: RX!");
        let tx_id = tx_id.expect("Could not find midi device: TX!");

        return (rx_id, tx_id);
    }

    fn for_all<F>(&mut self, f: F) -> Result<(), Error> where F: Fn(LaunchpadMiniButton) -> MidiMessage {
        for row_id in 0..=7 {
            let row_code = (row_id as u8) << 4;
            for column in 0..=8 {
                let button = row_code + (column as u8);
                let msg = f(LaunchpadMiniButton::NoteButton(button));
                self.tx.write_message(msg)?
            }
        }

        for button in 104..=111 {
            let msg = f(LaunchpadMiniButton::ControlButton(button));
            self.tx.write_message(msg);
        }
        Ok(())
    }
}

impl Launchpad for LaunchpadMini<'_> {
    type But = LaunchpadMiniButton;
    type Col = RGColor;

    fn set_led(&mut self, button: &LaunchpadMiniButton, color: RGColor) -> Result<(), Error> {
        self.tx.write_message(Self::led_on_message(button, color))?;
        Ok(())
    }

    fn reset_led(&mut self, button: &LaunchpadMiniButton) -> Result<(), Error> {
        self.tx.write_message(Self::led_reset_message(button))?;
        Ok(())
    }

    fn set_all(&mut self, color: RGColor) -> Result<(), Error> {
        self.for_all(|button| Self::led_on_message(&button, color))
    }

    fn reset_all(&mut self) -> Result<(), Error> {
        self.tx.write_message(Self::led_reset_all_message())?;
        Ok(())
    }

    fn poll(&mut self) -> Result<Vec<(LaunchpadMiniButton, ButtonEvent)>, Error> {
        let events = self.rx.read_n(MIDI_BUFFER_SIZE)?
            .unwrap_or_else(Vec::new)
            .iter()
            .map(Self::event_from_message)
            .collect();
        Ok(events)
    }
}

impl LaunchpadMini<'_> {
    #[inline]
    fn led_on_message(button: &LaunchpadMiniButton, color: RGColor) -> MidiMessage {
        match button {
            LaunchpadMiniButton::ControlButton(code) => MidiMessage {
                status: 0xb0,
                data1: *code,
                data2: color,
                data3: 0,
            },
            LaunchpadMiniButton::NoteButton(code) => MidiMessage {
                status: 0x90,
                data1: *code,
                data2: color,
                data3: 0,
            }
        }
    }

    #[inline]
    fn led_reset_message(button: &LaunchpadMiniButton) -> MidiMessage {
        match button {
            LaunchpadMiniButton::ControlButton(code) => MidiMessage {
                status: 0xb0,
                data1: *code,
                data2: 0,
                data3: 0,
            },
            LaunchpadMiniButton::NoteButton(code) => MidiMessage {
                status: 0x80,
                data1: *code,
                data2: 0,
                data3: 0,
            }
        }
    }

    #[inline]
    fn led_reset_all_message() -> MidiMessage {
        MidiMessage {
            status: 0xb0,
            data1: 0,
            data2: 0,
            data3: 0,
        }
    }

    fn event_from_message(msg: &MidiEvent) -> (LaunchpadMiniButton, ButtonEvent) {
        let msg = msg.message;
        let button = LaunchpadMiniButton::from(&msg);
        let event = match msg.data2 {
            127 => ButtonEvent::Pressed,
            0 => ButtonEvent::Released,
            _ => panic!("Received unexpected value: {}", msg),
        };
        return (button, event);
    }
}
