use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    (num > 0).then(|| {
        let aliquot_sum = (1..=num / 2).filter(|&factor| num % factor == 0).sum();
        match num.cmp(&aliquot_sum) {
            Ordering::Less => Classification::Abundant,
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Deficient,
        }
    })
}
