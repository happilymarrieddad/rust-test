mod banks;

fn main() {
    let mut bank = banks::Bank::new();
    let mut account = banks::Account::new(1, String::from("Nick Kotenberg"));
    let mut account2 = banks::Account::new(2, String::from("Candice Dickson"));

    account.deposit(500);
    account.withdraw(250);

    account2.deposit(300);

    bank.add_account(account);
    bank.add_account(account2);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
