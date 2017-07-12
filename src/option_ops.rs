use std::marker::Sized;

pub trait OptionOps {
    fn some(self) -> Option<Self> where Self: Sized {
        Some(self)
    }
}

impl<T> OptionOps for T {}

fn none<T>() -> Option<T> {
    None
}

#[test]
pub fn test_ops() {
    assert_eq!(Some(10), 10.some());
    assert_eq!(None, none::<i32>());
}