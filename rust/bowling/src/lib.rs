#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_frame: ActiveFrame,
    frames_played: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, knocked_pins: u16) -> Result<(), Error> {
        let pins_standing = {
            let before_throw_pins = self.current_frame_pins().ok_or(Error::GameComplete)?;
            before_throw_pins
                .checked_sub(knocked_pins)
                .ok_or(Error::NotEnoughPinsLeft)?
        };

        if let Some(frame) = self.frame_result(knocked_pins, pins_standing) {
            self.frames_played.push(frame);
            self.current_frame = ActiveFrame::New;
        } else if let Some(frame) = self.updated_frame(knocked_pins, pins_standing) {
            self.current_frame = frame;
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        self.game_finished().then(|| {
            (0..Self::NUM_FRAMES)
                .map(|frame| self.score_at_frame(frame))
                .sum()
        })
    }

    fn score_at_frame(&self, mut frame_index: usize) -> u16 {
        let required_throws = self.frames_played[frame_index].num_throws_for_score();
        let mut total_score = Vec::new();

        while total_score.len() < required_throws {
            let mut throw_scores = self.frames_played[frame_index].throws();
            total_score.append(&mut throw_scores);
            frame_index += 1;
        }

        total_score[0..required_throws].iter().sum()
    }

    fn current_frame_pins(&self) -> Option<u16> {
        (!self.game_finished()).then(|| match self.current_frame {
            ActiveFrame::SingleThrow(knocked_pins) => Self::wrap_sub_knocked(knocked_pins),
            ActiveFrame::FillThrow(Self::NUM_PINS, second_throw) => {
                Self::wrap_sub_knocked(second_throw)
            },
            _ => Self::NUM_PINS,
        })
    }

    fn frame_result(&self, knocked_pins: u16, pins_standing: u16) -> Option<Frame> {
        match (pins_standing, &self.current_frame) {
            (0, &ActiveFrame::New) => (!self.last_frame()).then_some(Frame::Strike(knocked_pins)),
            (0, &ActiveFrame::SingleThrow(first_throw)) => {
                (!self.last_frame()).then_some(Frame::Spare(first_throw, knocked_pins))
            },
            (_, &ActiveFrame::SingleThrow(first_throw)) => {
                (first_throw != Self::NUM_PINS).then_some(Frame::Open(first_throw, knocked_pins))
            },
            (_, &ActiveFrame::FillThrow(first_throw, second_throw)) => {
                Some(Frame::Special(first_throw, second_throw, knocked_pins))
            },
            _ => None,
        }
    }

    fn updated_frame(&self, knocked_pins: u16, pins_standing: u16) -> Option<ActiveFrame> {
        match self.current_frame {
            ActiveFrame::New => Some(ActiveFrame::SingleThrow(knocked_pins)),
            ActiveFrame::SingleThrow(first_throw) => (pins_standing == 0
                || first_throw == Self::NUM_PINS)
                .then_some(ActiveFrame::FillThrow(first_throw, knocked_pins)),
            _ => None,
        }
    }

    fn last_frame(&self) -> bool {
        self.frames_played.len() + 1 == Self::NUM_FRAMES
    }

    fn game_finished(&self) -> bool {
        self.frames_played.len() == Self::NUM_FRAMES
    }

    fn wrap_sub_knocked(knocked_pins: u16) -> u16 {
        if knocked_pins == Self::NUM_PINS {
            Self::NUM_PINS
        } else {
            Self::NUM_PINS - knocked_pins
        }
    }

    const NUM_PINS: u16 = 10;
    const NUM_FRAMES: usize = 10;
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            current_frame: ActiveFrame::New,
            frames_played: Vec::new(),
        }
    }
}

enum ActiveFrame {
    New,
    SingleThrow(u16),
    FillThrow(u16, u16),
}

enum Frame {
    Strike(u16),
    Spare(u16, u16),
    Open(u16, u16),
    Special(u16, u16, u16),
}

impl Frame {
    fn throws(&self) -> Vec<u16> {
        match *self {
            Self::Strike(score) => vec![score],
            Self::Spare(first, second) => vec![first, second],
            Self::Open(first, second) => vec![first, second],
            Self::Special(first, second, third) => vec![first, second, third],
        }
    }

    fn num_throws_for_score(&self) -> usize {
        match self {
            Self::Open(..) => 2,
            _ => 3,
        }
    }
}
