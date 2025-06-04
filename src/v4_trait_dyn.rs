pub trait Shape {
    fn area(&self) -> f32;
}

pub struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * core::f32::consts::PI
    }
}

pub struct Square {
    side: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Composite {
    items: Vec<Box<dyn Shape>>,
}

impl Shape for Composite {
    fn area(&self) -> f32 {
        self.items.iter().map(|i| i.area()).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn contrived_example() {
        let c: &dyn Shape = &Composite {
            items: vec![
                Box::new(Circle {
                    radius: FRAC_1_SQRT_PI,
                }),
                Box::new(Square { side: 1f32 }),
                Box::new(Rectangle {
                    width: 1f32,
                    height: 2f32,
                }),
            ],
        };
        assert_eq!(c.area(), 4f32);
    }
}
