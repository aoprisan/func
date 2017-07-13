use semigroup::*;

pub trait Monoid : Semigroup {
    fn zero() -> Self::T;
}

impl Monoid for String {
    fn zero() -> String { "".to_string() }
}

#[test]
fn test_string_monoid() {

    assert_eq!(String::mappend("a".to_string(), "b".to_string()), "ab".to_string());
    assert_eq!("a".to_string().add_and_own("b".to_string()), "ab".to_string());
    assert_eq!(String::zero(), "".to_string());
}


impl Monoid for u64 {
    fn zero() -> u64 { 0 }
}


#[test]
fn test_u64_monoid() {

    assert_eq!(u64::mappend(1,2), 3);
    assert_eq!(1.add_and_own(2), 3);
    assert_eq!(u64::zero(), 0);
}

impl<A> Monoid for Vec<A> {
    fn zero() -> Self::T {
       vec![]
    }
}

#[test]
fn test_vec_monoid() {
    let v1 = vec![1,2];
    let v2 = vec![3,4];
    let v3 =vec![1,2,3,4];
    assert_eq!(Vec::mappend(v1.clone(), v2.clone()), v3);
    assert_eq!(vec![1,2].add_and_own(vec![3,4]), vec![1,2,3,4]);
    assert!(Vec::<u64>::zero().is_empty());
}