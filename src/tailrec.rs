pub enum RecursionState<Done, Cont> {
    Continue(Cont),
    Done(Done)
}

pub trait TailRec<Output> {

    #[inline]
    fn rec<F>(self, iterate:F) -> Output
        where Self: Sized, F: Fn(Self) -> RecursionState<Output, Self> {

        let mut state = iterate(self);

        loop {
            match state {
                RecursionState::Done(output) => return output,
                RecursionState::Continue(more) => state = iterate(more)
            }
        }
    }

    #[inline]
    fn rec_ref<F>(&self, iterate:F) -> Output
        where Self: Sized, F: Fn(&Self) -> RecursionState<Output, Self> {

        let mut state = iterate(self);

        loop {
            match state {
                RecursionState::Done(output) => return output,
                RecursionState::Continue(more) => state = iterate(&more)
            }
        }
    }
}

impl<T,Output> TailRec<Output> for T {}

#[inline]
pub fn tail_rec<Input, Output, F>(input: Input, iterate:F) -> Output
    where Input: Sized, F: Fn(Input) -> RecursionState<Output, Input> {

    let mut state = iterate(input);

    loop {
        match state {
            RecursionState::Done(output) => return output,
            RecursionState::Continue(more) => state = iterate(more)
        }
    }
}

#[test]
fn test_tail_rec() {
    let r = 10.rec(|x| match x {
        0 => RecursionState::Done( () ),
        k => RecursionState::Continue( k -1 )
    });

    assert_eq!(r, ());
}

#[test]
fn test_tail_rec_ref() {
    let r = 10.rec_ref(|x| match x {
        &0 => RecursionState::Done( () ),
        &k => RecursionState::Continue( k -1 )
    });

    assert_eq!(r, ());
}

#[test]
fn test_tail_rec_as_func() {
    let r = tail_rec(10, |x| match x {
        0 => RecursionState::Done( () ),
        k => RecursionState::Continue( k -1 )
    });

    assert_eq!(r, ());
}