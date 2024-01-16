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
            println!("Insufficient balance");
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
    let mut owner = Account {
        holder: "Alejandro".to_string(),
        balance: 700.0,
    };

    //deposit
    owner.deposit_balance(200.0);
    owner.check_balance();

    // println!("======= --- ======");
    //
    //withdraw
    owner = owner.withdraw_balance(400.0);
    owner.check_balance()
}
