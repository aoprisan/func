use hkt::*;
use functor::*;

/// <Self as HKT<Fun>>::FOutput === F[Fun]
/// <Self as HKT<V>>::FOutput === F[V]
/// Fun: Fn(&<Self as HKT<V>>::Current) -> V === T =>V
///
pub trait Apply<V> : Functor<V> {
    //e.g. for Option: f: Option<Fun> -- in Scala F[Fun]
    fn ap<Fun>(&self, f: <Self as HKT<Fun>>::FOutput) -> <Self as HKT<V>>::FOutput where Fun: Fn(&<Self as HKT<V>>::Current) -> V, Self: HKT<Fun>; //kinda ugly
}

pub trait Applicative<V> : Apply<V> {
    fn point(a: Self::Output) -> Self::FOutput;
}

impl<T,V> Apply<V> for Option<T> {

    fn ap<Fun>(&self, f: <Self as HKT<Fun>>::FOutput) -> <Self as HKT<V>>::FOutput where Fun: Fn(&<Self as HKT<V>>::Current) -> V {
        match *self {
            Some(ref x) => match f {
                Some(fs) => Some(fs(x)),
                None => None
            },
            None => None
        }
    }

}

impl<T,V> Applicative<V> for Option<T> {
    fn point(a: Self::Output) -> Self::FOutput {
        Some(a)
    }
}


#[test]
fn test_option(){
    assert_eq!(Option::<i32>::point(10), Some(10));
    assert_eq!(Some(10).ap(Some(|x: &u32| *x + 1)), Some(11));
}