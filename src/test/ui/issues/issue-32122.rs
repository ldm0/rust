// check-pass
use std::ops::Deref;

struct A (u8);

impl Deref for A {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let a = A(0);
    let d: *const u8 = &a;
}
