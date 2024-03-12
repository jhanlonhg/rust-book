fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin_purse = 
        value_in_cents(Coin::Penny) 
        + value_in_cents(Coin::Nickel)
        + value_in_cents(Coin::Dime)
        + value_in_cents(Coin::Quarter);

    println!("Hey Mr. Moneybags, you have {} cents", coin_purse);

    let config_max = Some(3u8);
    
    if let Some(max) = config_max {
        println!("Config Max is {}", max);
    }

}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}