use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CommunityBank {
    balances: std::collections::HashMap<AccountId, Balance>,
    total_supply: Balance,
}

#[near_bindgen]
impl CommunityBank {
    #[init]
    pub fn new(total_supply: Balance) -> Self {
        let mut this = Self {
            balances: std::collections::HashMap::new(),
            total_supply,
        };
        let owner_id = near_sdk::env::predecessor_account_id();
        this.balances.insert(owner_id.clone(), total_supply);
        this
    }

    pub fn transfer(&mut self, recipient: AccountId, amount: Balance) {
        let sender_id = near_sdk::env::predecessor_account_id();
        let sender_balance = self.balances.get(&sender_id).unwrap_or(&0);
        assert!(*sender_balance >= amount, "Not enough balance");
        let recipient_balance = self.balances.get(&recipient).unwrap_or(&0);
        self.balances.insert(sender_id.clone(), sender_balance - amount);
        self.balances.insert(recipient.clone(), recipient_balance + amount);
    }

    pub fn get_balance(&self, account_id: AccountId) -> Balance {
        self.balances.get(&account_id).unwrap_or(&0).clone()
    }
}
