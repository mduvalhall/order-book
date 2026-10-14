# order-book

A simple limit order book.

## API guideline

### Core

| API                                               | Purpose                                                     |
| ------------------------------------------------- | ----------------------------------------------------------- |
| `place(side, price, qty) -> id, fills, remaining` | Match against the opposite side, then rest whatever is left |
| `cancel(id) -> remaining`                         | Remove a resting order from the book                        |
| `get(id) -> remaining`                            | Check the current status of an order                        |
| `best_bid() -> price, qty`                        | Highest bid and the total quantity at that price            |
| `best_ask() -> price, qty`                        | Lowest ask and the total quantity at that price             |
| `spread() -> price`                               | Difference between best ask and best bid                    |
| `mid_price() -> price`                            | Midpoint between best bid and best ask                      |
| `depth(side, levels) -> [(price, qty)]`           | Aggregated top N price levels (L2 snapshot)                 |

### Useful additions

| API                                               | Purpose                                                                 |
| ------------------------------------------------- | ----------------------------------------------------------------------- |
| `modify(id, qty) -> remaining`                    | Reduce an order's quantity while keeping its queue position             |
| `replace(id, price, qty) -> id, fills, remaining` | Cancel and re-place at a new price; loses queue position                |
| `market(side, qty) -> fills`                      | Take liquidity until filled; anything unfilled is dropped, never rested |
| `volume_at(side, price) -> qty`                   | Total quantity resting at a single price level                          |
| `orders_at(side, price) -> [order]`               | Orders at a price level in time priority (handy for tests)              |
| `len() -> count`                                  | Number of resting orders                                                |
| `is_empty() -> bool`                              | Whether the book has no resting orders                                  |

### Time in force

Options for `place`:

| Option                    | Behaviour                                               |
| ------------------------- | ------------------------------------------------------- |
| GTC (good till cancelled) | Rest any unfilled quantity on the book                  |
| IOC (immediate or cancel) | Fill what's possible now, cancel the rest               |
| FOK (fill or kill)        | Fill the entire quantity immediately or do nothing      |
| Post-only                 | Reject if the order would cross the spread (maker only) |

### Events

Emitted by the book so the exchange can build market data, logging and replay.

| Event                              | When                                                       |
| ---------------------------------- | ---------------------------------------------------------- |
| `Accepted(id)`                     | Order is accepted onto the book                            |
| `Filled(maker, taker, price, qty)` | Two orders match; one event per fill                       |
| `Cancelled(id, remaining)`         | Order removed by cancel, IOC or market remainder           |
| `Rejected(id, reason)`             | Order refused (e.g. post-only would cross, FOK can't fill) |

### Matching rules

- **Price-time priority**: the best price matches first, and orders at the same price match in the order they arrived (FIFO).
- **Fill price** is the resting (maker) order's price, not the incoming order's.
- An emptied price level is removed from the book.
