// library crate 的根文件。
// 同一个 package 里的 binary crate(src/main.rs、src/bin/*)都能通过 rust_book:: 使用这里 pub 的东西。
// 注意:package 名 rust-book 里的 `-` 在代码中会变成 `_`。

// 写法一:mod 名字; —— 内容放在单独的文件 src/front_of_house.rs 里
mod front_of_house;

// 写法二:mod 名字 { ... } —— 内容直接写在大括号里
mod back_of_house {
    // struct 是 pub 的,但字段默认仍然私有,要逐个加 pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // seasonal_fruit 是私有字段,外面没法直接构造 Breakfast,只能通过这个函数
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn describe(&self) {
            println!("[back_of_house] 早餐: {} 吐司 + {}", self.toast, self.seasonal_fruit);
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super:: 表示上一级模块,这里就是 crate 根
    }

    fn cook_order() {
        println!("[back_of_house] 重新做菜");
    }
}

fn deliver_order() {
    println!("[crate 根] 上菜");
}

// 重导出:外部可以直接写 rust_book::hosting,不用知道它其实在 front_of_house 里面
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径:从 crate 根开始
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径:从当前模块(这里就是 crate 根)开始
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // toast 是 pub,可以改
    // meal.seasonal_fruit = String::from("blueberries"); // ❌ 私有字段,编译不过
    meal.describe();

    back_of_house::fix_incorrect_order();
}
