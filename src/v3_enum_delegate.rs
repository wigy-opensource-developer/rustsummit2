pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        self.radius * self.radius * core::f32::consts::PI
    }
}

pub struct Square {
    side: f32,
}

impl Square {
    pub fn area(&self) -> f32 {
        self.side * self.side
    }
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Composite {
    items: Vec<Shape>,
}

impl Composite {
    pub fn area(&self) -> f32 {
        self.items.iter().map(|i| i.area()).sum()
    }
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
            Shape::Circle(inner) => inner.area(),
            Shape::Square(inner) => inner.area(),
            Shape::Rectangle(inner) => inner.area(),
            Shape::Composite(inner) => inner.area(),
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
