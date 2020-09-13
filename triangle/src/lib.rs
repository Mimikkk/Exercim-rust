pub struct Triangle {a: u64, b: u64, c: u64}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        match sides[0] + sides[1] > sides[2] {
            true => {Some(Triangle {a: sides[0], b: sides[1], c: sides[2]})},
            false => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b && self.b != self.c || self.a != self.b && self.b == self.c
    }
}
