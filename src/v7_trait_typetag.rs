use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "type")]
pub trait Shape {
    fn area(&self) -> f32;
}

#[derive(Serialize, Deserialize)]
pub struct Circle {
    radius: f32,
}

#[typetag::serde]
impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * core::f32::consts::PI
    }
}

#[derive(Serialize, Deserialize)]
pub struct Rectangle {
    width: f32,
    height: f32,
}

#[typetag::serde]
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

#[typetag::serde(name = "Composite")]
impl Shape for Vec<Box<dyn Shape>> {
    fn area(&self) -> f32 {
        self.iter().map(|i| i.area()).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contrived_example() {
        let json = include_str!("./v7_trait_typetag.json");
        let c: Box<dyn Shape> = serde_json::from_str(json).expect("Known to parse correctly");
        assert_eq!(c.area(), 3f32);
    }
}
