#[derive(Debug, PartialEq)]
struct IOError;

struct IO<T, F: FnOnce() -> Result<T, IOError>>(F);

impl<T, F: FnOnce() -> Result<T,IOError> > IO<T,F> {
    pub fn run(self) -> Result<T,IOError> {
        (self.0)()
    }

}

#[test]
fn test_io(){
    let my_io = IO(|| Ok(100));
    assert_eq!(my_io.run(), Ok(100));
}