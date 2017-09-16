use std::marker::PhantomData;

use lazy::*;
use std::fmt::Debug;
use std::marker::Sized;

struct SimpleIO<A:Clone + Sized>{
    run_value: A
}

impl<A:Clone> SimpleIO<A> {

    pub fn run(&self) -> A {
        self.run_value.clone()
    }

    pub fn map<B: Clone + Sized, Fun: FnOnce(&A) -> B>(&self, f: Fun) -> SimpleIO<B> {
        SimpleIO { run_value: f(&self.run()) }
    }

    pub fn flat_map<B: Clone + Sized, Fun: FnOnce(&A) -> SimpleIO<B>>(&self, f: Fun) -> SimpleIO<B> {
        SimpleIO { run_value: f(&self.run()).run() }
    }

    pub fn unit(a:A) -> SimpleIO<A> {
        SimpleIO { run_value: a}
    }

}


#[test]
fn test_io_pure() {
    let io_pure = SimpleIO::unit(10);
    assert_eq!(io_pure.run(), 10);

    let io_pure = SimpleIO::unit(10).map(|x| x + 10i32);
    assert_eq!(io_pure.run(), 20);

    let io_pure = SimpleIO::unit(10).flat_map(|x| SimpleIO::unit(x + 30));
    assert_eq!(io_pure.run(), 40);
}

trait IO {

    type Output;

    fn run(self) -> Self::Output;

    fn flat_map<F, IOB: IO >(self, f: F) -> FlatMap<Self,F, IOB> where F: FnOnce(Self::Output) -> IOB, Self: Sized {
        FlatMap {
            sub: self,
            k: f,
            ghost: PhantomData
        }
    }


    fn map<F, B>(self, f: F) -> Map<Self,F> where F: FnOnce(Self::Output) -> B, Self: Sized {
        Map {
            sub: self,
            k: f
        }
    }
}

pub struct Unit<A>{
    result: A
}

impl<A> IO for Unit<A> {
    type Output = A;

    fn run(self) -> Self::Output {
        self.result
    }

}

pub fn unit<A>(a: A) -> Unit<A> {
    Unit { result: a }
}

pub struct Suspend<A>{
    result: Lazy<A>
}

impl<A> IO for Suspend<A> {
    type Output = A;

    fn run(self) -> Self::Output {
        self.result.eval()
    }

}

pub fn suspend<A>(a: Lazy<A>) -> Suspend<A> {
    Suspend { result: a }
}

pub struct FlatMap<IOA,F, IOB> {
    sub: IOA,
    k: F,
    ghost: PhantomData<IOB>
}

impl<IOA: IO, IOB: IO, F: FnOnce(IOA::Output) -> IOB> IO for FlatMap<IOA,F, IOB> {
    type Output = IOB::Output;

    fn run(self) -> Self::Output {
        let a = self.sub.run();
        let iob = (self.k)(a);
        iob.run()

    }
}

pub struct Map<IOA, F> {
    sub: IOA,
    k: F
}

impl<B, IOA: IO, F: FnOnce(IOA::Output) -> B> IO for Map<IOA, F> {
    type Output = B;

    fn run(self) -> Self::Output {
        let a = self.sub.run();
        let b = (self.k)(a);
        b
    }
}


//http://degoes.net/articles/only-one-io

//struct Async<A,E> {
//    register: Box<
//        Fn(
//            Box< Fn(Result<A,E>) -> Box< IO< Output = ()>>>
//        ) -> Box<IO<Output = ()>>
//    >
//}
//
//impl<A: 'static, E: 'static> Async<A,E> {
//    pub fn map<B: 'static, F: Fn(A) -> B + 'static >(self, ab: F) -> Async<B, E> {
//
//        Async {
//            register: Box::new(move |cb| {
//                let call = cb;
//                (self.register)(Box::new(move |res| {
//                    match res {
//                        Err(e) => call(Err(e)),
//                        Ok(a) => call(Ok(ab(a)))
//                    }
//                }))
//            })
//        }
//    }
//}


//Playing with Generics, it doesn't compile, as it does not allow me to
// 'let tmp2: FB = move |cb: FIB | {...}' assign a closure to variable of a trait type

//trait Async {
//    type Output;
//    type O1;
//
//    fn map<Fun: Fn(Self::Output) -> Self::O1 >(self: Self, ab: Fun) -> Self;
//}
//
//struct AsyncContainer<A, E, F, FI, B, FIB, FB> {
//    register: F,
//    ghost1: PhantomData<A>,
//    ghost2: PhantomData<E>,
//    ghost3: PhantomData<FI>,
//    ghost4: PhantomData<B>,
//    ghost5: PhantomData<FIB>,
//    ghost6: PhantomData<FB>
//}
//
//impl<
//    A,E: Debug, B,
//    IOUnit: IO<Output = ()>,
//    FI: Fn(Result<A, E>) -> IOUnit,
//    FIB: Fn(Result<B, E>) -> IOUnit,
//    F: Fn(FI) -> IOUnit,
//    FB: Fn(FI) -> IOUnit,
//> Async for AsyncContainer<A, E, F, FI, B, FIB, FB> {
//
//    type Output = A;
//    type O1 = B;
//
//    fn map<Fun: Fn(Self::Output) -> Self::O1 >(self, ab: Fun) -> Self {
//        let tmp2: FB = move |cb: FIB | {
//            let tmp = |res: Result<A,E>| {
//                match res {
//                    Err(e) => cb(Err(e)),
//                    Ok(a) => cb(Ok(ab(a)))
//                }
//            };
//            (self.register)(tmp)
//        };
//        AsyncContainer {
//            register: tmp2,
//
//            ghost1: PhantomData,
//            ghost2: PhantomData,
//            ghost3: PhantomData,
//            ghost4: PhantomData,
//            ghost5: PhantomData,
//            ghost6: PhantomData
//        }
//    }
//}

#[test]
fn test_flat_map() {
    let r = unit(10).flat_map(|x| unit(x + 10)).run();
    assert_eq!(r, 20);
}

#[test]
fn test_map() {
    let r = unit(10).map(|x| x + 10 ).run();
    assert_eq!(r, 20);
}

#[test]
fn test_map_and_flat_map() {
    let r = unit(10).map(|x| x + 10 ).flat_map(|x| unit(x + 10)).run();
    assert_eq!(r, 30);
}

#[test]
fn test_suspend_map_and_flat_map() {
    let r = suspend(lazy!(10)).map(|x| x + 10 ).flat_map(|x| unit(x + 10)).run();
    assert_eq!(r, 30);
}
