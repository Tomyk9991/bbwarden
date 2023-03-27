#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WindowEvent;
use crate::account::AccountError;
use crate::commands::application::application_close;
use crate::commands::database::{fetch_passwords, unlock, logout, edit_entry};
use crate::states::LoggedInState;


#[derive(Debug)]
pub enum BackendError {
    Account(AccountError)
}

mod account;
mod commands;
mod states;
mod utils;

fn main() -> Result<(), BackendError> {
    tauri::Builder::default()
        .manage(LoggedInState::default())
        .invoke_handler(tauri::generate_handler![unlock, logout, fetch_passwords, edit_entry])
        .on_window_event(|event| match event.event() {
            WindowEvent::Destroyed {..} => { application_close(); return; },
            WindowEvent::CloseRequested { .. } => { application_close(); return; }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}
