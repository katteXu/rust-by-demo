use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}
struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

// 登录
fn try_login<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successfully logged");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed"),
    }
}
fn main() {
    let mut accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@gmail.com",
    };

    accounts.insert(account, account_info);
    println!("开始登录 ======>");
    try_login(&accounts, "j.everyman", "password123");
    println!("开始登录 ======>");
    try_login(&accounts, "j.everyman", "password234");
}
