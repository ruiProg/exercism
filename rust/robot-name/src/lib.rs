use std::{cell::RefCell, collections::HashSet};

use rand::Rng;

#[derive(Default)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let name = loop {
            let name = generate_name();
            if add_robot(&name) {
                break name;
            }
        };
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        *self = Self::new()
    }
}

thread_local!(static ROBOT_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

fn add_robot(name: &String) -> bool {
    ROBOT_NAMES.with(|robot_names| {
        let mut robot_names = robot_names.borrow_mut();
        let unique_name = !robot_names.contains(name);
        if unique_name {
            robot_names.insert(name.clone());
        }
        unique_name
    })
}

fn generate_name() -> String {
    let mut rng = rand::thread_rng();
    let first_letter = rng.gen_range('A'..='Z');
    let second_letter = rng.gen_range('A'..='Z');
    let number = rng.gen_range(0..1_000);

    format!("{first_letter}{second_letter}{number:<03}")
}
