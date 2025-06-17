#[derive(Debug, Clone, PartialEq)]
enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    order_type: OrderType,
    amount: f64,
    price: f64,
}

#[derive(Debug)]
struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    next_id: u32,
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            next_id: 1,
        }
    }

    fn add_order(&mut self, order_type: OrderType, amount: f64, price: f64) {
        let order = Order {
            id: self.next_id,
            order_type: order_type.clone(),
            amount,
            price,
        };

        match order_type {
            OrderType::Buy => self.buy_orders.push(order),
            OrderType::Sell => self.sell_orders.push(order),
        }

        self.next_id += 1;
    }

    fn show_order_book(&self) {
        println!("=== ORDER BOOK ===");

        println!("\n BUY ORDERS:");
        if self.buy_orders.is_empty() {
            println!("  No buy orders");
        } else {
            for order in &self.buy_orders {
                println!("  ID: {} | Type: {:?} | Amount: {:.2} | Price: ${:.2}",
                         order.id, order.order_type, order.amount, order.price);
            }
        }

        println!("\n SELL ORDERS:");
        if self.sell_orders.is_empty() {
            println!("  No sell orders");
        } else {
            for order in &self.sell_orders {
                println!("  ID: {} | Type: {:?} | Amount: {:.2} | Price: ${:.2}",
                         order.id, order.order_type, order.amount, order.price);
            }
        }
        println!("==================\n");
    }

    fn total_orders(&self) -> usize {
        self.buy_orders.len() + self.sell_orders.len()
    }

    fn get_orders_by_type(&self, order_type: &OrderType) -> &Vec<Order> {
        match order_type {
            OrderType::Buy => &self.buy_orders,
            OrderType::Sell => &self.sell_orders,
        }
    }

    fn find_order_by_id(&self, id: u32) -> Option<&Order> {
        for order in &self.buy_orders {
            if order.id == id {
                return Some(order);
            }
        }
        for order in &self.sell_orders {
            if order.id == id {
                return Some(order);
            }
        }
        None
    }

    fn get_total_value_by_type(&self, order_type: &OrderType) -> f64 {
        let orders = self.get_orders_by_type(order_type);
        orders.iter()
            .map(|order| order.amount * order.price)
            .sum()
    }
}

fn main() {
    println!(" Order Book System Demo\n");

    let mut order_book = OrderBook::new();

    println!("Adding buy orders...");
    order_book.add_order(OrderType::Buy, 100.0, 50.25);
    order_book.add_order(OrderType::Buy, 200.0, 49.80);
    order_book.add_order(OrderType::Buy, 150.0, 51.00);

    println!("Adding sell orders...");
    order_book.add_order(OrderType::Sell, 75.0, 52.50);
    order_book.add_order(OrderType::Sell, 300.0, 53.20);
    order_book.add_order(OrderType::Sell, 125.0, 51.75);

    order_book.show_order_book();

    println!(" Order Book Statistics:");
    println!("Total orders: {}", order_book.total_orders());
    println!("Buy orders: {}", order_book.buy_orders.len());
    println!("Sell orders: {}", order_book.sell_orders.len());

    println!("\n Finding order by ID:");
    if let Some(order) = order_book.find_order_by_id(3) {
        println!("Found order ID 3: {:?} - Amount: {}, Price: ${}",
                 order.order_type, order.amount, order.price);
    }

    // Demonstrate total value calculations
    let buy_total = order_book.get_total_value_by_type(&OrderType::Buy);
    let sell_total = order_book.get_total_value_by_type(&OrderType::Sell);
    println!("\n Total Values:");
    println!("Buy orders total value: ${:.2}", buy_total);
    println!("Sell orders total value: ${:.2}", sell_total);

    // Demonstrate immutable borrowing
    let buy_orders_ref = order_book.get_orders_by_type(&OrderType::Buy);
    println!("\n Buy orders via reference: {} orders", buy_orders_ref.len());

    // Demonstrate pattern matching with order types
    let order_types = vec![OrderType::Buy, OrderType::Sell];
    for ot in &order_types {
        let count = order_book.get_orders_by_type(ot).len();
        match ot {
            OrderType::Buy => println!(" Buy orders count: {}", count),
            OrderType::Sell => println!(" Sell orders count: {}", count),
        }
    }
}