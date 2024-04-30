use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use crate::services;


/// AppState struct
///
/// Describe the state of the application
///
/// # Example
///
/// ```
/// use app_state::AppState;
///
/// let app_state = AppState::new();
/// ```
///
/// # Fields
///
/// * `services_state` - The state of the services
///
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub services_state: services::ServiceState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            services_state: services::ServiceState::default(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}


/// State type
///
/// A type alias for `Arc<RwLock<AppState>>`
///
/// # Example
///
/// ```
/// use app_state::State;
///
/// let state = State::new();
///
/// ```
///
/// # Fields
///
/// * `services_state` - The state of the services
pub type State = Arc<RwLock<AppState>>;


impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            services_state: self.services_state.clone(),
        }
    }
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_app_state_new() {
        let app_state = AppState::new();
        assert_eq!(app_state.services_state.service_groups.len(), 0);
    }
}
