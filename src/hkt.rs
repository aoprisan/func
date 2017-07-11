trait HKT<F, V> {
    type Current;
    type Output;
    type FOutput;
}

/// macro to lift types
#[macro_export]
macro_rules! hkt {
    ($t:ident) => {
       impl<T, V> HKT<$t<T>, V> for $t<T> {
            type Current = T;
            type Output = V;
            type FOutput = $t<V>;
        }
    }
}

hkt!(Option);

//impl<T, V> HKT<Option<T>, V> for Option<T> {
//    type Current = T;
//    type Output = V;
//    type FOutput = Option<V>;
//}

trait Functor<F, V> : HKT<F, V>{
    fn fmap<Fun>(&self, f: Fun) -> Self::FOutput where Fun: FnOnce(&Self::Current) -> Self::Output;
}

impl<T, V> Functor<Option<T>,V> for Option<T> {
    fn fmap<Fun>(&self, f: Fun) -> Self::FOutput where Fun: FnOnce(&Self::Current) -> Self::Output {
        match *self {
            Some(ref x) => Some(f(x)),
            None => None,
        }
    }
}

trait Monad<F,V> : HKT<F,V>{
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: FnOnce(&Self::Current) -> Self::FOutput;
}

impl<T,V> Monad<Option<T>,V> for Option<T> {
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: FnOnce(&Self::Current) -> Self::FOutput {
        match *self {
            Some(ref v) => f(v),
            None => None,
        }
    }
}

#[test]
pub fn test_functor() {
    assert_eq!(Some(10).fmap(|x| x + 1), Some(11));
    assert_eq!(None.fmap(|x| x + 1), None);
}

#[test]
pub fn test_monad() {
    let none: Option<u32> = None;
    assert_eq!(Some(10 as u32).bind(|x| Some(x + 1)), Some(11));
    assert_eq!(None.bind(|x| Some(x + 1)), None);
    assert_eq!(Some(10).bind(|x| none), None);
}

