// front_of_house::hosting 模块,完整路径是 crate::front_of_house::hosting

pub fn add_to_waitlist() {
    println!("[front_of_house::hosting] 加入等位队列");
    seat_at_table();
}

// 没有 pub:只有 hosting 模块内部能调用
fn seat_at_table() {
    println!("[front_of_house::hosting] 安排座位");
}
