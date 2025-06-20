Learn Rust
===================

pub struct Account {
    id: i64,
    name: String
}

account:Account -> take ownership
account:&Account -> get read only reference
account:&mut Account -> get a mutable ref to account


General ideas
1) need to store the argument somewhere -> favor taking ownership (receive a value)
2) need to do a calculation with value -> favor receive read only reference
3) change the value but not store -> receive mut reference