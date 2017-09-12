use tailrec::*;
use tailrec::TailRec;

use lazy::*;

pub enum Free<A> {
    Return(A),
    Suspend(Lazy<A>),
// Need a Box on first Free as it's recursive without indirection
//    FlatMap(Free<A>, Box<Fn(A) -> Free<A> >)
//    FlatMap(Box<Free<A>>, Box<FnOnce(A) -> Free<A> >)
}

impl<A> Free<A> {
//    fn flat_map(self, f: Box<FnOnce(A) -> Free<A>>) -> Free<A>   {
//        Free::FlatMap(Box::new(self), f)
//    }
}

pub fn run<A: 'static>(c: Free<A>) -> A {
    tail_rec(c, |free| match free {
        Free::Return(a) => RecursionState::Done(a),
        Free::Suspend(a) => RecursionState::Done(a.eval())
    })
}

//pub fn run<A: 'static>(c: Free<A>) -> A {
//    c.rec(|comp|{ match comp {
//        Free::Return(value) => RecursionState::Done(value),
//        Free::Suspend(la) => RecursionState::Done(la.eval()),
//        Free::FlatMap(comp, k) => match *comp {
//            Free::Return(value) => {
//                let xxx = (k)(value);
//                RecursionState::Continue(xxx)
//            },
//            Free::Suspend(la) => RecursionState::Continue(k(la.eval())),
//            Free::FlatMap(comp2, k2) => {
////                let kkcomp: Free<A> = *comp2;
////                let kkk: Box<FnMut(A) -> Free<A>> = Box::new(move |a| k2(a).flat_map(k));
////                RecursionState::Continue(kkcomp.flat_map(kkk))
//                RecursionState::Continue(*comp2)
//            }
//        }
//    }})
//}

#[test]
fn test_odd_even(){

}

