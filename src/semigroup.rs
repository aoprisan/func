pub trait Semigroup {
    type T;
    fn add_and_own(self, t2: Self::T) -> Self::T;
}


///Macro to implement semigroup for numerics
#[macro_export]
macro_rules! semigroup_num {
    ($t:ident) => {
        impl Semigroup for $t {
            type T = $t;

            fn add_and_own(self, t2: Self::T) -> Self::T {
                self + t2
            }

        }
    }
}

///Macro for implementing SemiGroup for types that implement Extend
#[macro_export]
macro_rules! semigroup {
    ($t:ident) => {
        impl<T> Semigroup for $t<T> {
            type T = $t<T>;

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
    fn add_and_own(self, t2: Self::T) -> Self::T {
        format!("{}{}", self, t2)
    }
}

semigroup!(Vec);

impl<A: Semigroup<T = A>> Semigroup for Box<A> {
    type T = Box<A>;


    fn add_and_own(self, t2: Self::T) -> Self::T {
        let x = *self as A;
        let y = *t2 as A;
        let r = x.add_and_own(y);
        Box::new(r)
    }
}

#[test]
fn test_num_semigroup(){
    assert_eq!(1.add_and_own(2), 3);
}

#[test]
fn test_box_semigroup(){
    assert_eq!(Box::new(1).add_and_own(Box::new(2)), Box::new(3));
}