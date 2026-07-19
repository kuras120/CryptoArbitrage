# Jupiter Integration Domain

This document defines the language, responsibilities, and invariants of the Jupiter integration on Solana.

## Purpose

Provide access to Jupiter-routed Solana liquidity, quote metadata, and eventually transaction execution while isolating Jupiter and Solana contracts from the arbitrage policy.

## Ubiquitous Language

- **Mint**: Solana address identifying a fungible token.
- **Token decimals**: scale used to convert human-readable token quantities into integer base units.
- **Input amount**: exact token quantity offered to a route, expressed in base units.
- **Output amount**: quantity expected from the selected route, expressed in base units.
- **Other amount threshold**: minimum acceptable output or maximum acceptable input after slippage policy.
- **Quote**: time-sensitive route proposal for a specific mint pair, amount, and swap mode.
- **Route plan**: one or more AMM steps and allocation percentages selected by Jupiter.
- **Price impact**: estimated effect of the swap on execution price.
- **Slippage basis points**: allowed deviation between quoted and executed amounts.
- **Priority fee**: Solana fee used to improve transaction scheduling.
- **Blockhash**: recent Solana value limiting transaction validity.
- **Signature**: identifier of a submitted Solana transaction.
- **Confirmation state**: processed, confirmed, finalized, failed, expired, or unknown transaction outcome.

## Responsibilities

- Validate mint addresses, token decimals, amount, and swap direction.
- Request and deserialize Jupiter quotes.
- Expose route fees, price impact, thresholds, and quote timing as typed values.
- Build, simulate, sign, submit, and confirm swap transactions when execution is implemented.
- Convert Jupiter, RPC, and Solana failures into typed integration results.
- Preserve request IDs, signatures, and confirmation state required for reconciliation.

## Boundary

The integration does not decide whether a cross-venue spread is profitable, choose inventory targets, coordinate the Binance leg, or define portfolio-level risk. It provides validated quote and execution information to those higher-level policies.

## Invariants

- Token amounts must use integer base units at API and transaction boundaries.
- Mint decimals must come from trusted configuration or on-chain metadata and be validated before conversion.
- Quotes must include a local receipt time and be rejected after the configured maximum age.
- Route fees, network fees, price impact, and slippage threshold must remain visible to opportunity evaluation.
- Wallet keys and signed transactions must never be committed or logged.
- Transactions must use a fresh blockhash and pass simulation before live submission.
- A timed-out submission must remain unknown until signature and balance reconciliation complete.
- Retrying quote requests must be separated from retrying transaction submission.

## Outstanding Decisions

- Confirm the Jupiter API flow and authentication model used for execution.
- Define trusted token metadata and decimal resolution.
- Define quote freshness, slippage, price-impact, and route-selection limits.
- Define Solana RPC provider, commitment level, priority-fee policy, and confirmation timeout.
- Define transaction reconciliation and recovery after an unknown result.
