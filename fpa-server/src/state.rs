use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub id: u64,
}

pub fn shared() -> Arc<AppState> {
    Arc::new(AppState {
        id: 1010
    })
}
