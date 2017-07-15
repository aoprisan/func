pub trait Semigroup {
    type T;
    fn mappend(t1: Self::T, t2: Self::T) -> Self::T;
    fn add_and_own(self, t2: Self::T) -> Self::T;
//    fn add(&self, b: &Self::T) -> Self::T;
}


///Macro to implement semigroup for numerics
#[macro_export]
macro_rules! semigroup_num {
    ($t:ident) => {
        impl Semigroup for $t {
            type T = $t;

            fn mappend(t1: Self::T, t2: Self::T) -> Self::T {
                t1 + t2
            }

            fn add_and_own(self, t2: Self::T) -> Self::T {
                self + t2
            }

//            fn add(&self, b: &Self::T) -> Self::T {
//                self + t2
//            }
        }
    }
}

///Macro for implementing SemiGroup for types that implement Extend
#[macro_export]
macro_rules! semigroup {
    ($t:ident) => {
        impl<T> Semigroup for $t<T> {
            type T = $t<T>;
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
    }
}


semigroup_num!(i8);
semigroup_num!(i16);
semigroup_num!(i32);
semigroup_num!(i64);
semigroup_num!(u8);
semigroup_num!(u16);
semigroup_num!(u32);
semigroup_num!(u64);
semigroup_num!(isize);
semigroup_num!(usize);
semigroup_num!(f32);
semigroup_num!(f64);

impl Semigroup for String {
    type T = String;
    fn mappend(t1: Self::T, t2: Self::T) -> Self::T {
        format!("{}{}", t1, t2)
    }
    fn add_and_own(self, t2: Self::T) -> Self::T {
        format!("{}{}", self, t2)
    }
}

semigroup!(Vec);

//impl<A: Semigroup> Semigroup for Box<A> {
//    type T = Box<A>;
//
//    fn mappend(t1: Self::T, t2: Self::T) -> Self::T {
//        let x = *t1;
//        let y = *t2;
//        let r = x.add_and_own(y);
//        Box::new(r)
//    }
//
//    fn add_and_own(self, t2: Self::T) -> Self::T {
//        let x: A = *self;
//        let y = *t2;
//        let r = x.add_and_own(y);
//        Box::new(r)
//    }
//}

#[test]
fn test_num_semigroup(){
    assert_eq!(1.add_and_own(2), 3);
}