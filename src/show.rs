use std::fmt::{Display, Debug};

trait Show {
    fn show(&self) -> String;
}

trait ShowDebug {
    fn show_debug(&self) -> String;
}

impl<T: Display> Show for T {
    fn show(&self) -> String {
        format!("{}", self)
    }
}

impl<T: Debug> ShowDebug for T {
    fn show_debug(&self) -> String {
        format!("{:?}", self)
    }
}

#[test]
fn test_show_display() {
    assert_eq!(1.show(), "1".to_string());
    assert_eq!(1.show_debug(), "1".to_string());
}