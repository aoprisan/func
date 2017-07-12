pub trait Monoid<T> {
    fn zero() -> T;
    fn append(t1: T, t2: T) -> T;
}

impl Monoid<String> for String {
    fn zero() -> String { "".to_string() }
    fn append(t1: String, t2: String) -> String { format!("{}{}", t1, t2) }
}

impl Monoid<u64> for u64 {
    fn zero() -> u64 { 0 }
    fn append(t1: u64, t2: u64) -> u64 { t1 + t2 }
}