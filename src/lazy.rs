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
}

#[macro_export]
macro_rules! lazy {
    ($x: expr) => { $crate::trampoline::Lazy { computation: Box::new(move || { $x } ) } };
}
