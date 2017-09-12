use std::marker::PhantomData;

use lazy::*;

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


//type F<A> = Fn() -> A;
//
//enum Free<A> {
//    Pure(A),
//    Roll(Box<F<Free<A>>>),
//    Bind(Box<Free<*const u8>>, Box<Fn(*const u8) -> Free<A>>),
//}
//
//impl<'a,A> Free<A> {
//    #[inline]
//    fn resume(self) -> Result<A,F<Free<A>>> {
//        match self {
//            Free::Pure(val) => Ok (val),
//            Free::Roll(thk) => Err(thk),
//            Free::Bind(mon2, con2) => {
//                match mon2 {
//                    Free::Pure(val) => con2(val).resume(),
//                    Free::Roll(thk1) => {
//                        Err({
//                            // FIXME: Without the annotation, rustc thinks we
//                            // need a Send bound. Maybe report upstream?
//                            let thk2: F<Free<A>> = || {
//                                Free::Bind(Box::new(thk1()), con2)
//                            };
//                            thk2
//                        })
//                    },
//                    Free::Bind(bmon1, con1) => {
//                        Free::Bind(bmon1, |ptr1|
//                            Free::Bind(Box::new(con1(ptr1)), con2))
//                        .resume()
//                    },
//                }
//            },
//        }
//    }
//
//    #[inline]
//    fn go<FN>(mut self, f:FN) -> A where FN: Fn(F<Free<A>>) -> Free<A>{
//        let res: A;
//        loop { match self.resume() {
//            Ok (val) => { res  = val; break },
//            Err(cmp) => { self = f(cmp)     },
//        }};
//        res
//    }
//
//
//    #[inline]
//    fn run(self) -> A {
//        self.go(force)
//    }
//}

//#[inline(always)]
//fn coe_m<'a,A>(t:Trampoline<'a,A>) -> Free<'a,*const u8> {
//    let Trampoline(m) = t;
//    match m {
//        Pure(a) => Pure(unsafe { transmute(box a) }),
//        Roll(k) => Roll(proc() coe_m(Trampoline(k()))),
//        Bind(m,f) => Bind(m, proc(x) coe_m(Trampoline(f(x)))),
//    }
//}
//
//#[inline(always)]
//fn coe_f<'a,A,B>(k:proc(A):'a -> Trampoline<'a,B>) -> proc(*const u8):'a -> Free<'a,B> {
//proc(x) {
//let box a: Box<A> = unsafe { transmute(x) };
//let Trampoline(mb) = k(a);
//mb
//}
//}

//// Public API
//
//pub type Thunk<A> = Fn()-> A;
//
//#[inline(always)]
//pub fn force<'a,A>(k:Thunk<A>) -> A {
//    k()
//}


//pub enum IO<T> {
//    Pure(T),
//    Suspend(Box<Fn() -> T>),
//    FlatMap(Box<Fn(T) -> IO<T>>),
//}
//
//impl<T: 'static> IO<T> {
//    pub fn pure_io(t:T) -> IO<T> {
//        IO::Pure(t)
//    }
//
//    pub fn suspend<F>(f: F)  -> IO<T> where F : 'static + Fn() -> T {
//        IO::Suspend(Box::new(f))
//    }
//
//    pub fn flat_map_io<F>(f: F)  -> IO<T> where F : 'static + Fn() -> IO<T> {
//        IO::FlatMap(Box::new(f))
//    }
//
//    pub fn unsafe_perform_io(self) -> T {
//        match self {
//            IO::Pure(t) => t,
//            IO::Suspend(f) => f(),
//            IO::FlatMap(f) => f().unsafe_perform_io()
//        }
//    }
//
//    pub fn map<V: 'static,F: 'static + Fn(T) -> V>(self, f: F) -> IO<V>  {
//        match self {
//            IO::Pure(t) => IO::pure_io(f(t)),
//            IO::Suspend(k) => IO::suspend(move || f(k())),
//            IO::FlatMap(k) => IO::flat_map_io(move)
//        }
//    }
//}
//
//
//#[test]
//fn test_io_pure() {
//    let io_pure = IO::Pure(10);
//    assert_eq!(io_pure.unsafe_perform_io(), 10);
//
//    let io_suspend = IO::Suspend(Box::new(|| 10));
//    assert_eq!(io_suspend.unsafe_perform_io(), 10);
//}
//
//#[test]
//fn test_io_pure_helpers() {
//    let io_pure = IO::pure_io(10);
//    assert_eq!(io_pure.unsafe_perform_io(), 10);
//
//    let io_suspend = IO::suspend(|| 10);
//    assert_eq!(io_suspend.unsafe_perform_io(), 10);
//}
//
//#[test]
//fn test_map() {
//    let io_pure = IO::pure_io(10).map(|x| x + 10);
//    assert_eq!(io_pure.unsafe_perform_io(), 20);
//
//    let io_suspend = IO::suspend(|| 10).map(|x| x + 10);
//    assert_eq!(io_suspend.unsafe_perform_io(), 20);
//
//    let y = 20;
//    let io_suspend = IO::suspend(|| 10).map(move |x| x + y);
//    assert_eq!(io_suspend.unsafe_perform_io(), 30);
//}
//
//
