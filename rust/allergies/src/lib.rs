use enum_iterator::{all, Sequence};

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = 1_u32 << (*allergen as u32);
        self.score & allergen_score == allergen_score
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        all()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
