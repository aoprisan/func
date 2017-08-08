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


enum EHList<T,V> {
    Nil,
    Cons(T,V)
}

impl<T,V> EHList<T,V> {
    pub fn nil() -> EHList<T,V> {
        EHList::Nil
    }

    pub fn cons(t:T,v: V) -> EHList<T,V> {
        EHList::Cons(t,v)
    }

    pub fn prepend<N>(self, n: N) -> EHList<N, Self> {
        EHList::cons(n, self)
    }

}

//#[test]
//fn test_prepend_enum(){
//    let list = EHList::nil().prepend(100).prepend("string".to_string());
//    let EHList::Cons(str, rest) = list;
//    assert_eq!(str, "string".to_string());
//    //assert_eq!(list.1.0, 100);
//    //assert_eq!(list.tail.tail, HNil);
//}