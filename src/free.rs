use tailrec::*;
use tailrec::TailRec;

use lazy::*;
use std::rc::Rc;

pub enum Free<A> {
    Return(A),
    Suspend(Lazy<A>),
// Need a Box on first Free as it's recursive without indirection
// Rc required as Box cannot be moved into Fn
//    FlatMap(Free<A>, Box<Fn(A) -> Free<A> >)
    FlatMap(Box<Free<A>>, Rc<Fn(A) -> Free<A> >)
}

impl<A> Free<A> {
    fn flat_map(self, f: Rc<Fn(A) -> Free<A>>) -> Free<A>   {
        Free::FlatMap(Box::new(self), f)
    }
}

pub fn run<A: 'static>(c: Free<A>) -> A {
    tail_rec(c, |free| match free {
        Free::Return(a) => RecursionState::Done(a),
        Free::Suspend(a) => RecursionState::Done(a.eval()),
        Free::FlatMap(free, k) => {
            let f = *free;
            match f {
                Free::Return(a) => RecursionState::Continue(k(a)),
                Free::Suspend(a) => RecursionState::Continue(k(a.eval())),
                Free::FlatMap(free2, k2) => {
                    RecursionState::Continue(
                        free2.flat_map(
                            Rc::new( move |a| k2(a).flat_map(k.clone()))
                        )
                    )
                }
            }

        }
    })
}


#[test]
fn test_free(){
    let x = Free::Return(10).flat_map(Rc::new(|x| Free::Suspend(lazy!( x + 100 ))));
    assert_eq!(run(x), 110);
}

