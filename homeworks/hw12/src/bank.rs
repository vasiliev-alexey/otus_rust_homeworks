//! This module contains the implementation of a simple banking system.
//!
//! The [`Bank`] struct represents a bank and provides methods for managing accounts
//! and performing various banking operations such as deposits, withdrawals, and transfers.
///
use log::{debug, error, info};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

type Money = f64;
pub type Result<T, E = BankError> = std::result::Result<T, E>;

const MONEY_ZERO: Money = 0.0;

pub type TransactionId = String;

pub enum BankResponse {
    Transaction(Result<TransactionId>),
    History(Result<Vec<Operation>>),
    Balance(Result<Money>),
}

#[derive(Default)]
pub struct Bank {
    accounts: HashMap<String, RefCell<Money>>,
    accounts_history: HashMap<String, Vec<TransactionId>>,
    history: BTreeMap<TransactionId, Operation>,
    ulid_generator: ulid::Generator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Operation {
    #[allow(unused)]
    id: String,
    source_account: String,
    amount: Money,
    operation_type: OperationType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationType {
    CreateAccount,
    Deposit,
    Withdraw,
    Transfer { target_account: String },
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

#[macro_export]
macro_rules! check_account_exists {
    ($self: expr , $account: expr) => {{
        if !$self.accounts.contains_key(&$account) {
            error!("Account {} does not exist", $account);
            return Err(AccountNotFoundError { account: $account }.into());
        }
    }};
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
#[error("Insufficient funds for account `{0}` available `{1}` requested `{2}`", .account, .amount, .balance)]
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

impl BankError {
    pub fn account_not_found(account: String) -> Self {
        error!("Account {} does not exist", account);
        AccountNotFoundError { account }.into()
    }
}

impl Bank {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_next_id(&mut self) -> String {
        self.ulid_generator
            .generate_with_source(&mut StdRng::from_entropy())
            .unwrap()
            .to_string()
    }

    fn push_transaction(&mut self, operation: Operation) -> Result<(), BankError> {
        if !self.accounts.contains_key(&operation.source_account) {
            return Err(BankError::account_not_found(operation.source_account));
        }
        // check target account exists when transferring
        if let OperationType::Transfer { ref target_account } = operation.operation_type {
            if !self.accounts.contains_key(target_account) {
                return Err(BankError::account_not_found(target_account.to_string()));
            }
        }

        let account = operation.source_account.clone();
        self.accounts_history
            .entry(account)
            .or_default()
            .push(operation.id.clone());

        if let OperationType::Transfer { ref target_account } = operation.operation_type {
            let target_account = target_account.clone();
            self.accounts_history
                .get_mut(&target_account)
                .unwrap()
                .push(operation.id.clone());
        }
        self.history.insert(operation.id.clone(), operation);
        Ok(())
    }
}

impl BankTrait for Bank {
    /// Creates a new account with the specified name and adds it to the bank.
    ///
    /// # Arguments
    ///
    /// * `account` - The code of the account to create.
    ///
    /// # Errors
    /// AccountDuplicationError
    ///
    /// Result
    /// TransactionId for the new account
    /// Returns an error if an account with the same name already exists in the bank.
    ///
    /// ```
    fn create_account(&mut self, account: &str) -> Result<TransactionId> {
        if self.accounts.contains_key(account) {
            error!("Account already exists");
            return Err(AccountDuplicationError {
                account: account.to_owned(),
            }
            .into());
        }

        let next_id = self.get_next_id();
        self.accounts
            .insert(account.to_owned(), RefCell::from(MONEY_ZERO));
        let operation = Operation {
            id: next_id.clone(),
            source_account: account.to_owned(),
            amount: MONEY_ZERO,
            operation_type: OperationType::CreateAccount,
        };
        self.push_transaction(operation)?;
        info!("Created account {}", &account);
        Ok(next_id)
    }

    /// Deposits the specified amount into the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to deposit into the account.
    /// * `account` - The account to deposit into.
    ///
    /// Result
    /// `TransactionId` for operation
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    ///
    /// ```
    fn deposit(&mut self, account: &str, amount: Money) -> Result<TransactionId, BankError> {
        check_account_exists!(self, account.to_string());

        if let Some(balance) = self.accounts.get_mut(account) {
            if amount <= Money::default() {
                error!("Amount must be positive");
                Err(AmountNegativeError {
                    account: account.to_owned(),
                    amount,
                }
                .into())
            } else {
                *balance.get_mut() += amount;
                let transaction_id = self.get_next_id();
                let operation = Operation {
                    id: transaction_id.to_owned(),
                    source_account: account.to_owned(),
                    amount,
                    operation_type: OperationType::Deposit,
                };
                self.push_transaction(operation)?;
                info!("Deposited into account {}", &account);
                Ok(transaction_id.to_owned())
            }
        } else {
            Err(BankError::account_not_found(account.to_string()))
        }
    }

    /// Withdraws the specified amount from the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to withdraw from the account.
    /// * `account` - The account to withdraw from.
    ///
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    /// InsufficientFundsError
    ///
    /// Returns an error if the account balance is insufficient to cover the withdrawal amount.
    ///
    /// ```
    fn withdraw(&mut self, account: &str, amount: Money) -> Result<TransactionId, BankError> {
        check_account_exists!(self, account.to_string());

        let transaction_id = self.get_next_id();
        let operation = Operation {
            id: transaction_id.to_owned(),
            source_account: account.to_owned(),
            amount,
            operation_type: OperationType::Withdraw,
        };

        if let Some(balance) = self.accounts.get_mut(account) {
            if amount <= Money::default() {
                error!("Amount must be positive: amount {amount}");
                return Err(AmountNegativeError {
                    account: account.to_owned(),
                    amount,
                }
                .into());
            } else if *balance < RefCell::from(amount) {
                let balance = balance.borrow();
                error!(
                    "Insufficient funds for the operation. Balance: {balance:?} Amount: {amount}"
                );
                return Err(InsufficientFundsError {
                    amount,
                    account: account.to_owned(),
                    balance: balance.to_owned(),
                }
                .into());
            } else {
                let mut balance = balance.borrow_mut();
                debug!("Balance before: {balance:?}");
                *balance -= amount;
            }
        } else {
            return Err(BankError::account_not_found(account.to_string()));
        }
        info!("Withdrawn from account {} amount {}", &account, amount);
        self.push_transaction(operation)?;
        Ok(transaction_id)
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
    fn transfer(
        &mut self,
        sender_account: &str,
        receiver_account: &str,
        amount: Money,
    ) -> Result<TransactionId, BankError> {
        debug!(
            "transfer {} from {} to {}",
            amount, sender_account, receiver_account
        );

        check_account_exists!(self, sender_account.to_string());
        check_account_exists!(self, receiver_account.to_string());

        if sender_account == receiver_account {
            error!("Cannot transfer to the same account");
            return Err(SomeAccountTransferError {
                account: sender_account.to_owned(),
            }
            .into());
        }

        if let Some(sender_balance) = self.accounts.get(sender_account) {
            if let Some(receiver_balance) = self.accounts.get(receiver_account) {
                if amount <= MONEY_ZERO {
                    error!("Amount must be positive");
                    Err(AmountNegativeError {
                        amount,
                        account: sender_account.to_owned(),
                    }
                    .into())
                } else if *sender_balance < RefCell::from(amount) {
                    let sender_balance = sender_balance.borrow();
                    error!(
                        "Insufficient funds for the operation. Balance: {sender_balance:?} Amount: {amount}"
                    );
                    Err(InsufficientFundsError {
                        amount,
                        account: sender_account.to_owned(),
                        balance: sender_balance.to_owned(),
                    }
                    .into())
                } else {
                    *sender_balance.borrow_mut() -= amount;
                    *receiver_balance.borrow_mut() += amount;
                    let transaction_id = self.get_next_id();
                    let operation = Operation {
                        id: transaction_id.to_owned(),
                        source_account: sender_account.to_owned(),
                        amount,
                        operation_type: OperationType::Transfer {
                            target_account: receiver_account.to_owned(),
                        },
                    };
                    self.push_transaction(operation)?;
                    info!(
                        "Transaction id: {} Transferred {} from {} to {}",
                        transaction_id, amount, sender_account, receiver_account
                    );
                    Ok(transaction_id.to_owned())
                }
            } else {
                Err(BankError::account_not_found(receiver_account.to_string()))
            }
        } else {
            Err(BankError::account_not_found(sender_account.to_string()))
        }
    }

    /// Returns the current balance of the account.
    /// # Arguments
    ///
    /// * `account` - The code of the account for which to retrieve the balance.
    ///
    /// # Returns
    /// The current balance of the account.
    /// # Errors
    /// AccountNotFoundError
    /// ```
    fn get_balance(&self, account: &str) -> Result<Money, BankError> {
        debug!("get_balance {}", account);
        check_account_exists!(self, account.to_string());
        Ok(self
            .accounts
            .get(account)
            .map(|balance| *balance.borrow())
            .unwrap())
    }

    /// Returns the transaction history of the Bank.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A vector of [Operation] representing the transaction history of the account.
    ///
    /// # Errors
    /// BankError
    /// Returns an error if the specified account does not exist.
    /// ```
    fn get_history(&self) -> Result<Vec<Operation>, BankError> {
        let hist = self.history.iter().map(|k| k.1.clone()).collect::<Vec<_>>();
        Ok(hist)
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
    /// BankError
    /// ```
    fn get_account_history(&self, account: &str) -> Result<Vec<&Operation>, BankError> {
        check_account_exists!(self, account.to_string());
        let transaction_history = self.accounts_history.get(account);
        let transaction_history = transaction_history.unwrap();
        Ok(transaction_history
            .iter()
            .map(|t| self.history.get(t).unwrap())
            .collect())
    }

    /// Replays the transaction history stored in a source_bank for the new Bank instance.
    ///
    /// # Arguments
    ///
    /// * `operations_log` - history of operations to replay
    ///
    /// # Returns
    /// new instance of Bank
    /// # Errors
    /// Returns an error if the specified account does not exist, if the file does not exist,
    /// or if there was an error while replaying the transaction history.
    ///
    /// ```
    fn replay_history<'a>(operations_log: impl Iterator<Item = &'a Operation>) -> Bank {
        let mut target_bank = Bank::new();

        for operation in operations_log {
            match &operation.operation_type {
                OperationType::CreateAccount => target_bank
                    .create_account(&operation.source_account)
                    .unwrap(),
                OperationType::Deposit => target_bank
                    .deposit(&operation.source_account, operation.amount)
                    .unwrap(),
                OperationType::Withdraw => target_bank
                    .withdraw(&operation.source_account, operation.amount)
                    .unwrap(),
                OperationType::Transfer { target_account } => target_bank
                    .transfer(&operation.source_account, target_account, operation.amount)
                    .unwrap(),
            };
            //}
        }
        target_bank
    }

    /// Returns an `Option<&Operation>` representing the operation with the given ID if it exists in the history,
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the operation to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<&Operation>` representing the operation with the given ID if it exists in the history,
    /// or `None` if no operation with the given ID is found.
    ///
    fn get_operation_by_id(&self, id: &TransactionId) -> Option<&Operation> {
        self.history.get(id)
    }
}

pub trait BankTrait {
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
    fn create_account(&mut self, account: &str) -> Result<TransactionId>;

    /// Deposits the specified amount into the account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to deposit into.
    /// * `amount` - The amount to deposit into the account.
    ///
    /// #Returns
    ///  TransactionId for the new account
    ///  BankError if the account does not exist or the amount is negative
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    ///
    /// ```
    fn deposit(&mut self, account: &str, amount: Money) -> Result<TransactionId>;

    /// Withdraws the specified amount from the account.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to withdraw from.
    /// * `amount` - The amount to withdraw from the account.
    ///
    /// # Returns
    /// [TransactionId] for processing the transaction
    /// BankError if the account does not exist or the amount is negative or insufficient funds
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    /// InsufficientFundsError
    ///
    /// ```
    fn withdraw(&mut self, account: &str, amount: Money) -> Result<TransactionId>;

    /// Transfers the specified amount from one account to another.
    ///
    /// # Arguments
    ///
    /// * `sender` - The name of the account from which the amount will be transferred.
    /// * `receiver` - The name of the account to which the amount will be transferred.
    /// * `amount` - The amount to transfer.
    ///
    /// # Returns
    /// [TransactionId] for processing the transaction
    /// BankError if the account does not exist or the amount is negative or insufficient funds
    ///
    /// # Errors
    /// AmountNegativeError
    /// AccountNotFoundError
    /// InsufficientFundsError
    /// SomeAccountTransferError
    ///
    /// ```
    fn transfer(
        &mut self,
        sender_account: &str,
        receiver_account: &str,
        amount: Money,
    ) -> Result<TransactionId>;

    /// Returns the current balance of the account.
    /// # Arguments
    ///
    ///  * 'account' - The code of the account for which to retrieve the balance.
    ///
    /// # Returns
    /// The current balance of the account.
    /// # Errors
    /// AccountNotFoundError
    /// ```
    fn get_balance(&self, account: &str) -> Result<Money, BankError>;

    /// Returns the transaction history for the bank.
    ///
    /// # Arguments
    /// self
    /// # Returns
    ///
    /// A vector of strings representing the transaction history of the account.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified account does not exist.
    /// ```
    fn get_history(&self) -> Result<Vec<Operation>>;

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
    /// BankError
    /// ```
    fn get_account_history(&self, account: &str) -> Result<Vec<&Operation>>;

    /// Replays the transaction history stored in a source_bank for the new Bank instance.
    ///
    /// # Arguments
    ///
    /// * `source_bank` - The Bank for which to replay the transaction history.
    ///
    /// # Returns
    /// new instance of Bank
    /// # Errors
    /// BankError
    /// Returns an error if the specified account does not exist, if the file does not exist,
    /// or if there was an error while replaying the transaction history.
    ///
    /// ```
    fn replay_history<'a>(operations_log: impl Iterator<Item = &'a Operation>) -> Bank;

    // Retrieves an operation from the history by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the operation to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<&Operation>` representing the operation with the given ID if it exists in the history,
    /// or `None` if no operation with the given ID is found.
    ///
    fn get_operation_by_id(&self, id: &TransactionId) -> Option<&Operation>;
}

#[test_env_helpers::before_all]
#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! bank_with_accounts {
    ( $( $x:expr ),* ) => {{
        let mut bank = Bank::new();
        $(
           let _ =  bank.create_account($x);
        )*
        bank
    }};
}

    fn before_all() {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    }

    #[test]
    fn test_create_account() {
        let mut bank = Bank::new();
        match bank.create_account("Alice") {
            Ok(res) => assert!(!res.is_empty()),
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
        match bank.create_account("Bob") {
            Ok(res) => assert!(!res.is_empty()),
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
        match bank.create_account("Alice") {
            Ok(res) => panic!("Unexpected behavior when account already exists: {:?}", res),
            Err(res) => assert_eq!(
                res,
                BankError::AccountDuplication(AccountDuplicationError {
                    account: "Alice".to_string()
                })
            ),
        }
    }

    #[test]
    fn test_deposit() {
        let mut bank = bank_with_accounts!("Alice");

        match bank.deposit("Alice", 100.0) {
            Ok(res) => assert!(!res.is_empty()),
            Err(res) => panic!("Unexpected error: {:?}", res),
        };

        match bank.get_balance("Alice") {
            Ok(res) => assert_eq!(res, 100.0),
            Err(res) => panic!("Unexpected balance after deposit: {:?}", res),
        }
        match bank.deposit("Alice", -50.0) {
            Ok(res) => panic!("Unexpected error: {:?}", res),
            Err(res) => assert_eq!(
                res,
                BankError::AmountNegative(AmountNegativeError {
                    account: "Alice".to_string(),
                    amount: -50.0,
                })
            ),
        }
    }

    #[test]
    fn test_withdraw() {
        let mut bank = bank_with_accounts!("Alice");
        bank.deposit("Alice", 100.0).unwrap();

        match bank.withdraw("Alice", 50.0) {
            Ok(res) => assert!(!res.is_empty()),
            Err(res) => panic!("Unexpected error: {:?}", res),
        };

        match bank.get_balance("Alice") {
            Ok(res) => assert_eq!(res, 50.0),
            Err(res) => panic!("Unexpected balance after withdraw: {:?}", res),
        }

        match bank.withdraw("Alice", -30.0) {
            Ok(res) => panic!("Unexpected error: {:?}", res),
            Err(res) => assert_eq!(
                res,
                BankError::AmountNegative(AmountNegativeError {
                    account: "Alice".to_string(),
                    amount: -30.0,
                })
            ),
        }

        match bank.withdraw("Alice", 100.0) {
            Ok(res) => panic!("Unexpected error: {:?}", res),
            Err(res) => assert_eq!(
                res,
                BankError::InsufficientFunds(InsufficientFundsError {
                    account: "Alice".to_string(),
                    balance: 50.0,
                    amount: 100.0,
                })
            ),
        }
    }

    #[test]
    fn test_transfer() {
        let mut bank = bank_with_accounts!("Alice", "Bob");

        bank.deposit("Alice", 100.0).unwrap();
        let transaction_result = bank.transfer("Alice", "Bob", 50.0);
        assert!(transaction_result.is_ok());
        assert!(!transaction_result.unwrap().is_empty());
        assert_eq!(bank.get_balance("Alice").unwrap(), 50.0);
        assert_eq!(bank.get_balance("Bob").unwrap(), 50.0);
        //  Test for AmountNegativeError
        if let Err(res) = bank.transfer("Alice", "Bob", -30.0) {
            assert_eq!(
                res,
                AmountNegativeError {
                    amount: -30.0,
                    account: "Alice".to_string(),
                }
                .into()
            );
        } else {
            panic!("Unexpected logic")
        }
        //Test for InsufficientFundsError
        if let Err(res) = bank.transfer("Alice", "Bob", 100.0) {
            assert_eq!(
                res,
                InsufficientFundsError {
                    amount: 100.0,
                    account: "Alice".to_string(),
                    balance: 50.0,
                }
                .into()
            );
        } else {
            panic!("Unexpected logic")
        }
        //Test for SomeAccountTransferError
        if let Err(res) = bank.transfer("Alice", "Alice", 20.0) {
            assert_eq!(
                res,
                SomeAccountTransferError {
                    account: "Alice".to_string()
                }
                .into()
            );
        } else {
            panic!("Unexpected logic")
        }
        //Test for AccountNotFoundError
        if let Err(res) = bank.transfer("Eve", "Bob", 10.0) {
            assert_eq!(
                res,
                AccountNotFoundError {
                    account: "Eve".to_string()
                }
                .into()
            );
        } else {
            panic!("Unexpected logic")
        }
        //Test for AccountNotFoundError
        if let Err(res) = bank.transfer("Alice", "Eve", 10.0) {
            assert_eq!(
                res,
                AccountNotFoundError {
                    account: "Eve".to_string()
                }
                .into()
            );
        } else {
            panic!("Unexpected logic")
        }
    }

    #[test]
    fn test_transfer_without_target() {
        let mut bank = bank_with_accounts!("Alice");

        bank.deposit("Alice", 100.0).unwrap();
        let transaction_result = bank.transfer("Alice", "Bob", 50.0);
        assert!(transaction_result.is_err());
        assert_eq!(
            transaction_result.err().unwrap(),
            AccountNotFoundError {
                account: "Bob".to_string()
            }
            .into()
        )
    }

    #[test]
    fn test_get_balance() {
        let mut bank = bank_with_accounts!("Alice");
        bank.deposit("Alice", 100.0).unwrap();
        //        Test for
        if let Ok(res) = bank.get_balance("Alice") {
            assert_eq!(res, 100.0);
        } else {
            panic!("Unexpected logic")
        }
        // Test for AccountNotFoundError
        if let Err(res) = bank.get_balance("Bob") {
            assert_eq!(
                res,
                AccountNotFoundError {
                    account: "Bob".to_string()
                }
                .into()
            )
        } else {
            panic!("Unexpected logic")
        }
    }

    #[test]
    fn test_get_history() {
        let mut bank = bank_with_accounts!("Alice", "Bob");
        bank.deposit("Alice", 100.0).unwrap();
        bank.transfer("Alice", "Bob", 50.0).unwrap();

        match bank.get_history() {
            Ok(history) => {
                assert_eq!(history.len(), 4);

                println!("history: {:?}", history);
                assert_eq!(
                    history.iter().nth(0).unwrap().operation_type,
                    OperationType::CreateAccount
                );
                assert_eq!(
                    history.iter().nth(1).unwrap().operation_type,
                    OperationType::CreateAccount
                );
                assert_eq!(
                    history.iter().nth(2).unwrap().operation_type,
                    OperationType::Deposit
                );
                assert_eq!(
                    history.iter().nth(3).unwrap().operation_type,
                    OperationType::Transfer {
                        target_account: "Bob".to_string()
                    }
                );
            }
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
    }

    #[test]
    fn test_get_account_history() {
        let mut bank = bank_with_accounts!("Alice", "Bob");
        bank.deposit("Alice", 100.0).unwrap();
        bank.transfer("Alice", "Bob", 50.0).unwrap();

        match bank.get_account_history("Alice") {
            Ok(alice_history) => {
                assert_eq!(alice_history.len(), 3);
                assert_eq!(
                    alice_history[0].operation_type,
                    OperationType::CreateAccount
                );
                assert_eq!(alice_history[1].operation_type, OperationType::Deposit);
                assert_eq!(
                    alice_history[2].operation_type,
                    OperationType::Transfer {
                        target_account: "Bob".to_string()
                    }
                );
            }
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
        match bank.get_account_history("Bob") {
            Ok(bob_history) => {
                assert_eq!(bob_history.len(), 2);
                assert_eq!(bob_history[0].operation_type, OperationType::CreateAccount);
            }
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
    }

    #[test]
    fn test_replay_history() {
        let mut source_bank = bank_with_accounts!("Alice", "Bob");
        source_bank.deposit("Alice", 100.0).unwrap();
        source_bank.transfer("Alice", "Bob", 50.0).unwrap();
        let target_bank = Bank::replay_history(source_bank.get_history().unwrap().into_iter());
        assert_eq!(target_bank.get_balance("Alice").unwrap(), 50.0);
        assert_eq!(target_bank.get_balance("Bob").unwrap(), 50.0);
        // Checking Alice's history
        match target_bank.get_account_history("Alice") {
            Ok(alice_history) => {
                assert_eq!(alice_history.len(), 3);
                assert_eq!(
                    alice_history[0].operation_type,
                    OperationType::CreateAccount
                );
                assert_eq!(alice_history[1].operation_type, OperationType::Deposit);
                assert_eq!(
                    alice_history[2].operation_type,
                    OperationType::Transfer {
                        target_account: "Bob".to_string()
                    }
                );
            }
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
        // Checking Bob's history
        match target_bank.get_account_history("Bob") {
            Ok(bob_history) => {
                assert_eq!(bob_history.len(), 2);
                assert_eq!(bob_history[0].operation_type, OperationType::CreateAccount);
                assert_eq!(
                    bob_history[1].operation_type,
                    OperationType::Transfer {
                        target_account: "Bob".to_string()
                    }
                );
            }
            Err(res) => panic!("Unexpected error: {:?}", res),
        }
    }
    #[test]
    fn test_get_operation_by_id() {
        let mut bank = bank_with_accounts!("Alice", "Bob");
        let oper_1 = bank.deposit("Alice", 100.0).unwrap();
        let oper_2 = bank.transfer("Alice", "Bob", 50.0).unwrap();

        let res = bank.get_operation_by_id(&oper_1);
        if None == res {
            panic!("Unexpected error: result");
        }
        let res = res.unwrap();
        match res {
            oper => {
                assert_eq!(oper.operation_type, OperationType::Deposit);
                assert_eq!(oper.source_account, "Alice");
                assert_eq!(oper.amount, 100.0);
            }
        }

        let res = bank.get_operation_by_id(&oper_2);
        if None == res {
            panic!("Unexpected  result");
        }
        let res = res.unwrap();
        match res {
            oper => {
                assert_eq!(
                    oper.operation_type,
                    OperationType::Transfer {
                        target_account: "Bob".to_owned()
                    }
                );
                assert_eq!(oper.source_account, "Alice");
                assert_eq!(oper.amount, 50.0);
            }
        }
    }
}
