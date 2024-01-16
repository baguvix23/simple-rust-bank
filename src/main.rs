use std::io;

use prettytable::{row, Table};

extern crate prettytable;
struct Account {
    holder: String,
    balance: f32,
}

impl Account {
    fn withdraw_balance(self, amount: f32) -> Account {
        if amount <= self.balance {
            println!(
                "Retirement successfull, please take your money -> {}",
                amount
            );
            Account {
                holder: self.holder,
                balance: self.balance - amount,
            }
        } else {
            println!("Insufficient balance, the current cash is {}", self.balance);
            self
        }
    }

    fn check_balance(&self) {
        let mut table = Table::new();
        table.add_row(row!["Holder", "Balance"]);
        table.add_row(row![&self.holder, &self.balance]);

        table.printstd()
    }

    fn deposit_balance(&mut self, money: f32) {
        println!("The deposit successfull {}", money);
        self.balance += money
    }
}

fn main() {
    loop {
        let mut owner = Account {
            holder: "Alejandro".to_string(),
            balance: 700.0,
        };

        println!("Select an option");
        println!("1.Deposit amount ");
        println!("2.Withdraw amount");
        println!("3.Exit ");

        let mut menu = String::new();
        io::stdin()
            .read_line(&mut menu)
            .expect("Failed to read line");

        let menu: usize = match menu.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match menu {
            1 => {
                //deposit
                println!("Enter your amount to deposit");
                let deposit_amount: f32 = read_input();
                owner.deposit_balance(deposit_amount);
                owner.check_balance();
            }
            2 => {
                //withdraw_amount
                println!("Enter the amount to withdraw:");
                let withdraw_amount: f32 = read_input();
                owner = owner.withdraw_balance(withdraw_amount);
                owner.check_balance()
            }
            3 => {
                println!("Exit program");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }

        println!("======= --- ======");
        //withdraw
    }
}

fn read_input() -> f32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .parse()
        .expect("please enter your number valid")
}

// fn option() {
//     let mut options = String::new();
//     io::stdin()
//         .read_line(&mut options)
//         .expect("Failded to read line");
// }
