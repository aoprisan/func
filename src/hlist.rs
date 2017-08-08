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

struct HCons<T,V> {
    pub head: T,
    pub tail: V
}

impl<T,V> HList for HCons<T,V> {

}


#[test]
fn test_prepend(){
    let list = HNil.prepend(100).prepend("string".to_string());
    assert_eq!(list.head, "string".to_string());
    assert_eq!(list.tail.head, 100);
    assert_eq!(list.tail.tail, HNil);
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