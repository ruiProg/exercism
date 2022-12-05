pub struct RailFence(usize);

impl RailFence {
    pub fn new(rails: u32) -> Self {
        (rails > 1)
            .then_some(Self(rails as usize))
            .expect("Cannot create cypher with 0 or 1 rails")
    }

    pub fn encode(&self, text: &str) -> String {
        let chars: Vec<_> = text.chars().collect();

        (0..self.0)
            .map(|index| {
                let mut result = String::new();

                self.navigate_row(
                    index,
                    |pos| pos < chars.len(),
                    |pos| result.push(chars[pos]),
                );

                result
            })
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut result = vec![' '; cipher.len()];
        let mut cipher_iter = cipher.chars();

        (0..self.0).for_each(|index| {
            self.navigate_row(
                index,
                |pos| pos < cipher.len(),
                |pos| {
                    if let Some(ch) = cipher_iter.next() {
                        result[pos] = ch;
                    }
                },
            );
        });

        result.iter().collect()
    }

    fn navigate_row(
        &self,
        index: usize,
        continue_fn: impl Fn(usize) -> bool,
        mut action_fn: impl FnMut(usize),
    ) {
        if continue_fn(index) {
            action_fn(index);

            let mut step_iter = [Step::Down, Step::Up].iter().cycle();
            let mut pos = index;

            loop {
                let step_increment = match step_iter.next() {
                    Some(Step::Down) => 2 * (self.0 - index - 1),
                    Some(Step::Up) => 2 * index,
                    _ => unreachable!(),
                };

                if step_increment > 0 {
                    pos += step_increment;

                    if continue_fn(pos) {
                        action_fn(pos);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

enum Step {
    Down,
    Up,
}
