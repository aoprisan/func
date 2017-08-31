//! HList
//! Inspired by https://github.com/Sgeo/hlist/blob/master/src/lib.rs
//! and https://github.com/lloydmeta/frunk

trait HList : Sized {
    fn prepend<H>(self, h: H) -> HCons<H, Self> {
        HCons {
            head: h,
            tail: self
        }
    }
}

#[derive(PartialEq, Debug)]
struct HNil;

impl HList for HNil {

}

struct HCons<T,V : HList> {
    pub head: T,
    pub tail: V
}

impl<T,V : HList> HList for HCons<T,V> {

}

#[macro_export]
macro_rules! hlist {
    ( ) => { $crate::hlist::HNil } ;

    ($x: expr) => { $crate::hlist::HCons { head: $x, tail: $crate::hlist::HNil } };

    ($x: expr, $($y: expr),*) => { $crate::hlist::HCons { head: $x, tail: hlist![$($y),*] } };
}


#[macro_export]
macro_rules! Hlist {
    ( ) => { $crate::hlist::HNil } ;

    ($x: ty) => { $crate::hlist::HCons<$x, $crate::hlist::HNil> };

    ($x: ty, $($y: ty),*) => { $crate::hlist::HCons<$x, Hlist![$($y),*]> };
}

#[test]
fn test_prepend(){

    fn test_hlist_type(l: Hlist!(String, i32)) -> String {
        l.head
    }

    let list = HNil.prepend(100).prepend("string".to_string());
    assert_eq!(list.head, "string".to_string());
    assert_eq!(list.tail.head, 100);
    assert_eq!(list.tail.tail, HNil);

    let mlist = hlist!["string".to_string(), 100];
    assert_eq!(mlist.head, "string".to_string());
    assert_eq!(mlist.tail.head, 100);
    assert_eq!(mlist.tail.tail, HNil);
    let mmlist: Hlist!(String,i32) = HNil.prepend(100).prepend("string".to_string());
    assert_eq!(mmlist.head, "string".to_string());
    assert_eq!(mmlist.tail.head, 100);
    assert_eq!(mmlist.tail.tail, HNil);

    assert_eq!(test_hlist_type(mmlist), "string".to_string());
}




#[derive(Debug, PartialEq)]
pub enum EHList<T,V> {
    Nil,
    Cons(T,V)
}

pub fn nil() -> EHList<(),()> {
    EHList::Nil
}

pub fn cons<T,V>(t:T,v: V) -> EHList<T,V> {
    EHList::Cons(t,v)
}

impl<T,V> EHList<T,V> {
    pub fn prepend<N>(self, n: N) -> EHList<N, Self> {
        EHList::Cons(n, self)
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            &EHList::Cons(ref h, _) => Some(h),
            &EHList::Nil => None
        }
    }

    pub fn tail(&self) -> Option<&V> {
        match self {
            &EHList::Cons(_, ref rest) => Some(rest),
            &EHList::Nil => None
        }
    }

}

#[test]
fn test_prepend_enum(){
    let list = nil().prepend(100).prepend("string".to_string());
    assert_eq!(list.head(), Some("string".to_string()).as_ref());
    assert_eq!(list.tail().unwrap().head(), Some(100).as_ref());
    assert_eq!(list.tail().unwrap().tail().unwrap(), &nil());

}