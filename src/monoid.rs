pub trait Semigroup {
    type T;
    fn append(t1: Self::T, t2: Self::T) -> Self::T;
}

pub trait Monoid : Semigroup {
    fn zero() -> Self::T;
}

impl Semigroup for String {
    type T = String;
    fn append(t1: String, t2: String) -> String { format!("{}{}", t1, t2) }
}

impl Monoid for String {
    fn zero() -> String { "".to_string() }
}

impl Semigroup for u64 {
    type T = u64;
    fn append(t1: u64, t2: u64) -> u64 { t1 + t2 }
}

impl Monoid for u64 {
    fn zero() -> u64 { 0 }
}