/// This module contains the implementation of a simple banking system.
///
/// The `Bank` struct represents a bank and provides methods for managing accounts
/// and performing various banking operations such as deposits, withdrawals, and transfers.
///
use nanoid::nanoid;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

type Money = f64;

const MONEY_ZERO: Money = 0.0;

#[derive(Default)]
pub struct Bank {
    accounts: HashMap<String, RefCell<Money>>,
    history: VecDeque<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    #[allow(unused)]
    id: String,
    source_account: String,
    target_account: Option<String>,
    amount: Money,
    operation_type: OperationType,
}

#[derive(Debug, Clone, PartialEq)]
enum OperationType {
    CreateAccount,
    Deposit,
    Withdraw,
    Transfer,
}

#[derive(Debug, Error, PartialEq)]
#[error("Account already exists")]
pub struct AccountDuplicationError {
    account: String,
}

#[derive(Debug, Error, PartialEq)]
#[error("Account does not exist")]
pub struct AccountNotFoundError {
    account: String,
}
#[derive(Debug, Error, PartialEq)]
#[error("Cannot transfer to the same account")]
pub struct SomeAccountTransferError {
    account: String,
}

#[derive(Debug, Error, PartialEq)]
#[error("Amount must be positive")]
pub struct AmountNegativeError {
    account: String,
    amount: Money,
}
#[derive(Debug, Error, PartialEq)]
#[error("Insufficient funds")]
pub struct InsufficientFundsError {
    account: String,
    amount: Money,
    balance: Money,
}

#[derive(Debug, Error, PartialEq)]
pub enum BankError {
    #[error("Account already exists")]
    AccountDuplication(#[from] AccountDuplicationError),
    #[error("Amount must be positive")]
    AmountNegative(#[from] AmountNegativeError),
    #[error("Account does not exist")]
    AccountNotFound(#[from] AccountNotFoundError),
    #[error("Insufficient funds")]
    InsufficientFunds(#[from] InsufficientFundsError),
    #[error("Cannot transfer to the same account")]
    SomeAccountTransfer(#[from] SomeAccountTransferError),
}

fn get_next_id() -> String {
    nanoid!(10)
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
            history: VecDeque::new(),
        }
    }
    /// Creates a new account with the specified name and adds it to the bank.
    ///
    /// # Arguments
    ///
    /// * `account` - The code of the account to create.
    ///
    /// # Errors
    /// AccountDuplicationError
    ///
    /// Returns an error if an account with the same name already exists in the bank.
    ///
    /// ```
    pub fn create_account(&mut self, account: &str) -> Result<(), BankError> {
        if self.accounts.contains_key(account) {
            Err(AccountDuplicationError {
                account: account.to_owned(),
            }
            .into())
        } else {
            self.accounts
                .insert(account.to_owned(), RefCell::from(MONEY_ZERO));
            let operation = Operation {
                id: get_next_id(),
                source_account: account.to_owned(),
                target_account: None,
                amount: MONEY_ZERO,
                operation_type: OperationType::CreateAccount,
            };
            self.history.push_back(operation);
            Ok(())
        }
    }
    /// Deposits the specified amount into the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to deposit into the account.
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    ///
    /// ```
    pub fn deposit(&mut self, account: &str, amount: Money) -> Result<(), BankError> {
        if let Some(balance) = self.accounts.get_mut(account) {
            if amount <= Money::default() {
                Err(AmountNegativeError {
                    account: account.to_owned(),
                    amount,
                }
                .into())
            } else {
                *balance.get_mut() += amount;
                let operation = Operation {
                    id: get_next_id(),
                    source_account: account.to_owned(),
                    target_account: None,
                    amount,
                    operation_type: OperationType::Deposit,
                };
                self.history.push_back(operation);
                Ok(())
            }
        } else {
            Err(AccountNotFoundError {
                account: account.to_owned(),
            }
            .into())
        }
    }
    /// Withdraws the specified amount from the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to withdraw from the account.
    ///
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    /// InsufficientFundsError
    ///
    /// Returns an error if the account balance is insufficient to cover the withdrawal amount.
    ///
    /// ```

    pub fn withdraw(&mut self, account: &str, amount: Money) -> Result<(), BankError> {
        if let Some(balance) = self.accounts.get_mut(account) {
            if amount <= Money::default() {
                Err(AmountNegativeError {
                    account: account.to_owned(),
                    amount,
                }
                .into())
            } else if *balance < RefCell::from(amount) {
                Err(InsufficientFundsError {
                    amount,
                    account: account.to_owned(),
                    balance: *balance.borrow(),
                }
                .into())
            } else {
                *balance.get_mut() -= amount;
                let operation = Operation {
                    id: get_next_id(),
                    source_account: account.to_owned(),
                    target_account: None,
                    amount,
                    operation_type: OperationType::Withdraw,
                };
                self.history.push_back(operation);
                Ok(())
            }
        } else {
            Err(AccountNotFoundError {
                account: account.to_owned(),
            }
            .into())
        }
    }
    /// Transfers the specified amount from one account to another.
    ///
    /// # Arguments
    ///
    /// * `sender` - The name of the account from which the amount will be transferred.
    /// * `receiver` - The name of the account to which the amount will be transferred.
    /// * `amount` - The amount to transfer.
    ///
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    /// InsufficientFundsError
    /// SomeAccountTransferError
    ///
    /// Returns an error if either the sender or receiver account does not exist, or if
    /// the sender account does not have sufficient balance to cover the transfer amount.
    ///
    /// ```
    pub fn transfer(
        &mut self,
        sender_account: &str,
        receiver_account: &str,
        amount: Money,
    ) -> Result<(), BankError> {
        if sender_account == receiver_account {
            return Err(SomeAccountTransferError {
                account: sender_account.to_owned(),
            }
            .into());
        }

        if let Some(sender_balance) = self.accounts.get(sender_account) {
            if let Some(receiver_balance) = self.accounts.get(receiver_account) {
                if amount <= MONEY_ZERO {
                    Err(AmountNegativeError {
                        amount,
                        account: sender_account.to_owned(),
                    }
                    .into())
                } else if *sender_balance < RefCell::from(amount) {
                    Err(InsufficientFundsError {
                        amount,
                        account: sender_account.to_owned(),
                        balance: *sender_balance.borrow(),
                    }
                    .into())
                } else {
                    *sender_balance.borrow_mut() -= amount;
                    *receiver_balance.borrow_mut() += amount;
                    let operation = Operation {
                        id: get_next_id(),
                        source_account: sender_account.to_owned(),
                        amount,
                        target_account: Some(receiver_account.to_owned()),
                        operation_type: OperationType::Transfer,
                    };

                    self.history.push_back(operation);

                    Ok(())
                }
            } else {
                Err(AccountNotFoundError {
                    account: receiver_account.to_owned(),
                }
                .into())
            }
        } else {
            Err(AccountNotFoundError {
                account: sender_account.to_owned(),
            }
            .into())
        }
    }
    /// Returns the current balance of the account.
    /// ```
    pub fn get_balance(&self, account: &str) -> Option<Money> {
        self.accounts.get(account).map(|balance| *balance.borrow())
    }
    /// Returns the transaction history of the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account for which to retrieve the transaction history.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the transaction history of the account.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified account does not exist.
    /// ```
    pub fn get_history(&self) -> Vec<Operation> {
        self.history.iter().cloned().collect()
    }
    /// Returns the transaction history of the specified account.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account for which to retrieve the transaction history.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the transaction history of the account.
    ///
    /// # Errors
    /// ```

    pub fn get_account_history(&self, account: &str) -> Vec<Operation> {
        self.history
            .iter()
            .cloned()
            .filter(|operation| {
                operation.source_account == account
                    || operation.target_account == Some(account.to_string())
            })
            .collect()
    }
    /// Replays the transaction history stored in a source_bank for the new Bank instance.
    ///
    /// # Arguments
    ///
    /// * `source_bank` - The Bank for which to replay the transaction history.
    ///
    /// # Returns
    /// new instance of Bank
    /// # Errors
    /// Returns an error if the specified account does not exist, if the file does not exist,
    /// or if there was an error while replaying the transaction history.
    ///
    /// ```
    pub fn replay_history(source_bank: Bank) -> Bank {
        let mut target_bank = Bank::new();
        for operation in source_bank.get_history() {
            match operation.operation_type {
                OperationType::CreateAccount => target_bank
                    .create_account(operation.source_account.as_str())
                    .unwrap(),
                OperationType::Deposit => target_bank
                    .deposit(operation.source_account.as_str(), operation.amount)
                    .unwrap(),
                OperationType::Withdraw => target_bank
                    .withdraw(operation.source_account.as_str(), operation.amount)
                    .unwrap(),
                OperationType::Transfer => target_bank
                    .transfer(
                        operation.source_account.as_str(),
                        operation.target_account.unwrap().as_str(),
                        operation.amount,
                    )
                    .unwrap(),
            }
        }
        target_bank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let mut bank = Bank::new();
        assert!(bank.create_account("Alice").is_ok());
        assert!(bank.create_account("Bob").is_ok());
        let duplicate_account = bank.create_account("Alice");
        assert!(duplicate_account.is_err());
        assert_eq!(
            duplicate_account.unwrap_err(),
            BankError::AccountDuplication(AccountDuplicationError {
                account: "Alice".to_string()
            })
        );
    }

    #[test]
    fn test_deposit() {
        let mut bank = Bank::new();
        bank.create_account("Alice").unwrap();
        assert!(bank.deposit("Alice", 100.0).is_ok());
        assert_eq!(bank.get_balance("Alice"), Some(100.0));
        let res = bank.deposit("Alice", -50.0);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err(),
            BankError::AmountNegative(AmountNegativeError {
                account: "Alice".to_string(),
                amount: -50.0
            })
        );
    }

    #[test]
    fn test_withdraw() {
        let mut bank = Bank::new();
        bank.create_account("Alice").unwrap();
        bank.deposit("Alice", 100.0).unwrap();

        assert!(bank.withdraw("Alice", 50.0).is_ok());
        assert_eq!(bank.get_balance("Alice"), Some(50.0));
        let res = bank.withdraw("Alice", -30.0);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err(),
            BankError::AmountNegative(AmountNegativeError {
                account: "Alice".to_string(),
                amount: -30.0
            })
        );
        let res = bank.withdraw("Alice", 100.0);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err(),
            BankError::InsufficientFunds(InsufficientFundsError {
                account: "Alice".to_string(),
                balance: 50.0,
                amount: 100.0
            })
        );
    }

    #[test]
    fn test_transfer() {
        let mut bank = Bank::new();
        bank.create_account("Alice").unwrap();
        bank.create_account("Bob").unwrap();
        bank.deposit("Alice", 100.0).unwrap();

        assert!(bank.transfer("Alice", "Bob", 50.0).is_ok());
        assert_eq!(bank.get_balance("Alice"), Some(50.0));
        assert_eq!(bank.get_balance("Bob"), Some(50.0));

        let res = bank.transfer("Alice", "Bob", -30.0);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err(),
            AmountNegativeError {
                amount: -30.0,
                account: "Alice".to_string(),
            }
            .into()
        );
        let res = bank.transfer("Alice", "Bob", 100.0);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err(),
            InsufficientFundsError {
                amount: 100.0,
                account: "Alice".to_string(),
                balance: 50.0
            }
            .into()
        );
        let res = bank.transfer("Alice", "Alice", 20.0);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err(),
            SomeAccountTransferError {
                account: "Alice".to_string()
            }
            .into()
        );
        let res = bank.transfer("Eve", "Bob", 10.0);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            AccountNotFoundError {
                account: "Eve".to_string()
            }
            .into()
        );
        let res = bank.transfer("Alice", "Eve", 10.0);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            AccountNotFoundError {
                account: "Eve".to_string()
            }
            .into()
        );
    }

    #[test]
    fn test_get_balance() {
        let mut bank = Bank::new();
        bank.create_account("Alice").unwrap();
        bank.deposit("Alice", 100.0).unwrap();

        assert_eq!(bank.get_balance("Alice"), Some(100.0));

        assert_eq!(bank.get_balance("Bob"), None);
    }

    #[test]
    fn test_get_history() {
        let mut bank = Bank::new();
        bank.create_account("Alice").unwrap();
        bank.create_account("Bob").unwrap();
        bank.deposit("Alice", 100.0).unwrap();
        bank.transfer("Alice", "Bob", 50.0).unwrap();

        let history = bank.get_history();
        assert_eq!(history.len(), 4);
        assert_eq!(history[0].operation_type, OperationType::CreateAccount);
        assert_eq!(history[1].operation_type, OperationType::CreateAccount);
        assert_eq!(history[2].operation_type, OperationType::Deposit);
        assert_eq!(history[3].operation_type, OperationType::Transfer);
    }

    #[test]
    fn test_get_account_history() {
        let mut bank = Bank::new();
        bank.create_account("Alice").unwrap();
        bank.create_account("Bob").unwrap();
        bank.deposit("Alice", 100.0).unwrap();
        bank.transfer("Alice", "Bob", 50.0).unwrap();

        let alice_history = bank.get_account_history("Alice");
        assert_eq!(alice_history.len(), 3);
        assert_eq!(
            alice_history[0].operation_type,
            OperationType::CreateAccount
        );
        assert_eq!(alice_history[1].operation_type, OperationType::Deposit);
        assert_eq!(alice_history[2].operation_type, OperationType::Transfer);

        let bob_history = bank.get_account_history("Bob");
        assert_eq!(bob_history.len(), 2);
        assert_eq!(bob_history[0].operation_type, OperationType::CreateAccount);
    }
    #[test]
    fn test_replay_history() {
        let mut source_bank = Bank::new();
        source_bank.create_account("Alice").unwrap();
        source_bank.create_account("Bob").unwrap();
        source_bank.deposit("Alice", 100.0).unwrap();
        source_bank.transfer("Alice", "Bob", 50.0).unwrap();

        let target_bank = Bank::replay_history(source_bank);

        assert_eq!(target_bank.get_balance("Alice"), Some(50.0));
        assert_eq!(target_bank.get_balance("Bob"), Some(50.0));

        let alice_history = target_bank.get_account_history("Alice");
        assert_eq!(alice_history.len(), 3);
        assert_eq!(
            alice_history[0].operation_type,
            OperationType::CreateAccount
        );
        assert_eq!(alice_history[1].operation_type, OperationType::Deposit);
        assert_eq!(alice_history[2].operation_type, OperationType::Transfer);

        let bob_history = target_bank.get_account_history("Bob");
        assert_eq!(bob_history.len(), 2);
        assert_eq!(bob_history[0].operation_type, OperationType::CreateAccount);
        assert_eq!(bob_history[1].operation_type, OperationType::Transfer);
    }
}
