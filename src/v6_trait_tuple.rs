use impl_trait_for_tuples::impl_for_tuples;

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

#[impl_for_tuples(1, 4)]
impl<T> Shape for Tuple
where
    T: std::iter::Sum,
{
    for_tuples!( where #( Tuple: Shape<Number=T> )* );

    type Number = T;
    fn area(&self) -> T {
        [for_tuples! { #( Tuple.area() ),* }].into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn contrived_example() {
        let c = (
            Circle {
                radius: FRAC_1_SQRT_PI,
            },
            // Square { side: 1f64 },
            Rectangle {
                width: 1f32,
                height: 2f32,
            },
        );
        assert_eq!(c.area(), 3f32);
    }
}
