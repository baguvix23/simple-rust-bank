# Simple Rust Account Manager

This Rust project implements a simple account management system, providing basic functionality such as deposit, balance withdrawal and balance inquiry. The `Account` structure represents an account with an account holder and a balance.

## Functionalities

- Balance Deposit:\*\* Allows deposits to be made into the account.
- Balance Withdrawal:\*\* Allows balance withdrawals, ensuring that there is sufficient balance.
- Balance Inquiry:\*\* Prints on the console the current balance and the account holder.

## Including

Include the library as a dependency to your project by adding the following lines to your **Cargo.toml** file:

```toml
[dependencies]
prettytable-rs = "^0.10"
```
