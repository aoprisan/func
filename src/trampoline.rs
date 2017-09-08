use tailrec::*;
use tailrec::TailRec;

pub struct Lazy<A> {
    computation: Box<Fn() -> A>
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

pub enum Computation<A> {
    Done(A),
    Continue(Lazy<Computation<A>>)
}


pub fn run<A>(c: Computation<A>) -> A {
    c.rec(|comp|{ match comp {
        Computation::Done(value) => RecursionState::Done(value),
        Computation::Continue(la) => RecursionState::Continue(la.eval())
    }})
}

#[test]
fn test_odd_even(){

    fn even(i: u32) -> Computation<bool> { match i {
        0 => Computation::Done(true),
        _ => Computation::Continue(lazy!( odd(i - 1)))//Computation::Continue(Lazy::new(move || odd(i - 1)))
    }}

    fn odd(i: u32) -> Computation<bool> { match i {
        0 => Computation::Done(false),
        _ => Computation::Continue(lazy!(even(i - 1))) //Computation::Continue(Lazy::new(move || even(i - 1)))
    }}

    assert_eq!(run(even(3000)), true);
    assert_eq!(run(even(3000000)), true);
}

