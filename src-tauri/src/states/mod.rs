use std::sync::{RwLock};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoggedInState {
    pub session_id: RwLock<Option<String>>,
    pub account_name: RwLock<Option<String>>
}


impl Default for LoggedInState {
    fn default() -> Self {
        Self {
            session_id: RwLock::new(None),
            account_name: RwLock::new(None)
        }
    }
}
impl LoggedInState {
    pub fn cmp_session(&self, session: &str) -> bool {
        let state_guard = self.session_id.read().unwrap();
        return match &*state_guard {
            None => { false }
            Some(a) => { a == session }
        };
    }
    
    pub fn set_session(&self, session: Option<String>) {
        let mut state_guard = self.session_id.write().unwrap();
        *state_guard = session;
    }

    pub fn set_user_name(&self, user: Option<String>) {
        let mut state_guard = self.account_name.write().unwrap();
        *state_guard = user;
    }
}
