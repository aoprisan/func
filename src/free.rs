use tailrec::*;
use tailrec::TailRec;

use lazy::*;
use std::rc::Rc;
use std::mem::*;

//pub enum ETFree<A> {
//    Return(A),
//    Suspend(Lazy<A>),
//    FlatMap(Box<ETFree<*const u8>>, Rc<Fn(*const u8) -> ETFree<A> >)
//}
//
//fn magick_lhs<A: 'static>(free: ETFree<A>) -> ETFree<*const u8> { match free {
//    ETFree::Return(a) => ETFree::Return( unsafe { transmute( Box::new(a) ) } ),
//    ETFree::Suspend(a) => ETFree::Suspend( lazy![ unsafe { transmute( Box::new(a) ) } ] ),
//    ETFree::FlatMap(m, k) => ETFree::FlatMap( m, Rc::new( move |x| magick_lhs( k(x) ) ) )
//}}

pub enum Free<A> {
    Return(A),
    Suspend(Lazy<A>),
// Need a Box on first Free as it's recursive without indirection
// Rc required as Box cannot be moved into Fn
//    FlatMap(Free<A>, Box<Fn(A) -> Free<A> >)
    FlatMap(Box<Free<A>>, Rc<Fn(A) -> Free<A> >)
//    FlatMap(Box<Free<*const u8>>, Rc<Fn(*const u8) -> Free<A> >)
}



impl<A> Free<A> {
    fn flat_map_rc(self, f: Rc<Fn(A) -> Free<A>>) -> Free<A>   {
        Free::FlatMap(Box::new(self), f)
    }

    fn flat_map<F>(self, f: F) -> Free<A> where F: Fn(A) -> Free<A> + 'static  {
        self.flat_map_rc(Rc::new(f))
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
                        free2.flat_map_rc(
                            Rc::new( move |a| k2(a).flat_map_rc(k.clone()))
                        )
                    )
                }
            }

        }
    })
}

pub fn free_return<A>(a:A) -> Free<A> {
    Free::Return(a)
}

pub fn free_suspend<A>(a: Lazy<A>) -> Free<A> {
    Free::Suspend(a)
}

#[test]
fn test_free(){
    let x = free_return(10).flat_map(|x| free_suspend(lazy!( x + 100 )));
    assert_eq!(run(x), 110);


    let deep_size = 10000; //10000000

    let deepf = (1..deep_size).fold(free_return(10),|free, elem| {
        free.flat_map(move |x| free_suspend(lazy!(x + elem)) )
    });

    assert_eq!(run(deepf), 49995010);

//        let debug = (1..5).fold(free_return(10),|free, elem| {
//            free.flat_map(move |x| free_suspend(lazy!(x + elem)) )
//        });
//
//        assert_eq!(run(debug), 20);

}

