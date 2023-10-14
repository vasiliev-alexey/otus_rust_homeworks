use bank_engine::bank::{Bank, BankError, BankTrait};

fn main() -> Result<(), BankError> {
    // Instantiate the bank
    let mut bank = Bank::new();

    // Example of creating an account
    bank.create_account("Alice")?;
    bank.create_account("Bob")?;

    //Example of deposit
    bank.deposit("Alice", 100.0)?;

    //Example of withdraw
    bank.withdraw("Alice", 50.0)?;

    //Example of transfer
    bank.transfer("Alice", "Bob", 25.0)?;

    //Example of balance
    println!("Alice balance: {}", bank.get_balance("Alice")?);
    println!("Bob balance: {}", bank.get_balance("Bob")?);

    //Example of history
    let history = bank.get_history()?;
    for operation in history {
        println!("{:?}", operation);
    }

    //Example of account history
    let alice_history = bank.get_account_history("Alice")?;
    for operation in alice_history {
        println!("{:?}", operation);
    }

    Ok(())
}
