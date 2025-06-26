use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
pub struct GlobalState {
    pub account_state: String,
    pub mnemonic_phrase: Option<String>,
    pub phase_exists_in_state: bool,
}
