pub enum Shape {
    Circle { radius: f32 },
    Square { side: f32 },
    Rectangle { width: f32, height: f32 },
    Composite { items: Vec<Shape> },
}

impl Shape {
    pub fn area(&self) -> f32 {
        match self {
            Shape::Circle { radius } => radius * radius * core::f32::consts::PI,
            Shape::Square { side } => side * side,
            Shape::Rectangle { width, height } => width * height,
            Shape::Composite { items } => items.iter().map(|i| i.area()).sum(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use super::*;

    #[test]
    fn contrived_example() {
        let c: Shape = Shape::Composite {
            items: vec![
                Shape::Circle {
                    radius: FRAC_1_SQRT_PI,
                },
                Shape::Square { side: 1f32 },
                Shape::Rectangle {
                    width: 1f32,
                    height: 2f32,
                },
            ],
        };
        assert_eq!(c.area(), 4f32);
    }
}
