pub trait Semigroup {
    type T;
    fn mappend(t1: Self::T, t2: Self::T) -> Self::T;
    fn add_and_own(self, t2: Self::T) -> Self::T;
}

pub trait Monoid : Semigroup {
    fn zero() -> Self::T;
}

impl Semigroup for String {
    type T = String;
    fn mappend(t1: String, t2: String) -> String { format!("{}{}", t1, t2) }
    fn add_and_own(self, t2: Self::T) -> Self::T { format!("{}{}", self, t2) }
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

impl Semigroup for u64 {
    type T = u64;
    fn mappend(t1: u64, t2: u64) -> u64 { t1 + t2 }
    fn add_and_own(self, t2: Self::T) -> Self::T { self + t2 }
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


impl<A> Semigroup for Vec<A> {
    type T = Vec<A>;

    fn mappend(t1: Self::T, t2: Self::T) -> Self::T {
        let mut ret = Vec::new();
        ret.extend(t1);
        ret.extend(t2);
        ret
    }

    fn add_and_own(self, t2: Self::T) -> Self::T {
        let mut ret = Vec::new();
        ret.extend(self);
        ret.extend(t2);
        ret
    }
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