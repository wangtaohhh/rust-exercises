struct BankAccount {
    id: u64, 
    balance: i64,
}


impl BankAccount {

    pub fn new(id: u64) -> Self {
        Self {
            id, 
            balance: 0i64,
        }
    }


    pub fn withdraw(&mut self, amount: u32, id_new: u64) {
        self.balance -= amount as i64;
        self.id = id_new;
    }


}




fn main() {

    let mut ba = BankAccount::new(1);
    ba.withdraw(10000, "qwe");
    println!("{}", ba.balance);
    println!("{}", ba.id);
    println!("Hello, world!");
}
