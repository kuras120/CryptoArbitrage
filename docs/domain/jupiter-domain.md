# Jupiter Integration Domain

Owns Jupiter/Solana transport, mint and amount semantics, routes, and future transaction execution. Profitability, inventory targets, and cross-venue coordination belong to the [arbitrage domain](arbitrage-domain.md).

Current support is a quote-only Swap V1 prototype, disabled in main. See [architecture](../architecture/README.md) and the [quote workflow table](../architecture/README.md#jupiter-quote-workflow). Transaction responsibilities below are intended boundaries, not implemented support.

| Term | Meaning |
| --- | --- |
| Mint / token decimals | Token address / scale converting human amounts to integer base units |
| Input / output amount | Quantity offered / expected from the route, in base units |
| Other amount threshold | Minimum output or maximum input after slippage policy |
| Quote / route plan | Time-sensitive route for a mint pair, amount, and mode / AMM steps and allocation |
| Price impact / slippage bps | Estimated price effect / allowed execution deviation |
| Priority fee / blockhash | Scheduling fee / recent value limiting transaction validity |
| Signature / confirmation | Transaction identifier / processed, confirmed, finalized, failed, expired, or unknown outcome |

| Domain constraint | Meaning |
| --- | --- |
| Amount validation | Validate mint, amount, and direction. Resolve decimals from trusted configuration or on-chain metadata; convert at one boundary and test rounding. API/transaction amounts use integer base units. |
| Quote validity | Preserve receipt time; enforce maximum age; expose route/network fees, price impact, and thresholds |
| Future execution | Build, sign, simulate, submit, and confirm; transactions must use a fresh blockhash and pass simulation before live submission. Preserve signature, confirmation state, and request IDs. |
| Unknown outcomes | Reconcile signature and balances after timeout; distinguish quote retries from transaction retries |
| Security | Keep wallet keys and signed transactions outside Git and logs |
| Adapter boundary | Return typed quote/execution results and distinguish Jupiter, RPC, and Solana failures |

Open API, metadata, RPC, risk-limit, and recovery choices are tracked in [INBOX](../../INBOX.md).
