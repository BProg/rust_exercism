pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if Triangle::is_triangle(sides) {
            Some(Self { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|&s| s == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }

    pub fn is_triangle(sides: [u64; 3]) -> bool {
        sides.iter().all(|&s| s > 0)
            && sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[2] + sides[0] >= sides[1]
    }
}
