use hkt::*;
use functor::*;

/// <Self as HKT<Fun>>::FOutput === F[Fun]
/// <Self as HKT<V>>::FOutput === F[V]
/// Fun: Fn(&<Self as HKT<V>>::Current) -> V === T =>V
///
pub trait Apply<V> : Functor<V> {
    //e.g. for Option: f: Option<Fun> -- in Scala F[Fun]
    fn ap<Fun>(&self, f: <Self as HigherKindedType<Fun>>::FOutput) -> <Self as HigherKindedType<V>>::FOutput where Fun: Fn(&<Self as HigherKindedType<V>>::Current) -> V, Self: HigherKindedType<Fun>; //kinda ugly
}

pub trait Applicative<V> : Apply<V> {
    fn point(a: Self::Output) -> Self::FOutput;
}

impl<T,V> Apply<V> for Option<T> {

    fn ap<Fun>(&self, f: <Self as HigherKindedType<Fun>>::FOutput) -> <Self as HigherKindedType<V>>::FOutput where Fun: Fn(&<Self as HigherKindedType<V>>::Current) -> V {
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


impl<T,V,E : Clone> Apply<V> for Result<T,E> {
    fn ap<Fun>(&self, f: <Self as HigherKindedType<Fun>>::FOutput) -> <Self as HigherKindedType<V>>::FOutput where Fun: Fn(&<Self as HigherKindedType<V>>::Current) -> V {
        match *self {
            Ok(ref x) => match f {
                Ok(fs) => Ok(fs(x)),
                Err(ref e) => Err(e.clone())
            },
            Err(ref e) => Err(e.clone())
        }
    }
}

impl<T,V,E : Clone> Applicative<V> for Result<T,E> {
    fn point(a: Self::Output) -> Self::FOutput {
        Ok(a)
    }
}


impl<T,V> Apply<V> for Vec<T> {

    fn ap<Fun>(&self, f: <Self as HigherKindedType<Fun>>::FOutput) -> <Self as HigherKindedType<V>>::FOutput where Fun: Fn(&<Self as HigherKindedType<V>>::Current) -> V {
        self.iter().zip(f).map(|x| {
            let (e,f) = x;
            f(e)
        }).collect()
    }

}

impl<T,V> Applicative<V> for Vec<T> {
    fn point(a: Self::Output) -> Self::FOutput {
        vec![a]
    }
}


#[test]
fn test_option(){
    assert_eq!(Option::<i32>::point(10), Some(10));
    assert_eq!(Some(10).ap(Some(|x: &u32| *x + 1)), Some(11));
}

#[test]
fn test_result(){
    assert_eq!(Vec::<i32>::point(10), vec![10]);
    assert_eq!(vec![10].ap(vec![(|x: &u32| *x + 1)]), vec![11]);
}

