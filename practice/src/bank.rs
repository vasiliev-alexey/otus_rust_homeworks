#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Account {
    pub balance: i64,
    pub code: String,
}
#[allow(dead_code)]
fn print_balance(account: &Account) {
    println!(
        "Account code: {} Balance: {}",
        account.code, account.balance
    );
}
#[allow(dead_code)]
fn transfer_funds(ac1: &mut Account, ac2: &mut Account, amount: i64) {
    ac1.balance -= amount;
    ac2.balance += amount;
}
#[allow(dead_code)]
fn destroy_account(mut from_account: Account, to_account: &mut Account) {
    to_account.balance += from_account.balance;
    from_account.balance = 0;
    drop(from_account);
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance == 0 {
            println!("Account {} destroyed", self.code);
        } else {
            println!(
                "WARNING: Account {} not destroyed balance: {}",
                self.code, self.balance
            );
        }
    }
}

#[allow(dead_code)]
struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_rate: u32,
}
#[allow(dead_code)]
fn accrue_interest(bank: &mut Bank) {
    for account in bank.accounts.iter_mut() {
        account.balance += account.balance * bank.credit_rate as i64 / 1000;
    }
}
#[allow(dead_code)]
fn bank_balance(bank: &Bank) -> (i64, i64) {
    let liabilities = bank
        .accounts
        .iter()
        .filter(|account| account.balance < 0)
        .map(|account| account.balance)
        .sum();
    let assets = bank
        .accounts
        .iter()
        .filter(|account| account.balance > 0)
        .map(|account| account.balance)
        .sum();
    (liabilities, assets)
}
#[allow(dead_code)]
fn merge_banks(mut bank_from: Bank, bank_to: &mut Bank) {
    bank_to.accounts.append(&mut bank_from.accounts);
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let ac1 = Account {
            balance: 100,
            code: "ac1".to_string(),
        };
        let ac2 = Account {
            balance: 0,
            code: "ac2".to_string(),
        };
        let ac3 = Account {
            balance: -100,
            code: "ac3".to_string(),
        };
    }

    #[test]
    fn task2_print() {
        let ac1 = Account {
            balance: 100,
            code: "ac1".to_string(),
        };
        let ac2 = Account {
            balance: 0,
            code: "ac2".to_string(),
        };
        let ac3 = Account {
            balance: -100,
            code: "ac3".to_string(),
        };

        print_balance(&ac1);
        print_balance(&ac2);
        print_balance(&ac3);
    }

    #[test]
    fn task2_transfer_funds() {
        let mut ac1 = Account {
            balance: 100,
            code: "ac1".to_string(),
        };
        let mut ac2 = Account {
            balance: 0,
            code: "ac2".to_string(),
        };

        transfer_funds(&mut ac1, &mut ac2, 25);
        assert_eq!(ac1.balance, 75);
        assert_eq!(ac2.balance, 25);
    }

    #[test]
    fn task2_destroy_account() {
        let ac1 = Account {
            balance: 100,
            code: "ac1".to_string(),
        };
        let mut ac2 = Account {
            balance: 0,
            code: "ac2".to_string(),
        };

        destroy_account(ac1, &mut ac2);
        assert_eq!(ac2.balance, 100);
    }

    #[test]
    fn test_bank_acc() {
        let ac1 = Account {
            balance: 100,
            code: "ac1".to_string(),
        };
        let ac2 = Account {
            balance: 1000,
            code: "ac2".to_string(),
        };
        let mut bank = Bank {
            accounts: vec![ac1, ac2],
            credit_rate: 10,
            debit_rate: 0,
        };

        accrue_interest(&mut bank);
        for x in bank.accounts {
            if x.code == "ac1" {
                assert_eq!(x.balance, 101);
            } else if x.code == "ac2" {
                assert_eq!(x.balance, 1010);
            }
        }
    }

    #[test]
    fn test_bank_balance() {
        let ac1 = Account {
            balance: -1000,
            code: "ac1".to_string(),
        };
        let ac2 = Account {
            balance: 300,
            code: "ac2".to_string(),
        };
        let ac3 = Account {
            balance: 700,
            code: "ac3".to_string(),
        };

        let bank = Bank {
            accounts: vec![ac1, ac2, ac3],
            credit_rate: 10,
            debit_rate: 0,
        };

        let balance = bank_balance(&bank);
        assert_eq!(balance.0, -1000);
        assert_eq!(balance.1, 1000);
    }

    #[test]
    fn test_merge_banks() {
        let ac1 = Account {
            balance: 100,
            code: "ac1".to_string(),
        };
        let ac2 = Account {
            balance: 1000,
            code: "ac2".to_string(),
        };

        let ac3 = Account {
            balance: 200,
            code: "ac3".to_string(),
        };
        let ac4 = Account {
            balance: 2000,
            code: "ac4".to_string(),
        };

        let bank_from = Bank {
            accounts: vec![ac1, ac2],
            credit_rate: 10,
            debit_rate: 0,
        };
        let mut bank_to = Bank {
            accounts: vec![ac3, ac4],
            credit_rate: 10,
            debit_rate: 0,
        };

        merge_banks(bank_from, &mut bank_to);

        assert_eq!(bank_to.accounts.len(), 4);

        let a = bank_to.accounts.iter().find(|x| x.code == "ac1").unwrap();
        assert_eq!(a.balance, 100);
    }
}
