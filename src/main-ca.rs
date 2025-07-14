use std::cmp::min;

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    side: OrderSide,
    price: f64,
    quantity: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug)]
struct Trade {
    buy_order_id: u32,
    sell_order_id: u32,
    quantity: u32,
    clearing_price: f64,
}

struct NBBO {
    bid: f64,
    ask: f64,
}

struct AuctionEngine {
    nbbo: NBBO,
}

impl AuctionEngine {
    fn new(nbbo: NBBO) -> Self {
        AuctionEngine { nbbo }
    }

    fn run_greedy_auction(&self, buy_orders: &[Order], sell_orders: &[Order]) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut buy_orders = buy_orders.to_vec();
        let mut sell_orders = sell_orders.to_vec();

        for order in &buy_orders {
            if order.side != OrderSide::Buy {
                panic!("Invalid buy order ID {}: expected Buy, found {:?}", order.id, order.side);
            }
        }
        for order in &sell_orders {
            if order.side != OrderSide::Sell {
                panic!("Invalid sell order ID {}: expected Sell, found {:?}", order.id, order.side);
            }
        }

        buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        for buy_idx in 0..buy_orders.len() {
            for sell_idx in 0..sell_orders.len() {
                if buy_orders[buy_idx].quantity == 0 || sell_orders[sell_idx].quantity == 0 {
                    continue;
                }

                let buy_price = buy_orders[buy_idx].price;
                let sell_price = sell_orders[sell_idx].price;

                if buy_price >= sell_price
                    && buy_price <= self.nbbo.ask
                    && sell_price >= self.nbbo.bid
                {
                    let match_quantity = min(buy_orders[buy_idx].quantity, sell_orders[sell_idx].quantity);
                    let clearing_price = (buy_price + sell_price) / 2.0;

                    trades.push(Trade {
                        buy_order_id: buy_orders[buy_idx].id,
                        sell_order_id: sell_orders[sell_idx].id,
                        quantity: match_quantity,
                        clearing_price,
                    });

                    buy_orders[buy_idx].quantity -= match_quantity;
                    sell_orders[sell_idx].quantity -= match_quantity;
                }
            }
        }

        trades
    }

    // New combinatorial version: multi-unit double auction with uniform clearing price
    fn run_combinatorial_auction(&self, buy_orders: &[Order], sell_orders: &[Order]) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut buy_orders = buy_orders.to_vec();
        let mut sell_orders = sell_orders.to_vec();

        for order in &buy_orders {
            if order.side != OrderSide::Buy {
                panic!("Invalid buy order ID {}: expected Buy, found {:?}", order.id, order.side);
            }
        }
        for order in &sell_orders {
            if order.side != OrderSide::Sell {
                panic!("Invalid sell order ID {}: expected Sell, found {:?}", order.id, order.side);
            }
        }

        buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        // First pass: find matches, total volume, and marginal prices (no trades yet)
        let mut matches: Vec<(u32, u32, u32)> = Vec::new(); // (buy_id, sell_id, quantity)
        let mut marginal_buy = 0.0;
        let mut marginal_sell = 0.0;

        for buy_idx in 0..buy_orders.len() {
            for sell_idx in 0..sell_orders.len() {
                if buy_orders[buy_idx].quantity == 0 || sell_orders[sell_idx].quantity == 0 {
                    continue;
                }

                let buy_price = buy_orders[buy_idx].price;
                let sell_price = sell_orders[sell_idx].price;

                if buy_price >= sell_price
                    && buy_price <= self.nbbo.ask
                    && sell_price >= self.nbbo.bid
                {
                    let match_quantity = min(buy_orders[buy_idx].quantity, sell_orders[sell_idx].quantity);

                    matches.push((buy_orders[buy_idx].id, sell_orders[sell_idx].id, match_quantity));

                    marginal_buy = buy_price;
                    marginal_sell = sell_price;

                    buy_orders[buy_idx].quantity -= match_quantity;
                    sell_orders[sell_idx].quantity -= match_quantity;
                }
            }
        }

        // Compute uniform clearing price if there were matches
        if matches.is_empty() {
            return trades; // No trades
        }
        let uniform_clearing_price = (marginal_buy + marginal_sell) / 2.0;

        // Second pass: create trades at the uniform clearing price
        for (buy_id, sell_id, quantity) in matches {
            trades.push(Trade {
                buy_order_id: buy_id,
                sell_order_id: sell_id,
                quantity,
                clearing_price: uniform_clearing_price,
            });
        }

        trades
    }
}

fn main() {
    // Simulated NBBO for a stock (e.g., bid $99.50, ask $100.50)
    let nbbo = NBBO {
        bid: 99.50,
        ask: 100.50,
    };

    let engine = AuctionEngine::new(nbbo);

    let buy_orders = vec![
        Order {
            id: 1,
            side: OrderSide::Buy,
            price: 100.00,
            quantity: 100,
        },
        Order {
            id: 2,
            side: OrderSide::Buy,
            price: 99.75,
            quantity: 200,
        },
    ];

    let sell_orders = vec![
        Order {
            id: 3,
            side: OrderSide::Sell,
            price: 99.60,
            quantity: 150,
        },
        Order {
            id: 4,
            side: OrderSide::Sell,
            price: 100.10,
            quantity: 100,
        },
    ];

    // Run greedy auction (per-pair prices)
    let greedy_trades = engine.run_greedy_auction(&buy_orders, &sell_orders);
    println!("Greedy Matched Trades:");
    for trade in greedy_trades {
        println!(
            "Trade: Buy Order {} <> Sell Order {}, Quantity: {}, Clearing Price: ${:.2}",
            trade.buy_order_id,
            trade.sell_order_id,
            trade.quantity,
            trade.clearing_price
        );
    }

    // Run combinatorial auction (uniform clearing price)
    let combo_trades = engine.run_combinatorial_auction(&buy_orders, &sell_orders);
    println!("\nCombinatorial Matched Trades (Uniform Clearing):");
    for trade in combo_trades {
        println!(
            "Trade: Buy Order {} <> Sell Order {}, Quantity: {}, Clearing Price: ${:.2}",
            trade.buy_order_id,
            trade.sell_order_id,
            trade.quantity,
            trade.clearing_price
        );
    }
}