use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, Write};
use std::str::Utf8Error;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use crate::BackendError;

pub static STORAGE_DIRECTORY: &'static str = "../data";

#[derive(Debug, Clone, Serialize)]
pub enum AccountError {
    UserNotFound,
    CorruptedData(String),
    WrongPassword,
    SessionNotFound,
    EntryNotFound
}

impl From<std::io::Error> for AccountError {
    fn from(_: std::io::Error) -> Self {
        AccountError::UserNotFound
    }
}

impl From<Utf8Error> for AccountError {
    fn from(a: Utf8Error) -> Self {
        AccountError::CorruptedData(a.to_string())
    }
}

impl From<Error> for AccountError {
    fn from(a: Error) -> Self {
        AccountError::CorruptedData(a.to_string())
    }
}

impl From<AccountError> for BackendError {
    fn from(value: AccountError) -> Self {
        match value {
            a => {
                BackendError::Account(a) 
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Account {
    pub user_name: String,
    pub password_hash: String,
    pub entries: Vec<SafeEntry>
}

pub struct AccountPrinter<'a> {
    pub user_name: &'a str,
    pub session: &'a str
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SafeEntry {
    pub id: String,
    pub service: String,
    pub user_name: String,
    pub encrypted_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Session {
    pub session_id: String
}

impl<'a> AccountPrinter<'a> {
    pub fn new(user_name: &'a str, session: &'a str) -> Self {
        Self {
            user_name,
            session,
        }
    }
}

impl<'a> std::fmt::Display for AccountPrinter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\t-{}\n\t-{}", self.user_name.bright_yellow(), self.session.bright_yellow())
    }
}


impl Account {
    #[allow(unused)]
    pub fn save(&self) -> Result<(), AccountError> {
        let json = serde_json::ser::to_string(self)?;
        let mut file = File::create(format!("{}/{user_name}.json", STORAGE_DIRECTORY, user_name = self.user_name))?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

impl Account {
    pub fn search(func: Box<dyn Fn(&Account) -> bool>, abort_on_find: bool) -> Result<Vec<Account>, AccountError> {
        let mut accounts: Vec<Account> = vec![];

        let directory = std::fs::read_dir(STORAGE_DIRECTORY)?;
        for entry in directory {
            if let Ok(entry) = entry {
                let file = File::open(entry.path())?;
                let reader = BufReader::new(file);
                let obj: Account = serde_json::de::from_reader(reader)?;

                println!("{:?}", obj);
                
                if func(&obj) {
                    accounts.push(obj);
                    if abort_on_find { return Ok(accounts); }
                }
            }
        }

        return if !accounts.is_empty() {
            Ok(accounts)
        } else {
            Err(AccountError::UserNotFound)
        }

    }

    pub fn find(func: Box<dyn Fn(&Account) -> bool>) -> Result<Account, AccountError> {
        let s = Account::search(func, true)?;
        return Ok(s[0].clone());
    }
}
