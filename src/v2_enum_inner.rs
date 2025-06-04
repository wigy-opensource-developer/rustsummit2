pub struct Circle {
    radius: f32,
}

pub struct Square {
    side: f32,
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

pub struct Composite {
    items: Vec<Shape>,
}

pub enum Shape {
    Circle(Circle),
    Square(Square),
    Rectangle(Rectangle),
    Composite(Composite),
}

impl Shape {
    pub fn area(&self) -> f32 {
        match self {
            Shape::Circle(Circle { radius }) => radius * radius * core::f32::consts::PI,
            Shape::Square(Square { side }) => side * side,
            Shape::Rectangle(Rectangle { width, height }) => width * height,
            Shape::Composite(Composite { items }) => items.iter().map(|i| i.area()).sum(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn contrived_example() {
        let c: Shape = Shape::Composite(Composite {
            items: vec![
                Shape::Circle(Circle {
                    radius: FRAC_1_SQRT_PI,
                }),
                Shape::Square(Square { side: 1f32 }),
                Shape::Rectangle(Rectangle {
                    width: 1f32,
                    height: 2f32,
                }),
            ],
        });
        assert_eq!(c.area(), 4f32);
    }
}
