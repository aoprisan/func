pub trait Semigroup {
    type T;
    fn mappend(t1: Self::T, t2: Self::T) -> Self::T;
    fn add_and_own(self, t2: Self::T) -> Self::T;
}

impl Semigroup for String {
    type T = String;
    fn mappend(t1: String, t2: String) -> String { format!("{}{}", t1, t2) }
    fn add_and_own(self, t2: Self::T) -> Self::T { format!("{}{}", self, t2) }
}

impl Semigroup for u64 {
    type T = u64;
    fn mappend(t1: u64, t2: u64) -> u64 { t1 + t2 }
    fn add_and_own(self, t2: Self::T) -> Self::T { self + t2 }
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