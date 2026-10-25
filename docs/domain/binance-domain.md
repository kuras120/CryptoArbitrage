# Binance Integration Domain

Owns Binance Spot authentication, transport, account state, depth, symbol constraints, and orders. Profitability, inventory targets, and cross-venue coordination belong to the [arbitrage domain](arbitrage-domain.md).

Current support is a hard-coded Spot Testnet runner plus unused query/cancel helpers. See [architecture and limitations](../architecture/README.md) and the [sequence](../architecture/binance_spot_flow.puml). The constraints below describe the domain contract; the prototype does not enforce all of them.

| Term | Meaning |
| --- | --- |
| Symbol | Market identifier, e.g. `BTCUSDT`; base quantity priced in quote units |
| Bid / ask | Level willing to buy / sell the base asset |
| Depth snapshot | Bid/ask levels associated with an exchange update ID |
| Free / locked balance | Available inventory / inventory reserved by orders |
| Exchange filter | Tick size, quantity step, minimum quantity/notional constraint |
| LIMIT FOK | Limit order that fills completely and immediately or expires without a fill |
| Client order ID | Caller-controlled correlation key for submission and reconciliation |
| Unknown order state | Outcome requiring reconciliation before retry or replacement |

| Domain constraint | Meaning |
| --- | --- |
| Precision | Decimal prices/quantities conform to symbol filters; quantity fits free balance and risk limits |
| Snapshot validity | Preserve exchange update ID and receipt time; reject malformed/incomplete or stale data |
| Side effects | Stable client IDs, typed outcomes, and reconciliation; transport failure after submission leaves state unknown |
| Error semantics | Keep rate limits, exchange rejection codes, parsing, and validation failures distinguishable |
| Security | External credentials, redacted logs, explicit separation of testnet/production |
| Adapter boundary | Convert payloads into validated results; preserve order/request IDs, status, quantities, and timing |

Open symbol, refresh, model, and recovery choices are tracked in [INBOX](../../INBOX.md).
