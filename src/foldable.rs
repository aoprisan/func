pub trait Foldable {
    type T;
    fn foldr<Fun>(&self, acc: Self::T, f: Fun) -> Self::T where Fun: Fn(Self::T, &Self::T) -> Self::T;
}

///Macro to implement fold for iterables
#[macro_export]
macro_rules! foldable {
    ($t:ident) => {
        impl<A> Foldable for $t<A> {
            type T = A;
            fn foldr<Fun>(&self, acc: Self::T, f: Fun) -> Self::T
                where Fun: Fn(Self::T, &Self::T) -> Self::T
            {
                self.iter().fold(acc, f)
            }
        }
    }
}

//Implementation of Foldable for Vec
foldable!(Vec);

#[test]
fn test_foldable_vec(){
    assert_eq!(vec![1,2,3].foldr(0,|x,y| x + y), 6)
}

#[test]
fn test_foldable_vec_option() {
    use semigroup::Semigroup;
    use monoid::Monoid;

    assert_eq!(
        vec![Some(1),Some(2)].foldr(Option::<i32>::zero(),|x,y| x.add_and_own(*y) ),
        Some(3)
    );

}

//use hkt::HigherKindedType;
//
//pub fn sequence<R: Clone,
//    E: Clone,
//    V: Foldable + HigherKindedType<Result<R, E>>
//    >(vs: V) -> Result<Vec<R>, Vec<E>> {
//
//    vs.iter().fold(Ok(vec![]), |acc, v| {
//        match (acc, v.clone()) {
//            (Err(mut ess), Err(e2)) => {
//                ess.push(e2.clone());
//                Err(ess)
//            },
//            (Err(e), Ok(_)) => Err(e),
//            (Ok(_), Err(e)) => Err(vec![e.clone()]),
//            (Ok(mut vss), Ok(ok2)) => {
//                vss.push(ok2.clone());
//                Ok(vss)
//            },
//        }
//    })
//}
//
//#[test]
//fn test_sequence_generic() {
//    let pass: Vec<Result<u32,String>> = vec![Ok(1), Ok(2)];
//    assert_eq!( sequence(pass),  Ok(vec![1,2]) );
//
//    let err1: Vec<Result<u32,String>> = vec![Ok(1), Ok(2), Err("error".to_string())];
//    assert_eq!( sequence(err1),  Err(vec!["error".to_string()]) );
//
//    let err2: Vec<Result<u32,String>> = vec![Ok(1), Ok(2), Err("error".to_string()), Err("error2".to_string())];
//    assert_eq!( sequence(err2),  Err(vec!["error".to_string(), "error2".to_string()]) );
//}