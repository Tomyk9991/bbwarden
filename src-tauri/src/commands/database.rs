use serde::Serialize;
use tauri::State;
use uuid::Uuid;
use crate::account::{Account, AccountError, AccountPrinter, SafeEntry};
use crate::states::LoggedInState;
use crate::utils::funcs::assert_err;

#[derive(Serialize)]
pub struct AuthenticationDTO {
    user_name: String,
    session_id: Option<String>,
}

#[tauri::command]
pub fn unlock(user_name: &str, password: &str, logged_in_state: tauri::State<LoggedInState>) -> Result<AuthenticationDTO, AccountError> {
    let user_name_copied = user_name.to_string();
    let password_copied = password.to_string();
    
    let _ = Account::find(Box::new(move |account| {
        account.user_name == user_name_copied && account.password_hash == password_copied
    }))?;
    
    let session_id: Uuid = Uuid::new_v4();

    logged_in_state.set_session(Some(session_id.to_string()));
    logged_in_state.set_user_name(Some(user_name.to_string()));
    
    
    let session_binding = session_id.to_string();
    let account_printer = AccountPrinter::new(user_name, session_binding.as_str());
    println!("Login:{}", account_printer);

    return Ok(AuthenticationDTO {
        user_name: user_name.to_string(),
        session_id: Some(session_id.to_string()),
    });
}

#[tauri::command]
pub fn logout(session_id: &str, logged_in_state: tauri::State<LoggedInState>) -> Result<(), AccountError> {
    let user_name = logged_in_state.inner().account_name.read().unwrap().clone().ok_or(AccountError::UserNotFound)?;
    
    let printer = AccountPrinter::new(&user_name, session_id);
    println!("Logout:{}", printer);
    logged_in_state.set_session(None);
    
    Ok(())
}

#[tauri::command]
pub fn fetch_passwords(session_id: &str, logged_in_state: State<LoggedInState>) -> Result<Vec<SafeEntry>, AccountError> {
    assert_err(!logged_in_state.cmp_session(session_id)).ok_or(AccountError::SessionNotFound)?;
    
    let user_name = logged_in_state.inner().account_name.read().unwrap().clone().ok_or(AccountError::UserNotFound)?;
    
    let account = Account::find(Box::new(move |account| {
        user_name == account.user_name
    }))?;
    
    return Ok(account.entries);
}

#[tauri::command]
pub fn edit_entry(id: &str, session_id: &str, user_name: &str, password: &str, logged_in_state: State<LoggedInState>) -> Result<(), AccountError> {
    assert_err(!logged_in_state.cmp_session(session_id)).ok_or(AccountError::SessionNotFound)?;
    let user_name_copied = logged_in_state.inner().account_name.read().unwrap().clone().ok_or(AccountError::UserNotFound)?;
    
    let mut account = Account::find(Box::new(move |account| {
        user_name_copied == account.user_name
    }))?;
    
    let mut i = 0;
    while i < account.entries.len() {
        if account.entries[i].id == id {
            account.entries[i].user_name = user_name.to_string();
            account.entries[i].encrypted_password = password.to_string();
            break;
        }
        
        i += 1;
    }
    
    if let Err(err) = account.save() {
        println!("{err:?}");
    }
    
    Ok(())
}

#[tauri::command]
pub fn remove_entry(id: &str, session_id: &str, logged_in_state: State<LoggedInState>) -> Result<(), AccountError> {
    assert_err(!logged_in_state.cmp_session(session_id)).ok_or(AccountError::SessionNotFound)?;
    let user_name_copied = logged_in_state.inner().account_name.read().unwrap().clone().ok_or(AccountError::UserNotFound)?;

    let mut account = Account::find(Box::new(move |account| {
        user_name_copied == account.user_name
    }))?;
    
    let mut i: i32 = (account.entries.len() - 1) as i32;
    while i >= 0 {
        if account.entries[i as usize].id == id {
            account.entries.remove(i as usize);
            break;
        }
        
        i -= 1;
    }

    if let Err(err) = account.save() {
        println!("{err:?}");
    }

    Ok(())
}

#[tauri::command]
pub fn create_entry(session_id: &str, user_name: &str, password: &str, service: &str, logged_in_state: State<LoggedInState>) -> Result<SafeEntry, AccountError> {
    assert_err(!logged_in_state.cmp_session(session_id)).ok_or(AccountError::SessionNotFound)?;
    let user_name_copied = logged_in_state.inner().account_name.read().unwrap().clone().ok_or(AccountError::UserNotFound)?;

    let mut account = Account::find(Box::new(move |account| {
        user_name_copied == account.user_name
    }))?;

    let entry: SafeEntry = SafeEntry {
        id: Uuid::new_v4().to_string(),
        service: service.to_string(),
        user_name: user_name.to_string(),
        encrypted_password: password.to_string(),
    };
    
    account.entries.push(entry.clone());
    account.save()?;

    if let Err(err) = account.save() {
        println!("{err:?}");
    }

    Ok(entry)
}