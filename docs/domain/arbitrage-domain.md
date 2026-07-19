# Arbitrage Domain

This document defines the product language, boundaries, and invariants of cross-venue arbitrage.

## Mission

Capture executable price differences between Binance Spot and Jupiter while keeping inventory funded on both venues. A successful cycle buys an asset on the cheaper venue and sells the same quantity on the more expensive venue, increasing total portfolio value without transferring assets for each opportunity.

## Ubiquitous Language

- **Base asset**: the asset being bought or sold, such as SOL or BTC.
- **Quote asset**: the unit used to price the base asset, such as USDC or USDT.
- **Venue**: an exchange or liquidity source. Binance is a CEX; Jupiter routes across Solana DEX liquidity.
- **Inventory**: available base and quote balances held on each venue.
- **Executable price**: quantity-weighted price after depth, exchange/route fees, slippage, and network cost.
- **Opportunity**: a pair of opposing venue actions whose expected net value exceeds configured thresholds.
- **Buy leg**: acquisition of the base asset on the cheaper venue.
- **Sell leg**: disposal of the same base quantity on the more expensive venue.
- **Spread**: difference between normalized sell and buy prices.
- **Net spread**: spread after all known costs and safety margins.
- **Inventory drift**: divergence of per-venue balances caused by repeated trading or partial execution.
- **Rebalancing**: deliberate restoration of target venue inventories; it is separate from opportunity execution.
- **Partial execution**: one leg fills fully or partly while the other does not reach the matching state.

## Core Invariants

- Compare the same base asset, equivalent quote value, direction, and executable quantity.
- Use the smaller executable quantity after venue liquidity, balance, and risk constraints.
- Evaluate net spread after fees, slippage, network cost, precision rules, and a configured safety margin.
- Reject stale or incomplete market data.
- Never assume both legs succeed atomically.
- Record and recover partial execution explicitly.
- Do not transfer assets as part of the normal opportunity path.
- Do not execute when inventory, limits, credentials, or venue health are unknown.
- Keep production and testnet configuration separate and visible.

## Integration Domains

- **Binance Integration** owns Binance Spot terminology, account state, symbol filters, order lifecycle, and exchange-specific failures. See [`binance-domain.md`](binance-domain.md).
- **Jupiter Integration** owns Solana token amounts, Jupiter routes, transaction lifecycle, and integration-specific failures. See [`jupiter-domain.md`](jupiter-domain.md).
- The arbitrage domain consumes normalized information from both integrations and must not depend on their transport payloads or signing details.

## Bounded Contexts

### Market Data

Consumes validated Binance depth and Jupiter quotes, normalizes them into a common price and quantity model, and enforces freshness requirements.

### Opportunity Evaluation

Compares executable buy and sell prices for a common quantity. It owns fee, slippage, threshold, and quote-age policy but does not submit orders.

### Inventory

Tracks available and reserved balances per venue and asset. It answers whether an opportunity is fundable and exposes drift from configured targets.

### Execution

Coordinates both legs as an explicit state machine. It owns client identifiers, retries, fill tracking, recovery, and final reconciliation.

### Risk And Operations

Owns maximum position/notional, loss and slippage limits, kill switch, venue health, alerts, and execution mode.

## Outstanding Domain Decisions

- Select the first common asset and quote-value normalization strategy across Binance and Jupiter.
- Define fee sources and whether stablecoin conversion risk (USDT versus USDC) is included.
- Define maximum quote age, minimum net spread, and quantity-selection policy.
- Define leg ordering, simultaneous execution strategy, and partial-fill recovery.
- Define inventory targets and rebalancing policy.
- Define persistence and restart recovery for in-flight opportunities.
- Define risk limits, kill-switch ownership, and production-readiness criteria.
