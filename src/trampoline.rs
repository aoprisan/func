use tailrec::*;
use tailrec::TailRec;

use lazy::*;

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

