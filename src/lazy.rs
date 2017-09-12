pub struct Lazy<A> {
    pub computation: Box<Fn() -> A>
}

impl<A> Lazy<A> {

    pub fn new<F>(v: F) -> Lazy<A> where F:'static + Fn() -> A {
        Lazy { computation: Box::new(v) }
    }

    pub fn eval(&self) -> A {
        let r = (self.computation)();
        r
    }

    pub fn map<B, F >(self, f: F) -> Lazy<B> where F: 'static + Fn(A) -> B, A: 'static {
        Lazy::new(move || { f(self.eval()) })
    }

    pub fn flat_map<B, F >(self, f: F) -> Lazy<B> where F: 'static + Fn(A) -> Lazy<B>, A: 'static {
        Lazy::new(move || { f(self.eval()).eval() })
    }
}

#[macro_export]
macro_rules! lazy {
    ($x: expr) => { $crate::lazy::Lazy { computation: Box::new(move || { $x } ) } };
}

#[test]
fn test_map_flat_map(){
    let x = lazy!(10).map(|y| y + 10).flat_map(|y| lazy!(y + 100));
    assert_eq!(x.eval(), 120);
}