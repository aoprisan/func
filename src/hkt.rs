
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BinaryHeap, BTreeSet, HashSet};

///Encoding of higher kinded types, by capturing the type constructor, its current type parameter
/// and the output type parameter. This encoding defines types to support the definition of operations
/// from F<T> to F<V>.
/// F is the type constructor (F<_>)
/// V is the output type inside of F.
/// The current type inside of F is defined at implementation time.
pub trait HKT<F, V> {
    type Current; //current type in F
    type Output; // output type in F
    type FOutput; // F<Output>
}

/// macro to generate higher kinded encodings for type constructors with one parameter
/// such as Option<_>, Vec<_>
///
/// Option implementation
/// impl<T, V> HKT<Option<T>, V> for Option<T> {
///    type Current = T;
///    type Output = V;
///    type FOutput = Option<V>;
///}
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

#[macro_export]
macro_rules! hkt_partial_left {
    ($t:ident) => {
        impl<T,V,E> HKT<$t<T,E>, V> for $t<T, E> {
            type Current = T;
            type Output = V;
            type FOutput = $t<V, E>;
        }
    }
}

#[macro_export]
macro_rules! hkt_partial_right {
    ($t:ident) => {
        impl<T,V,L> HKT<$t<L,T>, V> for $t<L,T> {
            type Current = T;
            type Output = V;
            type FOutput = $t<L, V>;
        }
    }
}



hkt!(Vec);
hkt!(Option);
hkt!(Box);
hkt!(LinkedList);
hkt!(BinaryHeap);
hkt!(BTreeSet);
hkt!(VecDeque);
hkt!(HashSet);

hkt_partial_left!(Result);

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
    assert_eq!(Some(10).bind(|_| none), None);
}

