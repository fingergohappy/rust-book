enum IpAddrKind {
    V4(u8, u8),
    V6(String),
}

enum UsState{
    Alabama,
    Alaska
}
enum Coin {
    Peny,
    Nicklel,
    Dime,
    Quarter(UsState),
}


fn vale_in_certs(coin: Coin) -> u8 {
    match coin {
        Coin::Peny => {
            println!("Lucky Peny!");
            1
        },
        Coin::Nicklel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state}");
            15
        },
    }
}

fn main() {
    let local = IpAddrKind::V6(String::from("127.0.0.1"));
    let dice_doll = 9;
    match dice_doll{
        3=>add_fancy_hat(),
        7 => remove_fancy_hat(),
        other=> move_player(other),
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_spaces: u8){}
