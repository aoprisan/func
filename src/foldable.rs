pub trait Foldable {
    type T;
    fn foldr<Fun>(&self, acc: Self::T, f: Fun) -> Self::T where Fun: Fn(Self::T, &Self::T) -> Self::T;
}

///Macro to implement fold for iterables
#[macro_export]
macro_rules! foldable {
    ($t:ident) => {
        impl<A> Foldable for $t<A> {
            type T = A;
            fn foldr<Fun>(&self, acc: Self::T, f: Fun) -> Self::T
                where Fun: Fn(Self::T, &Self::T) -> Self::T
            {
                self.iter().fold(acc, f)
            }
        }
    }
}

//Implementation of Foldable for Vec
foldable!(Vec);

#[test]
fn test_foldable_vec(){
    assert_eq!(vec![1,2,3].foldr(0,|x,y| x + y), 6)
}