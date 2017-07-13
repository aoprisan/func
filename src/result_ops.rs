pub trait ResultExtOps {
    fn to_ok<E>(self) -> Result<Self,E> where Self: Sized {
        Ok(self)
    }

    fn to_err<T>(self) -> Result<T,Self> where Self: Sized {
        Err(self)
    }
}

impl<T> ResultExtOps for T {}


#[test]
pub fn test_ext_result_ops() {
    assert_eq!(Ok(10), 10.to_ok::<i32>());
    assert_eq!(Err(10), 10.to_err::<i32>());
}

pub trait ResultOps<T,V,E> {
    fn fold<F, FE>(&self, z: FE, nz: F) -> V where F: FnOnce(&T)-> V, FE: FnOnce(&E)-> V;
}

impl<T,V,E> ResultOps<T,V,E> for Result<T,E> {
    fn fold<F, FE>(&self, z: FE, nz: F) -> V where F: FnOnce(&T)-> V, FE: FnOnce(&E)-> V {
        match self.as_ref() {
            Ok(ref x) => nz(x),
            Err(ref e) => z(e)
        }
    }
}

#[test]
pub fn test_ops() {
    assert_eq!(10.to_ok::<i32>().fold(|_| 0, |_| 666), 666);
    assert_eq!(10.to_err::<i32>().fold(|_| 0, |_| 666), 0);

}