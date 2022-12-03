use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Default + Copy + PartialOrd + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let slide = sides.repeat(2);
        slide
            .windows(3)
            .take(3)
            .all(|sides_slide| {
                sides_slide[0] != T::default() && sides_slide[0] <= sides_slide[1] + sides_slide[2]
            })
            .then_some(Self { sides })
    }
    pub fn is_equilateral(&self) -> bool {
        (self.sides[0] == self.sides[1]) && (self.sides[1] == self.sides[2])
    }

    pub fn is_scalene(&self) -> bool {
        let slide = self.sides.repeat(2);
        slide
            .windows(2)
            .take(3)
            .all(|sides_slide| sides_slide[0] != sides_slide[1])
    }

    pub fn is_isosceles(&self) -> bool {
        let slide = self.sides.repeat(2);
        slide
            .windows(2)
            .take(3)
            .any(|sides_slide| sides_slide[0] == sides_slide[1])
    }
}
