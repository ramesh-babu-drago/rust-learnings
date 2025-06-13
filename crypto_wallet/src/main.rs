#[derive(Debug, Clone)]
pub struct Wallet {
    pub balance: u64,
    pub id: String,
}

impl Wallet {
    // 1. Create a new wallet with initial balance
    pub fn new_wallet(balance: u64) -> Wallet {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        // Generate a simple ID based on timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut hasher = DefaultHasher::new();
        timestamp.hash(&mut hasher);
        let id = format!("wallet_{:x}", hasher.finish());

        Wallet { balance, id }
    }

    // 2. Check balance (immutable borrow)
    pub fn check_balance(wallet: &Wallet) -> u64 {
        wallet.balance
    }

    // 3. Send money (mutable borrow) - returns Result for error handling
    pub fn send_money(wallet: &mut Wallet, amount: u64) -> Result<(), String> {
        if wallet.balance < amount {
            return Err(format!("Insufficient balance! Have: {}, Need: {}", wallet.balance, amount));
        }
        wallet.balance -= amount;
        Ok(())
    }

    // 4. Transfer ownership (moves the wallet)
    pub fn transfer_ownership(wallet: Wallet) -> Wallet {
        wallet // This moves ownership
    }

    // 5. Transfer between wallets
    pub fn transfer_between(from: &mut Wallet, to: &mut Wallet, amount: u64) -> Result<(), String> {
        if from.balance < amount {
            return Err(format!("Insufficient balance in source wallet! Have: {}, Need: {}", from.balance, amount));
        }
        from.balance -= amount;
        to.balance += amount;
        Ok(())
    }

    // 6. Get wallet info
    pub fn get_wallet_info(wallet: &Wallet) -> String {
        format!("Wallet ID: {}, Balance: {} ETH", wallet.id, wallet.balance)
    }

    // 7. Calculate the total balance of multiple wallets
    pub fn batch_check(wallets: &[Wallet]) -> u64 {
        wallets.iter().map(|w| w.balance).sum()
    }

    // 8. Clone wallet (create backup)
    pub fn clone_wallet(wallet: &Wallet) -> Wallet {
        wallet.clone()
    }
}

fn main() {
    println!("Crypto Wallet System Demo\n");

    // Task 1: Create wallets with some ETH
    println!("1. Creating wallets...");
    let mut wallet1 = Wallet::new_wallet(100);
    let mut wallet2 = Wallet::new_wallet(50);
    let wallet3 = Wallet::new_wallet(75);

    println!("   {}", Wallet::get_wallet_info(&wallet1));
    println!("   {}", Wallet::get_wallet_info(&wallet2));
    println!("   {}", Wallet::get_wallet_info(&wallet3));

    // Task 2: Check balance without losing ownership
    println!("\n2. Checking balances (immutable borrow)...");
    println!("   Wallet1 balance: {} ETH", Wallet::check_balance(&wallet1));
    println!("   Wallet2 balance: {} ETH", Wallet::check_balance(&wallet2));
    // wallet1 is still usable here because we only borrowed it!

    // Task 3: Send some ETH using mutable borrow
    println!("\n3. Sending ETH from wallet1...");
    match Wallet::send_money(&mut wallet1, 30) {
        Ok(()) => println!(" Successfully sent 30 ETH"),
        Err(e) => println!("  Error: {}", e),
    }
    println!("   Wallet1 new balance: {} ETH", Wallet::check_balance(&wallet1));

    // Task 4: Transfer money between two wallets
    println!("\n4. Transferring between wallets...");
    println!("   Before transfer:");
    println!("     Wallet1: {} ETH", Wallet::check_balance(&wallet1));
    println!("     Wallet2: {} ETH", Wallet::check_balance(&wallet2));

    match Wallet::transfer_between(&mut wallet1, &mut wallet2, 20) {
        Ok(()) => println!("  Successfully transferred 20 ETH from wallet1 to wallet2"),
        Err(e) => println!("  Error: {}", e),
    }

    println!("   After transfer:");
    println!("     Wallet1: {} ETH", Wallet::check_balance(&wallet1));
    println!("     Wallet2: {} ETH", Wallet::check_balance(&wallet2));

    // Task 5: Calculate total balance of 3+ wallets
    println!("\n5. Calculating total balance of all wallets...");
    let wallets = [&wallet1, &wallet2, &wallet3];
    let wallet_refs: Vec<Wallet> = wallets.iter().map(|&w| w.clone()).collect();
    let total = Wallet::batch_check(&wallet_refs);
    println!("   Total balance across all wallets: {} ETH", total);

    // Task 6: Transfer wallet ownership
    println!("\n6. Transferring wallet ownership...");
    println!("   Before transfer - Wallet3: {}", Wallet::get_wallet_info(&wallet3));
    let wallet3_new_owner = Wallet::transfer_ownership(wallet3);
    println!("   After transfer - New owner has: {}", Wallet::get_wallet_info(&wallet3_new_owner));

    // Task 7: Try to use original wallet after transfer (will cause compile error)
    println!("\n7. Attempting to use original wallet after ownership transfer...");
    println!("   This would cause a compile error if uncommented:");
    println!("   // println!(\"Original wallet3: {{}}\", Wallet::get_wallet_info(&wallet3));");
    println!("   Compile Error: borrow of moved value: `wallet3`");

    // Task 8: Handle insufficient balance
    println!("\n8. Testing insufficient balance handling...");
    println!("   Attempting to send 1000 ETH from wallet1 (balance: {} ETH)...", Wallet::check_balance(&wallet1));
    match Wallet::send_money(&mut wallet1, 1000) {
        Ok(()) => println!("Transaction successful"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    // Bonus: Clone wallet demonstration
    println!("\n🎯 Bonus: Cloning wallet...");
    let wallet1_backup = Wallet::clone_wallet(&wallet1);
    println!("   Original: {}", Wallet::get_wallet_info(&wallet1));
    println!("   Backup:   {}", Wallet::get_wallet_info(&wallet1_backup));

    println!("\n✨ Demo completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let wallet = Wallet::new_wallet(100);
        assert_eq!(wallet.balance, 100);
        assert!(wallet.id.starts_with("wallet_"));
    }

    #[test]
    fn test_check_balance() {
        let wallet = Wallet::new_wallet(50);
        assert_eq!(Wallet::check_balance(&wallet), 50);
    }

    #[test]
    fn test_send_money_success() {
        let mut wallet = Wallet::new_wallet(100);
        let result = Wallet::send_money(&mut wallet, 30);
        assert!(result.is_ok());
        assert_eq!(wallet.balance, 70);
    }

    #[test]
    fn test_send_money_insufficient_balance() {
        let mut wallet = Wallet::new_wallet(10);
        let result = Wallet::send_money(&mut wallet, 20);
        assert!(result.is_err());
        assert_eq!(wallet.balance, 10); // Balance should remain unchanged
    }

    #[test]
    fn test_transfer_between() {
        let mut wallet1 = Wallet::new_wallet(100);
        let mut wallet2 = Wallet::new_wallet(50);

        let result = Wallet::transfer_between(&mut wallet1, &mut wallet2, 30);
        assert!(result.is_ok());
        assert_eq!(wallet1.balance, 70);
        assert_eq!(wallet2.balance, 80);
    }

    #[test]
    fn test_batch_check() {
        let wallets = vec![
            Wallet::new_wallet(100),
            Wallet::new_wallet(50),
            Wallet::new_wallet(25),
        ];
        assert_eq!(Wallet::batch_check(&wallets), 175);
    }

    #[test]
    fn test_clone_wallet() {
        let original = Wallet::new_wallet(100);
        let cloned = Wallet::clone_wallet(&original);
        assert_eq!(original.balance, cloned.balance);
        assert_eq!(original.id, cloned.id);
    }
}