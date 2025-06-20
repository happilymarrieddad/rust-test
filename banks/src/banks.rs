#[derive(Debug)]
pub struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    pub fn new(id: u32, holder: String) -> Self {
        Account{id: id,balance: 0,holder: holder}
    }

    pub fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    pub fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    pub fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {accounts: vec![]}
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn total_balance(&self) -> i32 {
        self.accounts
            .iter()
            .map(|account| account.balance)
            .sum()
    }

    pub fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}