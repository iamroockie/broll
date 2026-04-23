# Broll

A Monte Carlo poker bankroll simulator.

## Features

- Limit transitions
- Cashout strategies: fixed amount, profit percentage, bankroll percentage
- Probability calculations: bust, reach limit, profit/bankroll target

## Usage

### CLI

Reads configuration from [input.toml](/input.toml):

```
Usage: cli [OPTIONS]

Options:
  -s, --seed <SEED>                        Optional seed for reproducibility
  -n, --num-simulations <NUM_SIMULATIONS>  Number of simulations [default: 30000]
  -h, --help                               Print help
```

### HTTP Server

**Endpoint:** `POST /simulate?seed=<optional>`  
Request body — JSON with the same structure as [input.toml](/input.toml).

```
Usage: server [OPTIONS]

Options:
  -p, --port <PORT>                        Server port [default: 3000]
  -n, --num-simulations <NUM_SIMULATIONS>  Number of simulations [default: 30000]
  -h, --help                               Print help
```

## Configuration

Example [input.toml](/input.toml):

```toml
hands = 1_000_000

[bankroll]
started = 1_000
busted = 0

[[limits]]
name = "L10"
ev = 8.5
std_dev = 80
move_up = 1000

[[limits]]
name = "L25"
ev = 8
std_dev = 90
move_up = 2000
move_down = 875

[[limits]]
name = "L50"
ev = 7.5
std_dev = 100
move_down = 1500

[cashout]
interval = 50_000
kind.type = "profit_percentage"
kind.percentage = 0.5

[[probabilities]]
type = "reach_limit"
limit_name = "L25"

[[probabilities]]
type = "profit"
amount = 20_000
```

### Parameters

| Field                | Description                                                                       |
| -------------------- | --------------------------------------------------------------------------------- |
| `hands`              | Number of hands to simulate. Multiple of 100                                      |
| `bankroll.started`   | Starting bankroll (USD)                                                           |
| `bankroll.busted`    | Bust threshold (USD)                                                              |
| `limits`             | Array of limits to be included in the simulation. At least one limit is required  |
| `limits[].name`      | One of [LimitName](src/core/models/limit.rs#L80)                                  |
| `limits[].ev`        | Expected value (BB/100)                                                           |
| `limits[].std_dev`   | Standard deviation (BB/100)                                                       |
| `limits[].move_up`   | Bankroll required to move up to the next limit (USD). None for top limit          |
| `limits[].move_down` | Bankroll required to move down to the previous limit (USD). None for bottom limit |
| `cashout`            | Cashout rule. Optional                                                            |
| `cashout.interval`   | Cashout frequency (Hands). Multiple of 100                                        |
| `cashout.kind`       | One of [CashoutKind](src/core/models/cashout.rs#L39)                              |
| `probabilities`      | Array of probabilities that need to be calculated. Optional                       |
| `probabilities[]`    | One of [ProbabilityInput](src/core/models/probability.rs#L5)                      |

## Output

The result contains:

- `elapsed_seconds` — execution time
- `num_simulations` — number of simulations
- `probabilities` — array of results for the specified probabilities
- `ci` — confidence intervals for p70, p85, p99 (bankroll, cashout, profit)
