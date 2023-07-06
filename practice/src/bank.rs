pub struct Account {
    balance: i64,
    code: String,
}

impl Account {
    fn new(balance: i64, code: &str) -> Self {
        Self {
            balance,
            code: code.to_string(),
        }
    }
    pub fn print_balance(self: &Account) {
        println!("Account code: {} Balance: {}", self.code, self.balance);
    }

    fn transfer_funds(self: &mut Account, from_account: &mut Account, amount: i64) {
        self.balance += amount;
        from_account.balance -= amount;
    }

    fn destroy_account(mut self: Account, to_account: &mut Account) {
        let amount = self.balance;
        to_account.transfer_funds(&mut self, amount);
    }
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
pub struct Bank {
    accounts: Vec<Account>,
    credit_rate: u32,
    debit_rate: u32,
}

impl Bank {
    fn new(credit_rate: u32, debit_rate: u32, accounts: Vec<Account>) -> Self {
        Self {
            accounts,
            credit_rate,
            debit_rate,
        }
    }

    fn merge_banks(self: &mut Bank, mut bank_from: Bank) {
        self.accounts.append(&mut bank_from.accounts);
    }

    fn bank_balance(self: &Bank) -> (i64, i64) {
        let liabilities = self
            .accounts
            .iter()
            .map(|account| account.balance)
            .filter(|b| b.is_negative())
            .sum();
        let assets = self
            .accounts
            .iter()
            .map(|account| account.balance)
            .filter(|b| b.is_positive())
            .sum();
        (liabilities, assets)
    }
    fn accrue_interest(self: &mut Bank) {
        for account in self.accounts.iter_mut() {
            if account.balance < 0 {
                account.balance += account.balance * (self.credit_rate as i64) / 1000;
            } else {
                account.balance += account.balance * (self.debit_rate as i64) / 1000;
            }
        }
    }
}

#[allow(unused_variables, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        let ac1 = Account::new(100, "ac1");
        let ac2 = Account::new(0, "ac2");
        let ac3 = Account::new(-100, "ac3");
        ac1.print_balance();
    }

    #[test]
    fn task2_print() {
        let ac1 = Account::new(100, "ac1");
        let ac2 = Account::new(0, "ac2");
        let ac3 = Account::new(-100, "ac3");
        ac1.print_balance();
        ac2.print_balance();
        ac3.print_balance();
    }

    #[test]
    fn task2_transfer_funds() {
        let mut ac1 = Account::new(100, "ac1");
        let mut ac2 = Account::new(0, "ac2");
        ac2.transfer_funds(&mut ac1, 25);
        assert_eq!(ac1.balance, 75);
        assert_eq!(ac2.balance, 25);
    }

    #[test]
    fn task2_destroy_account() {
        let ac1 = Account::new(100, "ac1");
        let mut ac2 = Account::new(0, "ac2");
        ac1.destroy_account(&mut ac2);
        assert_eq!(ac2.balance, 100);
    }

    #[test]
    fn test_bank_acc() {
        let ac1 = Account::new(100, "ac1");
        let ac2 = Account::new(1000, "ac2");
        let ac2 = Account::new(-1100, "ac3");
        let mut bank = Bank::new(10, 10, vec![ac1, ac2]);

        bank.accrue_interest();
        for x in bank.accounts {
            if x.code == "ac1" {
                assert_eq!(x.balance, 101);
            } else if x.code == "ac2" {
                assert_eq!(x.balance, 1010);
            } else {
                assert_eq!(x.balance, -1111);
            }
        }
    }

    #[test]
    fn test_bank_balance() {
        let ac1 = Account::new(-1000, "ac1");
        let ac2 = Account::new(300, "ac2");
        let ac3 = Account::new(700, "ac3");
        let bank = Bank::new(10, 0, vec![ac1, ac2, ac3]);
        let balance = bank.bank_balance();
        assert_eq!(balance.0, -1000);
        assert_eq!(balance.1, 1000);
    }

    #[test]
    fn test_merge_banks() {
        let ac1 = Account::new(100, "ac1");
        let ac2 = Account::new(1000, "ac2");
        let ac3 = Account::new(200, "ac3");
        let ac4 = Account::new(2000, "ac4");
        let bank_from = Bank::new(10, 0, vec![ac1, ac2]);
        let mut bank_to = Bank::new(10, 0, vec![ac3, ac4]);
        bank_to.merge_banks(bank_from);
        assert_eq!(bank_to.accounts.len(), 4);
        let a = bank_to.accounts.iter().find(|x| x.code == "ac1").unwrap();
        assert_eq!(a.balance, 100);
    }
}
