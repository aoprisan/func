use semigroup::*;

pub trait Monoid : Semigroup {
    fn zero() -> Self::T;
}

///A macro to implement monoid for numeric semigroups
#[macro_export]
macro_rules! monoid_num {
    ($t:ident, $z:expr) => {
        impl Monoid for $t {
            fn zero() -> Self::T {
                $z
            }
        }
    }
}

///A macro to implement monoid for Semigroups which have a new method
#[macro_export]
macro_rules! monoid {
    ($t:ident) => {
        impl<T> Monoid for $t<T> {
            fn zero() -> Self::T {
                $t::new()
            }
        }
    }
}

//Implementation for numeric Monoids
monoid_num!(i8, 0);
monoid_num!(i16, 0);
monoid_num!(i32, 0);
monoid_num!(i64, 0);
monoid_num!(u8, 0);
monoid_num!(u16, 0);
monoid_num!(u32, 0);
monoid_num!(u64, 0);
monoid_num!(isize, 0);
monoid_num!(usize, 0);
monoid_num!(f32, 0.0);
monoid_num!(f64, 0.0);

//Implementation of Monoid for Vec<T>
monoid!(Vec);


impl Monoid for String {
    fn zero() -> String { "".to_string() }
}

impl<A: Monoid + Semigroup<T = A>> Monoid for Box<A> {

    fn zero() -> Self::T {
        Box::new(A::zero())
    }
}

#[test]
fn test_string_monoid() {

    assert_eq!("a".to_string().add_and_own("b".to_string()), "ab".to_string());
    assert_eq!(String::zero(), "".to_string());
}

#[test]
fn test_u64_monoid() {

    assert_eq!(1.add_and_own(2), 3);
    assert_eq!(u64::zero(), 0);
}


#[test]
fn test_vec_monoid() {
    assert_eq!(vec![1,2].add_and_own(vec![3,4]), vec![1,2,3,4]);
    assert!(Vec::<u64>::zero().is_empty());
}

#[test]
fn test_box_monoid(){
    assert_eq!(Box::<u64>::zero(), Box::new(0));
}