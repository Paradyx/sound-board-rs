# Sound Board RS
A little tool written in Rust for playing sounds on a Launchpad. Thanks to the audio library [rodio] it can run 
many tracks concurrently, which is a rare feature on other Launchpad tools. 

It was written with Dungeonmastering in mind. Loading many long audio files at the same time, does should not 
cause performance issues. 

# Requirements
For the Midi connection it uses [Portmidi](http://portmedia.sourceforge.net/portmidi/). The library must be installed. 

# Features
Supported Devices:
  - [x] Launchpad Mini
  - [ ] Launchpad Mk2

Usability:
- [x] Configure all tracks in a config file.
- [x] Reference buttons by a human readable name instead of a midi code.
- [x] Nice color feedback.

Controll options:
- [x] ToggledTrack 
  - Play: short press
  - Pause: short press
  - Restart: long press (not working at the moment)
- [ ] Fire & Forget (can play the same sound multiple times at the same time)
  - Play: press the button
- [ ] Fire & Restart (does not allow concurrent play of the same sound)
  - Play: press the button
- [ ] PlayWhileHold
  - Play: press and hold the button
  - Stop: relese the button

Performance: 
- [ ] Buffered Tracks

# Build
```
cargo build
```

# Execution 
```
cargo run .config/example.toml
```

