use std::fmt::{Display, Formatter, Result as FmtResult, Debug};

pub enum Validation<T, E> {
    VOk(T),
    VErr(Vec<E>),
}

impl<T: Display, E: Display + Debug> Display for Validation<T,E> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Validation::VOk(ref x) => write!(f, "OK: {}", x),
            &Validation::VErr(ref errs) => write!(f, "Errors: {:?}", errs),

        }
    }
}

impl<T,E> Validation<T,E> {
    pub fn from_result(r: Result<T,E>) -> Validation<T,E> { match r {
        Ok(x) => Validation::VOk(x),
        Err(e) => Validation::VErr(vec![e])
    }}

    pub fn is_ok(&self) -> bool { match self {
        &Validation::VOk(_) => true,
        _ => false
    }}

    pub fn is_error(&self) -> bool { !self.is_ok() }

    pub fn map<U, F>(self, op: F) -> Validation<U, E>
        where F: FnOnce(T) -> U { match self {
        Validation::VOk(ok) => Validation::VOk(op(ok)),
        Validation::VErr(err) => Validation::VErr(err)
    }
    }

    pub fn map_err<F, O>(self, op: O) -> Validation<T, F>
        where O: Fn(&E) -> F { match self {
        Validation::VOk(ok) => Validation::VOk(ok),
        Validation::VErr(err) => Validation::VErr(
            err.iter().map(|e| op(e)).collect::<Vec<F>>()
        )
    }
    }

    pub fn and_then<U, F>(self, op: F) -> Validation<U, E>
        where F: FnOnce(T) -> Validation<U, E> { match self {
        Validation::VOk(ok) => op(ok),
        Validation::VErr(err) => Validation::VErr(err)
    }
    }

    pub fn append<U>(self, r2: Validation<U,E>)-> Validation<(T,U),E> { match (self,r2) {
        (Validation::VOk(o1), Validation::VOk(o2)) => Validation::VOk((o1, o2)),
        (Validation::VOk(_), Validation::VErr(e2)) => Validation::VErr(e2),
        (Validation::VErr(e1), Validation::VOk(_)) => Validation::VErr(e1),
        (Validation::VErr(e1), Validation::VErr(e2)) => {
            let mut new_errors: Vec<E> = Vec::new();
            new_errors.extend(e1);
            new_errors.extend(e2);
            Validation::VErr(new_errors)
        },
    }}

}

