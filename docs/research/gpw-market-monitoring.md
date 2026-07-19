# GPW Market Monitoring Data Providers

## Research Status

- Outcome: Yahoo Finance is the only practical no-account, zero-subscription-cost
  candidate found for a personal intraday spike. No reviewed free plan supports
  reliable one-minute polling of the entire GPW instrument universe.
- Investigated: 2026-07-18; alternatives reviewed: 2026-07-19
- Implementation status: not implemented

## Goal

Evaluate an extension of the repository beyond cryptocurrency arbitrage to
monitor equities and indices listed on the Warsaw Stock Exchange (GPW). The
initial scope is observation and alerting only, without order submission or a
real-time data guarantee.

The acceptable freshness target is a documented delay of approximately 15 to 20
minutes during a GPW session. End-of-day-only sources are useful for backfills
and daily reports, but do not satisfy intraday monitoring.

The market-data source must have no recurring monetary cost. During continuous
trading, the monitor should check for newly published delayed observations once
per minute for its priority universe. A 15-minute delay is acceptable when each
successive delayed minute becomes available; a provider that publishes only one
daily close is not.

## Findings

`yfinance` is a practical source for an MVP and private analysis. Yahoo Finance
covers Warsaw-listed instruments. Equity symbols commonly use the `.WA` suffix,
for example `PKO.WA`, `PEO.WA`, `PZU.WA`, `KGH.WA`, `ALE.WA`, and `PKN.WA`.
Indices are available under symbols such as `WIG20.WA` and `WIG.WA`. Yahoo marks
Warsaw quotes as delayed and reports them in PLN. Yahoo's exchange coverage
documentation specifies a 15-minute delay for `.WA` instruments, so the expected
latency is within the monitoring requirement.

The source must not be treated as an execution dependency. `yfinance` is an
unofficial client for Yahoo's public interfaces. Its documentation describes it
as a research and educational tool and notes that Yahoo Finance data is intended
for personal use. GPW requires a market-data licence for certain company use of
real-time non-display data and for redistribution of delayed data.

## Provider Comparison

| Provider | GPW freshness | Interface | Main advantages | Main limitations | Assessment |
| --- | --- | --- | --- | --- | --- |
| Yahoo Finance through `yfinance` | 15-minute exchange delay | Unofficial Python client | Free, easiest spike, historical and intraday candles, corporate actions | Unsupported public API, personal-use and no-redistribution restrictions, upstream changes can break the client | Best zero-cost spike |
| EODHD | 15-20-minute delayed snapshots, refreshed approximately once per minute | Documented REST API with JSON or CSV and API token | Explicit global delayed-data product, batch requests, timestamps, OHLCV, historical intraday and EOD data, documented GPW (`WAR`) coverage | Free plan is limited to 20 calls per day and limited EOD history; useful delayed monitoring starts with a paid plan | Reject under the zero-cost requirement |
| Interactive Brokers | WSE delayed data is documented as 15 minutes; API documentation describes delayed data generally as 15-20 minutes | TWS API or Web API, including streaming snapshots and historical bars | Broker-supported API, delayed bid, ask, last, sizes, volume, OHLC and timestamps | Requires an IBKR Pro account, market-data acknowledgement, authenticated TWS or IB Gateway session, and usually a funded account; 100 market-data lines are allocated by default | Conditional fallback, not a no-account free API |
| Stooq | Public GPW pages appear approximately 15 minutes delayed | Website and commonly used CSV download URLs, but no supported public API was found | Free, strong Polish-market coverage, convenient historical CSV, visible XWAR symbols | Programmatic endpoints are undocumented and may change; terms and redistribution rights require review; weak operational guarantees | Useful manual cross-check or emergency backfill, not a primary adapter |
| DM BOŚ / bossaAPI | Yellow package quotes are delayed 15 minutes with a stated 120-second refresh interval; full intraday access is tied to higher packages | FIXML-style API through the bossaNOL3 desktop application | Free Yellow account package and broker-sourced GPW data | Requires a brokerage account and bossaNOL3 on supported Windows; two-minute refresh misses the preferred cadence; API also exposes order functions | Poor fit for a headless one-minute monitor |
| Twelve Data | XWAR is listed as EOD | Documented REST API | Clean API, symbol discovery, fundamentals, exchange calendar and corporate actions | Current exchange page classifies GPW core pricing as EOD, so it misses the intraday freshness target | Reject for intraday; possible EOD metadata source |
| Direct GPW feed or licensed GPW vendor | Delayed GPW product is 15 minutes | Contracted feed or vendor-specific API | Authoritative data and explicit usage rights | Licence agreement, annual fees, integration work, and vendor negotiation are disproportionate for a personal MVP | Revisit only for commercial or execution-sensitive use |

Google Finance also documents a 15-minute WSE delay, but it does not provide a
supported general-purpose market-data API. It can serve as a manual comparison,
not as an application dependency. No sufficiently clear current GPW coverage was
confirmed for Polygon, Tiingo, or Financial Modeling Prep, so they should not
enter a spike without first obtaining a provider-confirmed XWAR symbol list,
latency, free-plan entitlement, and licensing terms. Alpha Vantage's free plan
allows only 25 requests per day, Marketstack allows 100 requests per month, and
Finnhub's free quote product covers US rather than international real-time data;
none meets this monitoring requirement.

## Free-Tier Capacity

A GPW continuous-trading session lasts approximately 470 minutes. One logical
poll per minute therefore means about 470 polling cycles per session.

`yf.download()` accepts a list of symbols, but its implementation creates a
`Ticker` and calls `history()` separately for every symbol. Threads reduce wall
time; they do not turn the operation into one upstream request. Approximate daily
request pressure is therefore:

| Universe | Symbols | One-minute polling cycles | Approximate ticker fetches per session |
| --- | ---: | ---: | ---: |
| Small priority watchlist | 10 | 470 | 4,700 |
| WIG20 | 20 | 470 | 9,400 |
| WIG20 plus mWIG40 | 60 | 470 | 28,200 |
| Broad GPW universe | approximately 600 | 470 | approximately 282,000 |

Yahoo does not publish a supported API quota for this use. `yfinance` explicitly
handles rate-limit errors, and its issue history contains reports of HTTP 429
responses during bulk collection. Consequently, a full-market one-minute poll
cannot be called reliable or sustainable. Even WIG20 polling must be validated
empirically and use backoff rather than assuming unlimited access.

The `yfinance` WebSocket client could avoid repeated polling by subscribing once,
but GPW symbol delivery, delayed-feed behavior, and symbol-count limits are not
documented. It is the first zero-cost experiment to run, not a confirmed design.

## Recommendation

Use a zero-cost, tiered evaluation:

1. Test `yfinance` WebSocket delivery for 5, then 20, then 60 GPW symbols. Record
   whether new delayed observations arrive every minute and whether connections
   remain stable for a complete session.
2. If WebSocket delivery is unavailable, poll only a 10-symbol watchlist every
   minute for five sessions. Increase to WIG20 only if no rate limiting occurs.
3. Refresh a broader secondary universe every 5 or 15 minutes, with jitter, and
   collect the complete market only after the session. Never retry all symbols
   immediately after a rate-limit response.
4. Compare timestamps and values with Yahoo's website plus one independent manual
   source such as Stooq, Google Finance, or a broker view.

Under a strict zero-cost rule, EODHD is no longer a candidate. IBKR is a fallback
only if opening and funding a broker account is independently acceptable. Do not
add a provider abstraction for Stooq scraping: undocumented endpoints would
reproduce the main operational risk of `yfinance` without a comparable maintained
client.

The provider decision must remain separate from signal logic. Store the declared
provider delay and measured observation age with every sample. An observation is
stale when its source timestamp is older than `declared_delay + 5 minutes` during
a continuous-trading session. The monitor should report staleness rather than
silently reusing the last value.

## Candidate MVP Capabilities

- Maintain an explicit, configuration-driven watchlist.
- Collect OHLCV candles and daily percentage changes.
- Track the move from open, daily high and low, turnover, and volume.
- Detect moving-average crosses, RSI conditions, gaps, and N-session breakouts.
- Detect unusual volume relative to a 20-session median.
- Compare relative strength with `WIG20.WA` or `WIG.WA`.
- Check the priority universe every minute and lower-priority tiers every 5 or
  15 minutes, subject to measured provider limits.
- Collect basic corporate actions exposed by Yahoo, such as dividends and
  splits, while treating this data as potentially incomplete.

## Suggested Boundaries

1. Define a `MarketDataProvider` interface so the alerting logic does not depend
   directly on Yahoo, EODHD, or a broker API.
2. Start with a `YahooFinanceProvider` for the learning spike, preferring a
   validated subscription connection over per-symbol polling. It must validate
   timestamps and empty responses, apply bounded backoff, and enforce per-tier
   request budgets.
3. Keep symbols, benchmark, interval, and enabled rules in configuration such as
   `markets/gpw.yaml`. Index composition changes must not require a code change.
4. Start with SQLite and an idempotent `(symbol, interval, timestamp)` key. Store
   source timestamps, ingestion timestamps, and raw candles.
5. Keep signal calculation independent of the data provider.
6. Schedule work in `Europe/Warsaw` and account for weekends, exchange holidays,
   and daylight-saving transitions.
7. Keep alert delivery behind a separate adapter. A log or report is sufficient
   initially; email, webhooks, or messaging integrations can follow.

## Minimal Subscription Experiment

```python
import yfinance as yf

symbols = ["PKO.WA", "PEO.WA", "PZU.WA", "KGH.WA", "ALE.WA", "WIG20.WA"]

def record(message: dict) -> None:
    print(message)

with yf.WebSocket(verbose=False) as ws:
    ws.subscribe(symbols)
    ws.listen(record)
```

This experiment must run during a GPW session and record both the provider event
timestamp and local receipt timestamp. Lack of messages is a valid negative
result: the existence of a WebSocket client neither guarantees delivery for GPW
symbols nor grants product usage rights. A fallback polling experiment may use
`yf.download(..., period="1d", interval="1m")`, initially for no more than 10
symbols.

The experiment is intentionally Python-based even though the production project
uses Rust. It should remain an isolated learning tool. A production implementation
would require an approved project proposal and a provider boundary appropriate
for the Rust application.

## Risks And Data Quality Controls

- **Delay and availability:** treat observations as monitoring data, not
  executable quotes. Expose `observed_at`, `fetched_at`, and data age.
- **Missing or changed symbols:** validate on startup that each symbol returns
  data for the expected venue and currency. ORLEN still appears under `PKN.WA`,
  so a company name cannot be converted mechanically into a Yahoo symbol.
- **Historical adjustments:** retain raw OHLC, adjusted values, and corporate
  actions. Set `auto_adjust` deliberately instead of relying on its default.
- **Provider instability:** cache results, use bounded retries, request small
  batches, and alert on stale data. Do not trigger an alert from one unverified
  anomalous sample.
- **Trading calendar:** do not assume every weekday is a GPW session.
- **Licensing:** reassess Yahoo and GPW terms before offering the system to other
  users or using automated signals as part of commercial activity.

## Recommended First Milestone

Create a read-only feed feasibility experiment with no signal calculation:

1. observe 5 GPW symbols through the Yahoo WebSocket for one complete session;
2. if successful, repeat with 20 and then 60 symbols; otherwise poll 10 symbols
   once per minute with bounded backoff;
3. store each distinct observation idempotently with source and receipt times;
4. report connection gaps, new observations per symbol and minute, source delay,
   duplicate rate, HTTP 429 responses, and stale-data periods;
5. distinguish an unchanged quote from a transport failure and from a symbol that
   did not trade during the observation window;
6. validate a sample manually against Yahoo Finance and one independent display.

After five sessions, decide which universe can be monitored reliably at each
cadence without cost. If even the 10-symbol minute tier experiences sustained
rate limiting or stale data, the one-minute requirement and zero-cost requirement
cannot both be met with the reviewed sources.

## Sources

- [yfinance documentation and legal disclaimer](https://ranaroussi.github.io/yfinance/)
- [yfinance.download parameters and intervals](https://ranaroussi.github.io/yfinance/reference/api/yfinance.download.html)
- [yfinance WebSocket documentation](https://ranaroussi.github.io/yfinance/reference/yfinance.websocket.html)
- [yfinance multi-symbol downloader implementation](https://github.com/ranaroussi/yfinance/blob/main/yfinance/multi.py)
- [yfinance rate-limit error history](https://github.com/ranaroussi/yfinance/issues/2128)
- [Yahoo Finance exchange coverage and documented WSE delay](https://help.yahoo.com/kb/finance/article-exchanges-data-delays-sln2310.html)
- [Yahoo Finance example for PEO.WA](https://uk.finance.yahoo.com/quote/PEO.WA/)
- [Yahoo Finance example for WIG20.WA](https://uk.finance.yahoo.com/quote/WIG20.WA/)
- [EODHD delayed quote API](https://eodhd.com/financial-apis/live-ohlcv-stocks-api)
- [EODHD free-plan limit](https://eodhd.com/)
- [EODHD Warsaw exchange coverage](https://eodhd.com/exchange/WAR)
- [EODHD data-source description](https://eodhd.com/financial-apis/our-data-sources-and-data-partners)
- [IBKR delayed market-data API behavior](https://www.interactivebrokers.com/campus/ibkr-api-page/twsapi-doc/)
- [IBKR account and market-data-line requirements](https://www.interactivebrokers.com/campus/ibkr-api-page/market-data-subscriptions/)
- [IBKR WSE delay and subscription table](https://www.interactivebrokers.com/en/pricing/market-data-pricing.php?p=mktDataPricing)
- [DM BOŚ bossaAPI overview](https://bossa.pl/oferta/narzedzia/bossaapi)
- [Twelve Data XWAR coverage](https://twelvedata.com/exchanges/xwar?group=core)
- [Stooq example for a GPW instrument](https://stooq.pl/q/a2/?s=peo)
- [Google Finance exchange-delay table](https://www.google.co.uk/intl/en-GB_GB/googlefinance/disclaimer/)
- [Alpha Vantage free API limit](https://www.alphavantage.co/premium/)
- [Marketstack free API limit](https://marketstack.com/pricing/)
- [Finnhub free-plan international market-data coverage](https://finnhub.io/pricing)
- [GPW market-data licensing overview](https://www.gpw.pl/market-data)
