use hw12::bank::Bank;

fn main() {
    let mut bank = Bank::new();

    bank.create_account("Alice").unwrap();
    bank.create_account("Bob").unwrap();

    bank.deposit("Alice", 100.0).unwrap();
    bank.withdraw("Alice", 50.0).unwrap();

    bank.transfer("Alice", "Bob", 25.0).unwrap();

    println!("Alice balance: {}", bank.get_balance("Alice").unwrap()); // Output: Alice balance: 25
    println!("Bob balance: {}", bank.get_balance("Bob").unwrap()); // Output: Bob balance: 25

    let history = bank.get_history();
    for operation in history {
        println!("{:?}", operation);
    }

    let alice_history = bank.get_account_history("Alice");
    for operation in alice_history {
        println!("{:?}", operation);
    }
}
