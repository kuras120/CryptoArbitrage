# Binance Integration Domain

This document defines the language, responsibilities, and invariants of the Binance Spot integration.

## Purpose

Provide authenticated access to Binance Spot market data, account state, exchange constraints, and order execution while isolating Binance-specific contracts from the arbitrage policy.

## Ubiquitous Language

- **Symbol**: Binance identifier for a market, such as `SOLUSDT` or `BTCUSDT`.
- **Base asset**: asset whose quantity is bought or sold.
- **Quote asset**: asset used to express the price and notional value.
- **Bid**: order-book level at which the market is willing to buy the base asset.
- **Ask**: order-book level at which the market is willing to sell the base asset.
- **Depth snapshot**: ordered bid and ask levels associated with an exchange update identifier.
- **Free balance**: account inventory available for a new order.
- **Locked balance**: account inventory reserved by open orders.
- **Exchange filter**: symbol rule governing price tick, quantity step, minimum quantity, or minimum notional.
- **LIMIT FOK order**: limit order that must fill completely and immediately or be canceled.
- **Client order ID**: caller-controlled identifier used to correlate submission, status, and recovery.
- **Unknown order state**: submission outcome that cannot be classified as accepted, rejected, filled, or canceled without reconciliation.

## Responsibilities

- Authenticate signed Binance Spot requests.
- Retrieve account permissions and balances.
- Retrieve market metadata and order-book snapshots.
- Validate prices, quantities, and notionals against symbol filters.
- Submit, query, and cancel orders.
- Convert Binance payloads and errors into typed integration results.
- Preserve request and order identifiers required for reconciliation.

## Boundary

The integration does not decide whether an arbitrage opportunity is profitable, choose inventory targets, coordinate the Jupiter leg, or define portfolio-level risk. It supplies validated market, balance, and execution information to those higher-level policies.

## Invariants

- Testnet and production endpoints and credentials must be explicitly separated.
- Credentials must remain outside source code and logs.
- Prices and quantities must use decimal types and conform to current symbol filters.
- A depth snapshot must include its exchange update identifier and local receipt time.
- Available quantity must not exceed free balance or configured risk limits.
- State-changing requests must use stable client identifiers before retry behavior is introduced.
- Transport failure after submission must produce an unknown state until the order is reconciled.
- Rate limits and exchange error codes must remain distinguishable from parsing and validation failures.

## Outstanding Decisions

- Define the first common Binance symbol used by the arbitrage strategy.
- Define account and market-data refresh cadence.
- Define client order ID format and order reconciliation policy.
- Define behavior for partial fills, filter changes, rate limits, and unknown order state.
- Define the boundary between exchange balances and normalized portfolio inventory.
