use std::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    strikes_indexes: Vec<usize>,
    spare_indexes: Vec<usize>,
    frame_index: usize,
    throw_index: usize,
    pins_table: [[u16; 3]; 10],
    pins_on_track: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            strikes_indexes: vec![],
            spare_indexes: vec![],
            frame_index: 0,
            throw_index: 0,
            pins_on_track: Self::MAX_PINS,
            pins_table: [
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
                [0, u16::MAX, u16::MAX],
            ],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.verify_roll_valid(pins)?;
        self.pins_table[self.frame_index][self.throw_index] = pins;
        self.update_score_for_previous_strikes(pins);
        self.update_score_for_previous_spares(pins);
        self.update_strike_spare_indexes(pins);
        self.update_pins_on_track();
        self.update_indexes();
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_index < 10 {
            return None;
        }
        let get_pins = |frame: &[u16], index: usize| {
            if frame[index] == u16::MAX {
                0
            } else {
                frame[index]
            }
        };
        let score = self.pins_table.iter().fold(0u16, |sum, frame| {
            sum + get_pins(frame, 0) + get_pins(frame, 1) + get_pins(frame, 2)
        });
        Some(score)
    }

    fn verify_roll_valid(&self, pins: u16) -> Result<(), Error> {
        if self.frame_index > Self::MAX_FRAMES_INDEX {
            return Err(Error::GameComplete);
        }
        if pins > self.pins_on_track {
            return Err(Error::NotEnoughPinsLeft);
        }
        Ok(())
    }

    fn update_score_for_previous_strikes(&mut self, pins: u16) {
        for strike in self.strikes_indexes.iter() {
            if self.pins_table[*strike][1] == u16::MAX {
                self.pins_table[*strike][1] = pins;
            } else if self.pins_table[*strike][2] == u16::MAX {
                self.pins_table[*strike][2] = pins;
            }
        }
    }

    fn update_score_for_previous_spares(&mut self, pins: u16) {
        for spare in self.spare_indexes.iter() {
            if self.pins_table[*spare][2] == u16::MAX {
                self.pins_table[*spare][2] = pins;
            }
        }
    }

    fn update_strike_spare_indexes(&mut self, pins: u16) {
        if self.current_frame_is_strike() {
            self.strikes_indexes.push(self.frame_index);
        } else if self.current_frame_is_spare() {
            self.spare_indexes.push(self.frame_index);
        }
    }

    fn update_pins_on_track(&mut self) {
        let pins_down = self.pins_table[self.frame_index][self.throw_index];
        if pins_down == Self::MAX_PINS {
            self.pins_on_track = Self::MAX_PINS;
        } else if self.throw_index == 1 && self.pins_table[self.frame_index][0] < Self::MAX_PINS {
            self.pins_on_track = Self::MAX_PINS
        } else {
            self.pins_on_track -= pins_down;
        }
    }

    fn update_indexes(&mut self) {
        if self.frame_index == Self::MAX_FRAMES_INDEX {
            self.last_frame_update_indexes();
        } else if self.current_frame_is_strike() || self.throw_index == 1 {
            self.frame_index += 1;
            self.throw_index = 0;
        } else {
            self.throw_index = 1;
        }
    }

    fn last_frame_update_indexes(&mut self) {
        match (
            self.throw_index,
            self.current_frame_is_strike(),
            self.current_frame_is_spare(),
        ) {
            (1, true, _) => self.throw_index += 1,
            (1, _, true) => self.throw_index += 1,
            (1, false, false) => self.frame_index += 1,
            (2, _, _) => self.frame_index += 1,
            _ => self.throw_index += 1,
        }
    }

    fn current_frame_is_strike(&self) -> bool {
        let pins = self.pins_table[self.frame_index][0];
        pins != u16::MAX && pins == Self::MAX_PINS
    }

    fn current_frame_is_spare(&self) -> bool {
        let pins_0 = self.pins_table[self.frame_index][0];
        let pins_1 = self.pins_table[self.frame_index][1];
        pins_0 != u16::MAX && pins_1 != u16::MAX && (pins_0 + pins_1 == Self::MAX_PINS)
    }
}

impl BowlingGame {
    const MAX_PINS: u16 = 10;
    const MAX_FRAMES_INDEX: usize = 9;
}
