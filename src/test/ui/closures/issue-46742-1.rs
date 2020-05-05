// check-pass
fn main() {
    let _ = (match "" {
        "+" => ::std::ops::Add::add,
        "-" => ::std::ops::Sub::sub,
        "<" => |a, b| (a < b) as i32,
        _ => unimplemented!(),
    })(5, 5);

    let _ = (match "" {
        "-" => ::std::ops::Sub::sub,
        "<" => |a, b| (a < b) as i32,
        "+" => ::std::ops::Add::add,
        _ => unimplemented!(),
    })(5, 5);

    let _ = (match "" {
        "<" => |a, b| (a < b) as i32,
        "+" => ::std::ops::Add::add,
        "-" => ::std::ops::Sub::sub,
        _ => unimplemented!(),
    })(5, 5);



    let _ = (match "" {
        "+" => ::std::ops::Add::add,
        "<" => |a, b| (a < b) as i32,
        _ => unimplemented!(),
    })(5, 5);

    let _ = (match "" {
        "<" => |a, b| (a < b) as i32,
        "+" => ::std::ops::Add::add,
        _ => unimplemented!(),
    })(5, 5);



    let _ = (match "" {
        "+" => |c,d| (c > d) as i32,
        "<" => |a, b| (a < b) as i32,
        "-" => ::std::ops::Sub::sub,
        _ => unimplemented!(),
    })(5, 5);

    let _ = (match "" {
        "<" => |a, b| (a < b) as i32,
        "-" => ::std::ops::Sub::sub,
        "+" => |c,d| (c > d) as i32,
        _ => unimplemented!(),
    })(5, 5);

    let _ = (match "" {
        "-" => ::std::ops::Sub::sub,
        "+" => |c,d| (c > d) as i32,
        "<" => |a, b| (a < b) as i32,
        _ => unimplemented!(),
    })(5, 5);
}
