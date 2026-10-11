// 这是一个 binary crate,通过 rust_book:: 使用同一个 package 里的 library crate(src/lib.rs)

use rust_book::hosting; // 能这么写,是因为 lib.rs 里有 `pub use crate::front_of_house::hosting;`

fn main() {
    rust_book::eat_at_restaurant();

    println!("---");
    hosting::add_to_waitlist();

    // rust_book::front_of_house::hosting::add_to_waitlist(); // ❌ front_of_house 是私有模块
    // hosting::seat_at_table();                              // ❌ seat_at_table 是私有函数
}
