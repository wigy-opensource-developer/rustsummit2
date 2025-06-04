pub trait Shape {
    type Number;
    fn area(&self) -> Self::Number;
}

pub struct Circle {
    radius: f32,
}

impl Shape for Circle {
    type Number = f32;

    fn area(&self) -> f32 {
        self.radius * self.radius * core::f32::consts::PI
    }
}

pub struct Square {
    side: f64,
}

impl Shape for Square {
    type Number = f64;

    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    type Number = f32;
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Composite<T> {
    items: Vec<Box<dyn Shape<Number = T>>>,
}

impl<T> Shape for Composite<T>
where
    T: std::iter::Sum,
{
    type Number = T;
    fn area(&self) -> T {
        self.items.iter().map(|i| i.area()).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn contrived_example() {
        let c: &dyn Shape<Number = _> = &Composite {
            items: vec![
                Box::new(Circle {
                    radius: FRAC_1_SQRT_PI,
                }),
                //Box::new(Square { side: 1f64 }),
                Box::new(Rectangle {
                    width: 1f32,
                    height: 2f32,
                }),
            ],
        };
        assert_eq!(c.area(), 3f32);
    }
}
