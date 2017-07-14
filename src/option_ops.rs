#![allow(dead_code, unused)]

use std::marker::Sized;

pub trait OptionExtOps {
    fn some(self) -> Option<Self> where Self: Sized {
        Some(self)
    }
}

impl<T> OptionExtOps for T {}

fn none<T>() -> Option<T> {
    None
}

#[test]
pub fn test_ext_option_ops() {
    assert_eq!(Some(10), 10.some());
    assert_eq!(None, none::<i32>());
}

pub trait OptionOps<T,V> {
    fn fold<F>(&self, z: V, nz: F) -> V where F: FnOnce(&T)-> V;
}

impl<T,V> OptionOps<T,V> for Option<T> {
    fn fold<F>(&self, z: V, nz: F) -> V where F: FnOnce(&T)-> V {
        match self.as_ref() {
            Some(ref x) => nz(x),
            None => z
        }
    }
}

#[test]
pub fn test_ops() {
    assert_eq!(10.some().fold(0, |_| 666), 666);
    assert_eq!(none::<i32>().fold(0,|_| 666), 0);

}