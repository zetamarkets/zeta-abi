<div align="center">
  <img height="120px" src="./logo.png" />

  <h1 style="margin-top: 0px">Zeta ABI 🧬</h1>

  <p>
    <a href="https://discord.gg/dD7YREfBkR"
      ><img
        alt="Discord Chat"
        src="https://img.shields.io/discord/841556000632078378?color=blueviolet"
    /></a>
    <a href="https://opensource.org/licenses/Apache-2.0"
      ><img
        alt="License"
        src="https://img.shields.io/github/license/project-serum/anchor?color=blueviolet"
    /></a>
  </p>
</div>

# Zeta ABI

ABI is Zeta's cross-program integration ecosystem.

This repository contains the Zeta Cross Program Invocation (CPI) interface as well as a usage example `abi-wrapper`.

```toml
# Add to dependencies
zeta-abi = "0.1.1"
```

## Cross Program Invocations

### Dependencies

- anchor 0.26.0
- solana 0.14.x

### Instructions

The instructions currently supported are as follows:

- `initialize_margin_account` - create and initialize a user's margin account
- `initialize_open_orders` - create and initialize user's open orders account
- `deposit` - deposit USDC collateral into the margin account
- `withdraw` - withdraw USDC collateral from the margin account
- `place_order_v4` - place a futures order of (price, size, side) on the relevant market
- `place_perp_order_v2` - place a perp order of (price, size, side) on the relevant market
- `cancel_order_xxx` - collection of order cancellation functions, by orderId, clientOrderId, market, etc
- `close_open_orders` - close open orders account
- `liquidate` - trigger liquidation

### Accounts

The accounts and relevant data that is currently supported (non-exhaustive):

- `ZetaGroup` - contains information relating to all derivatives market for an underlying
  - Underlying
  - Serum Market
  - Strike
  - Kind (Call, Put, Future)
  - Expiry
- `Greeks`
  - Mark Price
  - Delta
  - Vega
  - IV
- `MarginAccount`
  - Balance
  - Positions
  - helper functions: get_initial_margin(), get_maintenance_margin(), get_unrealized_pnl()

## Programs

### abi-wrapper

Basic usage example outlined in a dummy wrapper program, which simply calls the main zeta program instructions. Also includes account layouts and outlines how to read all relevant on-chain data from the Zeta program.
This should give all the boilerplate needed to execute core program functionality both as the smart contract and off-chain client.
