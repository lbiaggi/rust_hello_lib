#[cfg(test)]
use crate::rectangle::Rectangle;
mod rectangle;



#[allow(unused_imports)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(8,7);
        let smaller = Rectangle::new(7,6);
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cant_holder_larger() {
        let larger = Rectangle::new(8,7);
        let smaller = Rectangle::new(7,6);
        assert_ne!(smaller.can_hold(&larger), true);
    }
}
