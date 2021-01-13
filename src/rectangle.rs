#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    #[allow(dead_code)]
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    #[allow(dead_code)]
    pub fn new(i:u32, j:u32) -> Rectangle {
        return Rectangle {
            width:i, height:j
        };
    }
}

