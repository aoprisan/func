pub mod hkt;
#[macro_use] pub mod lazy;
pub mod option_ops;
pub mod result_ops;
pub mod validation;
pub mod semigroup;
pub mod monoid;
pub mod monad;
pub mod functor;
pub mod applicative;
pub mod foldable;
pub mod hlist;
pub mod show;
pub mod effect;
pub mod tailrec;
pub mod trampoline;
pub mod free;
pub mod io;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
