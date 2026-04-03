# rust-maphf-sandbox
random bits of rust w/ mathematics bent


## Statistical Whatnots
We will use market data generation as a case study to illustrate the application of various statistical concepts 
in the context of high-frequency trading (HFT) backtesting. The generated data will be structured to meet the requirements 
of the `hftbacktest` engine, which expects a specific format for tick data.

- [Hipster Ipsum](https://hipsum.co/) some tick data ([market-data-source](https://github.com/destenson/market-data-source))
    - Chain in stress events (flash crash) and volatility wobbles
    - Format the data according to `hftbacktest`'s schema (flat, normalized version of CME Core Message Specifications (MDP 3.0))
- Replay ([hftbacktest](https://github.com/nkaz001/hftbacktest))
    - hftbacktest's built-in metrics and visualization tools?
- Kitchen sink statisitcal analysis
    - Start with descriptive, then move to inferential, and finally predictive statistics



### Market Data Generation (MES)
Synthetic tick data generator for Micro E-mini S&P 500 (MES) using the `market-data-source` crate, compatible with `hftbacktest`.

#### MES Contract Specifications
| Parameter | Value | Description |
|---|---|---|
| Tick Size | $0.25$ | Minimum price increment |
| Contract Multiplier | $\$5$ | Value of one full point |
| Lot Size | $1$ | Minimum trading unit (1 contract) |
| Format | .npz | Required binary format for hftbacktest |

#### hftbacktest .npz Schema (8-Column Structured Array)
Each row in the `.npz` files follows this exact schema to ensure compatibility with high-frequency backtesting engines.

| Column | Type | Description |
|---|---|---|
| `ev` | `u64` | Event flags (Bitmask: Type + Side + Validity) |
| `exch_ts` | `i64` | Exchange timestamp in nanoseconds |
| `local_ts` | `i64` | Local timestamp in nanoseconds (simulated 1ms latency) |
| `px` | `f64` | Price scaled by 100 (e.g., $5000.25 \to 500025.0$) |
| `qty` | `f64` | Trade quantity |
| `order_id` | `u64` | Order ID (for L3 feeds, defaulted to 0) |
| `ival` | `i64` | Reserved integer value |
| `fval` | `f64` | Reserved float value |

#### Generated Files
Located in `output/data/`:
- **`mes_chained_reproducible.npz`**: A composite market scenario consisting of five distinct segments:
  1. **Morning Bull Run** (Bullish, low vol)
  2. **Noon Consolidation** (Sideways, very low vol)
  3. **Flash Crash** (Bearish, high vol)
  4. **V-Shape Recovery** (Bullish, high vol)
  5. **Afternoon Sell-off** (Bearish, moderate vol)
- **Reproducibility**: Entire sequences can be recreated by providing the same `base_seed`, ensuring consistent testing across multiple runs.
