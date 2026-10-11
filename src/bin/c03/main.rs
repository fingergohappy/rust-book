fn main() {
    // error01();
    // comppundtype();
    play_array();
    let x = five();
    let x = plus_one(x);
}

fn error_01() {
    // let x = 5;
    let mut x = 5;
    println!("the value x is :{x}");
    x = 6;
    println!("the value x is {x}");
}

fn comppund_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value is x:{x},y:{y},z:{z}");
    println!("this tup first value is :{}", tup.0);
    println!("the tup is :{tup:?}");
}

fn play_array() {
    let a = [1, 2, 2, 3];
    let zeros = [0; 5];
    let first = a[0];
    let second = a[1];
    println!("the first is {first}");
    println!("the second is {second}");
    println!("the array a is {a:?}");
    println!("the zero array is {zeros:?}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
