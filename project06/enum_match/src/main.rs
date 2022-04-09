fn main() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[allow(dead_code)]
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let pcoin = Coin::Penny;
    let qcoin = Coin::Quarter(UsState::Alabama);
    value_in_cents(pcoin);
    value_in_cents(qcoin);
}
