
///Encoding of higher kinded types, by capturing the type constructor, its current type parameter
/// and the output type parameter. This encoding defines types to support the definition of operations
/// from F<T> to F<V>.
/// F is the type constructor (F<_>)
/// V is the output type inside of F.
/// The current type inside of F is defined at implementation time.
pub trait HKT<V> {
    type Current; //current type in F
    type Output; // output type in F
    type FOutput; // F<Output>
}

/// macro to generate higher kinded encodings for type constructors with one parameter
/// such as Option<_>, Vec<_>
///
/// Option implementation
/// impl<T, V> HKT<V> for Option<T> {
///    type Current = T;
///    type Output = V;
///    type FOutput = Option<V>;
///}
#[macro_export]
macro_rules! hkt {
    ($t:ident) => {
       impl<T, V> HKT<V> for $t<T> {
            type Current = T;
            type Output = V;
            type FOutput = $t<V>;
        }
    }
}

#[macro_export]
macro_rules! hkt_partial_left {
    ($t:ident) => {
        impl<T,V,E> HKT<V> for $t<T, E> {
            type Current = T;
            type Output = V;
            type FOutput = $t<V, E>;
        }
    }
}

#[macro_export]
macro_rules! hkt_partial_right {
    ($t:ident) => {
        impl<T,V,L> HKT<V> for $t<L,T> {
            type Current = T;
            type Output = V;
            type FOutput = $t<L, V>;
        }
    }
}



hkt!(Vec);
hkt!(Option);

hkt_partial_left!(Result);



