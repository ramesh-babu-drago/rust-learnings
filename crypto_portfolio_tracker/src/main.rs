use std::collections::HashMap;
use std::io::{self, Write};

// Enum for different cryptocurrency types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CryptoCoin {
    Bitcoin,
    Ethereum,
    Solana,
    Cardano,
    Polkadot,
    Aptos,
}

// Enum for menu choices - demonstrates pattern matching
#[derive(Debug)]
enum MenuChoice{
    ViewPortfolio,
    AddCoin,
    ShowPrices,
    Exit,
    Invalid(String),
}

#[derive(Debug)]
enum PortfolioOperation {
    Replace(f64),
    Add(f64),
}

//Implementation block for CryptoCoin enum - adding methods to enums
impl CryptoCoin{
    fn from_string(input: &str) -> Option<CryptoCoin>{
        match input.to_lowercase().as_str(){
            "bitcoin" | "btc" => Some(CryptoCoin::Bitcoin),
            "ethereum" | "eth" => Some(CryptoCoin::Ethereum),
            "solana" | "sol" => Some(CryptoCoin::Solana),
            "cardano" | "ada" => Some(CryptoCoin::Cardano),
            "polkadot" | "dot" => Some(CryptoCoin::Polkadot),
            "aptos" | "apt" => Some(CryptoCoin::Aptos),
            _ => None,
        }
    }

    fn display_name(&self) -> &str {
        match self{
            CryptoCoin::Bitcoin => "Bitcoin (BTC)",
            CryptoCoin::Ethereum => "Ethereum (ETH)",
            CryptoCoin::Solana => "Solana (SOL)",
            CryptoCoin::Cardano => "Cardano (ADA)",
            CryptoCoin::Polkadot => "Polkadot (DOT)",
            CryptoCoin::Aptos => "Aptos (APT)"
        }
    }

    //Method to get a symbol
    fn symbol(&self) -> &str {
        match self{
            CryptoCoin::Bitcoin => "BTC",
            CryptoCoin::Ethereum => "ETH",
            CryptoCoin::Solana => "SOL",
            CryptoCoin::Cardano => "ADA",
            CryptoCoin::Polkadot => "DOT",
            CryptoCoin::Aptos => "APT"
        }
    }
}

impl MenuChoice{
    fn from_input(input: &str) -> MenuChoice{
        match input.trim() {
            "1" => MenuChoice::ViewPortfolio,
            "2" => MenuChoice::AddCoin,
            "3" => MenuChoice::ShowPrices,
            "4" => MenuChoice::Exit,
            invalid => MenuChoice::Invalid(invalid.to_string()),
        }
    }
}

// Struct to represent the portfolio tracker
struct PortfolioTracker{
    prices: HashMap<CryptoCoin, f64>,
    portfolio: HashMap<CryptoCoin, f64>,
}

impl PortfolioTracker{
    fn new() -> Self {
        let mut prices = HashMap::new();

        //Initialize price database - real world example prices
        prices.insert(CryptoCoin::Bitcoin, 45000.0);
        prices.insert(CryptoCoin::Ethereum, 2000.0);
        prices.insert(CryptoCoin::Solana, 157.0);
        prices.insert(CryptoCoin::Cardano, 0.45);
        prices.insert(CryptoCoin::Polkadot, 10.01);
        prices.insert(CryptoCoin::Aptos, 4.8);

        PortfolioTracker {
            prices,
            portfolio: HashMap::new(),
        }
    }

    fn display_menu(&self){
        println!("\n --------- CRYPTO PORTFOLIO TRACKER  ---------");
        println!("1. View Portfolio");
        println!("2. Add/Update Coin");
        println!("3. Show Prices");
        println!("4. Exit");
        print!("Enter your choice (1-4): ");
        io::stdout().flush().unwrap();
    }

    fn get_user_input(&self) -> String{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    //View portfolio with calculations
    fn view_portfolio(&self){
        if self.portfolio.is_empty(){
            println!("\n Portfolio is empty. Add coins to view portfolio.");
            return;
        }

        println!("\n -------------- PORTFOLIO ---------------");
        println!("{:<15} {:<10} {:<12} {:<12}", "Coin", "Amount", "Price", "Value");
        println!("{}", "-".repeat(50));

        let mut total_value = 0.0;

        for (coin, amount) in &self.portfolio{
            // Using Option<T> to safely get price - demonstrates null safety

            if let Some(price) = self.prices.get(coin){
                let value = amount * price;
                total_value += value;

                println!(
                    "{:<15} {:<10.4} ${:<11.2} ${:<11.2}",
                    coin.display_name(),
                    amount,
                    price,
                    value
                );
            }
        }

        println!("{}", "-".repeat(50));
        println!("Total Value: ${:.2}", total_value);
    }

    //Add/Update coin
    fn add_coin(&mut self){
        println!("\n Add/Update Coin");
        println!("Available coins: Bitcoin, Ethereum, Solana, Cardano, Polkadot, Aptos");
        print!("Enter coin name: ");
        io::stdout().flush().unwrap();

        let coin_input = self.get_user_input();

        //using Option<T> for safe conversion - demonstrates null safety
        let coin = match CryptoCoin::from_string(&coin_input){
            Some(c) => c,
            None => {
                println!("Invalid coin name. Try again.");
                return;
            }
        };

        // Check if the coin already exists in the portfolio
        let existing_amount = self.portfolio.get(&coin).copied().unwrap_or(0.0);

        if existing_amount > 0.0 {
            println!("You currently own {:.4} {}", existing_amount, coin.symbol());
            print!("Do you want to (R)replace or (A)dd to existing amount? ");
            io::stdout().flush().unwrap();

            let choice = self.get_user_input().to_lowercase();
            let operation = match choice.as_str() {
                "a" | "add" => PortfolioOperation::Add(existing_amount),
                "r" | "replace" | "" => PortfolioOperation::Replace(0.0),
                _ => {
                    println!("Invalid choice. Defaulting to replace.");
                    PortfolioOperation::Replace(0.0)
                }
            };

            print!("Enter amount: ");
            io::stdout().flush().unwrap();

            let amount_input = self.get_user_input();
            let amount: f64 = match amount_input.parse() {
                Ok(a) if a > 0.0 => a,
                _ => {
                    println!(" Invalid amount. Please enter a positive number.");
                    return;
                }
            };

            // Pattern matching on the operation enum
            let final_amount = match operation {
                PortfolioOperation::Add(existing) => existing + amount,
                PortfolioOperation::Replace(_) => amount,
            };

            self.portfolio.insert(coin.clone(), final_amount);

            match operation {
                PortfolioOperation::Add(_) => {
                    println!(" Added {:.4} {} to your portfolio!", amount, coin.symbol());
                    println!("Total {} holdings: {:.4}", coin.symbol(), final_amount);
                }
                PortfolioOperation::Replace(_) => {
                    println!(" Updated {} holdings to {:.4}!", coin.symbol(), final_amount);
                }
            }
        } else {
            print!("Enter amount: ");
            io::stdout().flush().unwrap();

            let amount_input = self.get_user_input();
            let amount: f64 = match amount_input.parse() {
                Ok(a) if a > 0.0 => a,
                _ => {
                    println!(" Invalid amount. Please enter a positive number.");
                    return;
                }
            };

            self.portfolio.insert(coin.clone(), amount);
            println!(" Added {:.4} {} to your portfolio!", amount, coin.symbol());
        }
    }

    // Show all available prices
    fn show_prices(&self) {
        println!("\n === CURRENT CRYPTO PRICES ===");
        println!("{:<20} {:<12}", "Coin", "Price (USD)");
        println!("{}", "-".repeat(33));

        // Sort coins by name for a consistent display
        let mut sorted_prices: Vec<_> = self.prices.iter().collect();
        sorted_prices.sort_by_key(|(coin, _)| coin.display_name());

        for (coin, price) in sorted_prices {
            println!("{:<20} ${:<11.2}", coin.display_name(), price);
        }
    }

    // Main program loop
    fn run(&mut self) {
        println!(" Welcome to the Crypto Portfolio Tracker!");

        loop {
            self.display_menu();
            let input = self.get_user_input();
            let choice = MenuChoice::from_input(&input);

            // Pattern matching on MenuChoice enum - demonstrates match expression
            match choice {
                MenuChoice::ViewPortfolio => self.view_portfolio(),
                MenuChoice::AddCoin => self.add_coin(),
                MenuChoice::ShowPrices => self.show_prices(),
                MenuChoice::Exit => {
                    println!("\n Thank you for using Crypto Portfolio Tracker!");
                    println!("Happy trading! ");
                    break;
                }
                MenuChoice::Invalid(ref invalid_input) => {
                    println!(" Invalid choice: '{}'. Please enter 1-4.", invalid_input);
                }

            }

            // Optional: Add a small pause for better UX
            if !matches!(choice, MenuChoice::Exit) {
                println!("\nPress Enter to continue...");
                self.get_user_input();
            }
        }
    }
}

fn main() {
    // Create and run the portfolio tracker
    let mut tracker = PortfolioTracker::new();
    tracker.run();
}

// Additional example showing more advanced enum usage
#[allow(dead_code)]
enum TradeType {
    Buy { amount: f64, price: f64 },
    Sell { amount: f64, price: f64 },
    Transfer { from: String, to: String, amount: f64 },
}

#[allow(dead_code)]
impl TradeType {
    fn execute(&self) -> String {
        match self {
            TradeType::Buy { amount, price } => {
                format!("Executed BUY order: {} coins at ${} each", amount, price)
            }
            TradeType::Sell { amount, price } => {
                format!("Executed SELL order: {} coins at ${} each", amount, price)
            }
            TradeType::Transfer { from, to, amount } => {
                format!("Transferred {} coins from {} to {}", amount, from, to)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_coin_from_string() {
        assert_eq!(CryptoCoin::from_string("bitcoin"), Some(CryptoCoin::Bitcoin));
        assert_eq!(CryptoCoin::from_string("BTC"), Some(CryptoCoin::Bitcoin));
        assert_eq!(CryptoCoin::from_string("invalid"), None);
    }

    #[test]
    fn test_menu_choice_from_input() {
        assert!(matches!(MenuChoice::from_input("1"), MenuChoice::ViewPortfolio));
        assert!(matches!(MenuChoice::from_input("invalid"), MenuChoice::Invalid(_)));
    }
}