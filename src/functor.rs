use hkt::*;

pub trait Functor<V> : HigherKindedType<V>{
    fn fmap<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::Output;
}

///A quick macro to functorize types implementing Iter
#[macro_export]
macro_rules! functorize {
    ($t:ident) => {
        impl<T,V> Functor<V> for $t<T> {
            fn fmap<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::Output {
                self.iter().map(f).collect::<$t<V>>()
            }
        }
    }
}

functorize!(Vec);

impl<T, V> Functor<V> for Option<T> {
    fn fmap<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::Output {
        match self {
            &Some(ref x) => Some(f(x)),
            &None => None,
        }
    }
}

impl<T, V, E : Clone> Functor<V> for Result<T,E> {
    fn fmap<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::Output {
        match self {
            &Ok(ref x) => Ok(f(x)),
            &Err(ref e) => Err(e.clone()),
        }
    }
}


#[test]
pub fn test_functor_option() {
    assert_eq!(Some(10).fmap(|x| x + 1), Some(11));
    assert_eq!(None.fmap(|x| x + 1), None);
}

#[test]
pub fn test_functor_result() {
    use result_ops::*;

    assert_eq!(10.to_ok::<i32>().fmap(|x| x + 1), Ok(11));
    assert_eq!(10.to_err::<i32>().fmap(|x| x + 1), Err(10));
}

#[test]
pub fn test_functor_vec() {
    assert_eq!(vec![1,2,3].fmap(|x| x + 1), vec![2,3,4]);
}