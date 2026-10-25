# Arbitrage Domain

The intended strategy keeps inventory on Binance Spot and Solana, buys on the cheaper venue, and sells the same base quantity on the more expensive venue. The coordinator is not implemented; see [current architecture](../architecture/README.md).

| Term | Meaning |
| --- | --- |
| Base / quote asset | Asset traded / unit used to price it |
| Venue | Exchange or liquidity source; Binance is a CEX, Jupiter routes Solana DEX liquidity |
| Inventory | Available and reserved balances per venue and asset |
| Executable price | Quantity-weighted price including depth, fees, slippage, and network costs |
| Buy / sell leg | Acquire the base asset on the cheaper venue / dispose of the same base quantity on the more expensive venue |
| Opportunity | Opposing buy/sell legs whose expected net value exceeds the configured threshold |
| Spread / net spread | Normalized sell minus buy price / spread after costs and safety margin |
| Inventory drift | Venue balances diverging through trading or partial execution |
| Rebalancing | Restoring inventory targets separately from opportunity execution |
| Partial execution | Legs do not reach matching filled quantities; explicit recovery is needed |

## Intended boundaries

| Owner | Responsibility |
| --- | --- |
| Market data | Normalize validated depth/quotes and enforce freshness |
| Opportunity evaluation | Own fee, slippage, threshold, and quote-age policy; compare the same base asset, equivalent quote value, direction, and executable quantity. Never submit orders. |
| Inventory | Check free/reserved balances, fundability, and drift |
| Execution | Explicit state machine for non-atomic legs; own client identifiers, safe retry policy, fills, recovery, and final order/balance reconciliation |
| Risk and operations | Position/notional/loss/slippage/frequency limits, kill switch, health, alerts, execution mode |
| [Binance integration](binance-domain.md) | Binance contracts, account state, filters, orders, and failures |
| [Jupiter integration](jupiter-domain.md) | Solana units, routes, transactions, and failures |

## Domain constraints

- Cap the common executable quantity at the smaller venue capacity after liquidity, balances, and risk limits. Quantity selection within that cap remains undecided.
- Include fees, slippage, network cost, precision/rounding, and safety margin in net spread.
- Reject stale/incomplete data and unknown inventory, credentials, limits, or venue health.
- Never assume atomic execution across venues; record partial/unknown outcomes and reconcile.
- Keep transfers outside the normal opportunity path and production/testnet configuration distinct.
- Keep arbitrage policy independent of transport payloads and signing details. Use distinct venue-specific and normalized types; make base/quote roles explicit.
- Validate inventory, venue filters, limits, and freshness immediately before execution; reconcile order state and balances after every attempt, including unknown outcomes.

Unresolved strategy and implementation choices are tracked in [INBOX](../../INBOX.md).
