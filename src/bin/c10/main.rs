// ---------- 1. 两个参数共用 'a ----------
// 'a 把 x、y 和返回值连在一起:返回值的有效期 = x 和 y 中较短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ---------- 2. 返回值只和 x 连在一起 ----------
// y 用的是另一个生命周期 'b,和返回值无关,所以 y 先释放也不影响结果
fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("first 忽略了 y = {y}");
    x
    // 如果这里返回 y,会报错:签名承诺返回的是 'a,而 y 是 'b
}

// ---------- 3. 返回自己拥有的 String,不需要生命周期 ----------
// 结果是复制出来的新数据,不依赖 x、y 是否还活着;代价是多一次分配
fn longest_owned(x: &str, y: &str) -> String {
    if x.len() > y.len() { x.to_string() } else { y.to_string() }
}

// ---------- 4. 只有一个引用参数,编译器自动推断(省略规则) ----------
// 等价于 fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

// ---------- 5. struct 里存引用,也要标生命周期 ----------
// 含义:ImportantExcerpt 活着的时候,part 指向的字符串也必须活着
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // 1. 共用 'a:result 只能在 s2 还活着的范围内使用
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("1. longest: {result}");
    } // s2 在这里释放
    // println!("{result}"); // ❌ s2 已释放,而 result 可能指向 s2(只能取最短的那个)

    // 2. 只和 x 绑定:s2 释放后 result2 仍然能用
    let result2;
    {
        let s2 = String::from("xyz");
        result2 = first(s1.as_str(), s2.as_str());
    }
    println!("2. first: {result2}"); // ✅ result2 只依赖 s1

    // 3. 返回 String:和生命周期无关,随便用
    let result3;
    {
        let s2 = String::from("xyz");
        result3 = longest_owned(s1.as_str(), s2.as_str());
    }
    println!("3. longest_owned: {result3}");

    // 4. 省略规则
    println!("4. first_word: {}", first_word(&s1));

    // 5. struct 持有引用
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap_or("");
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("5. excerpt: {}", excerpt.part);
    // 如果 novel 先于 excerpt 被释放,编译器会报错
}
