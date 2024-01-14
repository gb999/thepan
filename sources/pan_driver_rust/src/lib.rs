use midi_msg::*;
use std::{
    cmp::{max, min},
    time::{Duration, SystemTime},
};

#[derive(PartialEq, Eq)]
pub enum Command {
    Potentiometer { id: usize, value: u16 },
    RotaryEncoder { id: usize, direction: i8 },
    Button { id: usize },
}

/// Parses a valid command
/// # Panics!
/// If command is not valid
pub fn parse_line(line: &str) -> Command {
    let line: Vec<&str> = line.trim().split(" ").collect();
    match line[0] {
        "BTN" => Command::Button {
            id: line[1].parse().expect("Invalid argument."),
        },
        "POT" => Command::Potentiometer {
            id: line[1].parse().expect("Invalid argument."),
            value: line[2].parse().expect("Invalid argument."),
        },
        "ROT" => Command::RotaryEncoder {
            id: line[1].parse().expect("Invalid argument."),
            direction: line[2].parse().expect("Invalid argument."),
        },
        invalid => panic!("Invalid command: {}", invalid),
    }
}

#[derive(Clone, Copy)]
enum ButtonState {
    SinglePressed,
    SingleReleased,
    DoublePressed,
    DoubleReleased,
}

#[derive(Clone, Copy)]
struct Button {
    state: ButtonState,
    last_time_pressed: SystemTime,
    last_time_released: SystemTime,
}

impl Button {
    fn set_state(&mut self, timestamp: SystemTime) {
        self.state = match self.state {
            ButtonState::SinglePressed => {
                self.last_time_released = timestamp;
                ButtonState::SingleReleased
            }
            ButtonState::DoublePressed => {
                self.last_time_released = timestamp;
                ButtonState::DoubleReleased
            }

            ButtonState::SingleReleased => {
                self.last_time_pressed = timestamp;
                if timestamp
                    .duration_since(self.last_time_pressed)
                    .unwrap_or_default()
                    < Duration::from_millis(200)
                    && timestamp
                        .duration_since(self.last_time_released)
                        .unwrap_or_default()
                        < Duration::from_millis(300)
                {
                    ButtonState::DoublePressed
                } else {
                    ButtonState::SinglePressed
                }
            }
            ButtonState::DoubleReleased => {
                self.last_time_pressed = timestamp;
                ButtonState::SinglePressed
            }
        };
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            state: ButtonState::SingleReleased,
            last_time_pressed: SystemTime::now(),
            last_time_released: SystemTime::now(),
        }
    }
}

#[derive(Clone, Copy)]
struct Potentiometer {
    val: u16,
}

impl Potentiometer {
    fn set_state(&mut self, val: u16, _timestamp: SystemTime) {
        self.val = 1023 - val;
    }
}

#[derive(Clone, Copy)]
struct RotaryEncoder {
    last_time_rotated: SystemTime,
    velocity: i8, // Number between -64, +64
}
impl RotaryEncoder {
    fn set_state(&mut self, val: i8, timestamp: SystemTime) {
        let time_since_last_rotation = timestamp
            .duration_since(self.last_time_rotated)
            .unwrap_or_default();
        self.last_time_rotated = timestamp;
        if time_since_last_rotation > Duration::from_millis(200) {
            self.velocity = 0;
        }

        if val.signum() == self.velocity.signum() {
            self.velocity += f32::sqrt(val.abs() as f32) as i8 * val.signum();
        } else {
            self.velocity = val;
        }
        self.velocity = min(max(self.velocity, -64), 63);
    }
}
impl Default for RotaryEncoder {
    fn default() -> Self {
        Self {
            last_time_rotated: SystemTime::now(),
            velocity: Default::default(),
        }
    }
}
pub struct Pan {
    potentiometers: [Potentiometer; 5],
    buttons: [Button; 6],
    rotary_encoders: [RotaryEncoder; 5],
}

impl Pan {
    pub fn new() -> Self {
        Pan {
            potentiometers: [Potentiometer { val: 0 }; 5],
            buttons: [Button::default(); 6], // Buttons[5] is alt button
            rotary_encoders: [RotaryEncoder::default(); 5],
        }
    }

    pub fn handle_command(&mut self, command: Command) -> MidiMsg {
        let current_time = SystemTime::now();
        match command {
            Command::Potentiometer { id, value } => {
                self.potentiometers[id].set_state(value, current_time);
                let msg = ControlChange::Undefined {
                    control: (0x66 + id) as u8,
                    value: (self.potentiometers[id].val / 8) as u8,
                };
                MidiMsg::ChannelVoice {
                    channel: Channel::Ch1,
                    msg: midi_msg::ChannelVoiceMsg::ControlChange { control: msg },
                }
            }
            Command::RotaryEncoder { id, direction } => {
                self.rotary_encoders[id].set_state(direction, current_time);
                let msg = ControlChange::Undefined {
                    control: (0x0E
                        + id
                        + match self.buttons[id].state {
                            ButtonState::SingleReleased => 0,
                            ButtonState::DoubleReleased => 5,
                            ButtonState::SinglePressed => 10,
                            ButtonState::DoublePressed => 15,
                        }) as u8,
                    value: (64 + self.rotary_encoders[id].velocity) as u8,
                };
                MidiMsg::ChannelVoice {
                    channel: Channel::Ch1,
                    msg: midi_msg::ChannelVoiceMsg::ControlChange { control: msg },
                }
            }
            Command::Button { id } => {
                self.buttons[id].set_state(current_time);
                let msg = match self.buttons[id].state {
                    ButtonState::SinglePressed | ButtonState::DoublePressed => {
                        ChannelVoiceMsg::NoteOn {
                            note: id as u8,
                            velocity: 127,
                        }
                    }
                    ButtonState::SingleReleased | ButtonState::DoubleReleased => {
                        ChannelVoiceMsg::NoteOff {
                            note: id as u8,
                            velocity: 127,
                        }
                    }
                };
                MidiMsg::ChannelVoice {
                    channel: Channel::Ch1,
                    msg: msg,
                }
            }
        }
    }
}
