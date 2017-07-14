use applicative::*;

pub trait Monad<V> : Applicative<V>{
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: FnOnce(&Self::Current) -> Self::FOutput;
}

impl<T,V> Monad<V> for Option<T> {
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: FnOnce(&Self::Current) -> Self::FOutput {
        match *self {
            Some(ref v) => f(v),
            None => None,
        }
    }
}


#[test]
pub fn test_monad() {
    let none: Option<u32> = None;
    assert_eq!(Some(10 as u32).bind(|x| Some(x + 1)), Some(11));
    assert_eq!(None.bind(|x| Some(x + 1)), None);
    assert_eq!(Some(10).bind(|_| none), None);
}