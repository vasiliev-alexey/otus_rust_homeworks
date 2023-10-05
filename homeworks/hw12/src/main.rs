use bank_engine::bank::{Bank, BankTrait};

fn main() {
    let mut bank = Bank::new();

    bank.create_account("Alice").unwrap();
    bank.create_account("Bob").unwrap();

    bank.deposit("Alice", 100.0).unwrap();
    bank.withdraw("Alice", 50.0).unwrap();

    bank.transfer("Alice", "Bob", 25.0).unwrap();

    println!("Alice balance: {}", bank.get_balance("Alice").unwrap());
    println!("Bob balance: {}", bank.get_balance("Bob").unwrap());

    let history = bank.get_history();

    if history.is_err() {
        println!("History: {:?}", history.err().unwrap());
        return;
    }
    let history = history.unwrap();

    for operation in history {
        println!("{:?}", operation);
    }

    let alice_history = bank.get_account_history("Alice");

    if alice_history.is_err() {
        println!("Alice history: {:?}", alice_history.err().unwrap());
        return;
    }
    let alice_history = alice_history.unwrap();

    for operation in alice_history {
        println!("{:?}", operation);
    }
}
