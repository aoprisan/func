use applicative::*;

pub trait Monad<V> : Applicative<V>{
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::FOutput;
}

impl<T,V> Monad<V> for Option<T> {
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::FOutput {
        match *self {
            Some(ref v) => f(v),
            None => None,
        }
    }
}

impl<T,V,E: Clone> Monad<V> for Result<T, E> {
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::FOutput {
        match *self {
            Ok(ref v) => f(v),
            Err(ref e) => Err(e.clone()),
        }
    }
}

impl<T,V> Monad<V> for Vec<T> {
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::FOutput {
        self.iter().flat_map(f).collect()
    }
}

impl<T,V> Monad<V> for Box<T> {
    fn bind<Fun>(&self, f: Fun) -> Self::FOutput where Fun: Fn(&Self::Current) -> Self::FOutput {
        f(self)
    }
}

#[test]
pub fn test_monad_option() {
    let none: Option<u32> = None;
    assert_eq!(Some(10 as u32).bind(|x| Some(x + 1)), Some(11));
    assert_eq!(None.bind(|x| Some(x + 1)), None);
    assert_eq!(Some(10).bind(|_| none), None);
}

#[test]
pub fn test_monad_result() {
    assert_eq!(Result::<u32,u32>::point(10).bind(|x| Ok(x + 1)), Ok(11));
    assert_eq!(Err(10).bind(|x| Ok(x + 1)), Err(10));
    assert_eq!(Result::<u32,u32>::point(10).bind(|_| Err(10 as u32) as Result<u32, u32>), Err(10));
}

#[test]
pub fn test_monad_box() {
    assert_eq!(Box::new(10 as u32).bind(|x| Box::new(x + 1)), Box::new(11));
}