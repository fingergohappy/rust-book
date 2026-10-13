enum SpreadsheelCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![1, 2, 3];
    // mut  vec
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    let third = &v[2];
    println!("the third is {third}");
    let second = v.get(1);

    // handle Optional
    if let Some(value) = second {
        println!("the second is {value}");
    }
    let third = v.get(2);
    match third {
        Some(third) => println!("the third value is :{third}"),
        None => println!("there is no third eleemnt"),
    }
    // 可变 for each
    for i in &mut v {
        *i += 50;
    }

    // for each
    for i in &v {
        println!("{i}")
    }

    // enum 保存变量
    let row = vec![
        SpreadsheelCell::Int(3),
        SpreadsheelCell::Text(String::from("blue")),
        SpreadsheelCell::Float(10.12)
    ];
}

fn err_ref() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //这里将会报错
    // v.push(6);
    println!("the first value is {first}");
}
