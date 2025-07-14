# Smart Auction Engine: A Simplified Model for Order Matching

## Abstract
This project presents a lightweight, Rust-based implementation of an auction engine designed to simulate order matching in an Alternative Trading System (ATS) for U.S. equities. Drawing inspiration from modern Smart Markets, the engine employs a greedy pairwise matching algorithm to optimize trades while adhering to National Best Bid and Offer (NBBO) constraints. By prioritizing aggressive orders and ensuring price improvement through midpoint clearing, it demonstrates foundational principles of efficient, fair execution.

** This model serves as an educational tool for understanding batch auctions, with potential extensions to combinatorial optimization for real-world scalability. **

## Introduction
In traditional financial markets, order matching often relies on continuous limit order books, which can suffer from issues such as information asymmetry, adverse selection, and suboptimal liquidity for complex strategies. Smart Markets, as exemplified by innovative ATS platforms, address these challenges through periodic auctions that leverage mathematical optimization to unlock mutually beneficial trades.

This implementation provides a minimal viable model of such a system in Rust, focusing on safety, performance, and clarity. It processes buy and sell orders in a single-security context, validating inputs, sorting for priority, and executing matches in a batched manner. While not production-ready, it illustrates key concepts like NBBO compliance and quantity management, making it suitable for prototyping, interviews, or educational purposes.

## Design Principles
The engine is built on the following core tenets:

- Fairness and Compliance: All matches occur within the NBBO to ensure regulatory adherence and prevent off-market executions.
- Efficiency: A greedy algorithm minimizes computational overhead while approximating price improvement.
- Extensibility: Structured to allow future integration of advanced features, such as multi-security bundles or Expressive Bidding-inspired constraints.
- Safety: Leveraging Rust's ownership model and panic-based validation to enforce order integrity (e.g., side checks).

## Key Components
- Order Struct: Represents buy or sell orders with an ID, side (Buy/Sell enum), price, and quantity.
- Trade Struct: Captures matched outcomes, including buyer/seller IDs, filled quantity, and clearing price.
- NBBO Struct: Simulates the National Best Bid and Offer as fixed bounds for valid matches.
- AuctionEngine: The core class that initializes with NBBO and runs auctions on input order slices.

## Methodology: The Matching Algorithm
The run_auction method executes a point-in-time auction as follows:

- Input Validation: Ensures all buy orders are marked as Buy and sell orders as Sell, panicking on mismatches to maintain data integrity.
- Order Prioritization:
  - Buy orders are sorted in descending price order (most aggressive first).
  - Sell orders are sorted in ascending price order (most aggressive first).
- Greedy Pairwise Matching:
  - Iterate over all buy-sell pairs using indices for mutable access.
  - Skip pairs with zero remaining quantity.
  - Match if buy_price >= sell_price, buy_price <= nbbo.ask, and sell_price >= nbbo.bid.
  - Fill the minimum available quantity and compute a midpoint clearing price: (buy_price + sell_price) / 2.0.
  - Immediately update remaining quantities to reflect partial fills, preventing over-allocation.
- Output: Return a vector of Trade instances representing executed matches.

This approach yields local optima by favoring high-improvement pairs early, though it differs from global combinatorial auctions (e.g., those solving the winner determination problem via integer programming). In the provided example, it achieves fills with shared price benefits, demonstrating reduced market impact in a controlled simulation.

## Usage

### Prerequisites
Rust (stable toolchain recommended).

### Running the Example
Clone the repository and execute:

```bash
cargo run --bin smart-auction
```

Sample output:

```text
Matched Trades:
Trade: Buy Order 1 <> Sell Order 3, Quantity: 100, Clearing Price: $99.80
Trade: Buy Order 2 <> Sell Order 3, Quantity: 50, Clearing Price: $99.68
```

This simulates matching with predefined orders under an NBBO of bid $99.50 and ask $100.50.

### Integration
Instantiate the engine and invoke run_auction with custom order vectors:

```rust
let nbbo = NBBO { bid: 99.50, ask: 100.50 };
let engine = AuctionEngine::new(nbbo);
let trades = engine.run_auction(&buy_orders, &sell_orders);
```

## Limitations and Future Directions
While effective for small-scale demonstrations, the greedy algorithm may yield suboptimal results in scenarios with inter-order dependencies (e.g., portfolio trades).
Future enhancements could include:

- Integration of optimization solvers (e.g., via Rust crates like good_lp) for combinatorial auctions.
- Support for multi-security matching and expressive constraints (e.g., indifference curves or hedging ratios).
- Asynchronous execution to simulate high-frequency (~100ms) cycles.
- Error handling beyond panics for production resilience.
- This model underscores the potential of algorithmic trading innovations to enhance liquidity and fairness, paving the way for more sophisticated Smart Market implementations.

## Matched Trades:
Trade: Buy Order 1 <> Sell Order 3, Quantity: 100, Clearing Price: $99.80
Trade: Buy Order 2 <> Sell Order 3, Quantity: 50, Clearing Price: $99.68
This simulates matching with predefined orders under an NBBO of bid $99.50 and ask $100.50.

## Extended Combinatorial Auction Example
For a more complex auction scenario, see `src/main-ca.rs`, which implements a combinatorial auction engine. This version extends the basic model to handle multiple securities and more complex order matching strategies, allowing for richer simulations of market dynamics.

## Example Combinatorial Auction
To run the combinatorial auction example, execute:
```bash
cargo run --bin combinatorial-auction
```