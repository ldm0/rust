// check-pass
fn foo(x: i32) -> i32 {
    x
}
fn main() {
    // We still can coerce non-capturing closure to function pointer
    let _ = if true {
        foo as fn(i32) -> i32
    } else {
        |x: i32| x
    };

    // We still can coerce function pointer to non-capturing closure
    let _ = if true {
        |x: i32| x
    } else {
        foo as fn(i32) -> i32
    };
}
