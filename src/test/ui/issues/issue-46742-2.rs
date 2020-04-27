unsafe fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    let foo = match "+" {
        "+" => add,
        "-" => |a,b| (a - b) as i32,
        _ => unimplemented!(),
    };
    let result: i32 = foo(5, 5); //~ ERROR call to unsafe function
}
