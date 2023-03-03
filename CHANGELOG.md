# Changelog

All notable changes to this project will be documented in this file. Version changes are pinned to SDK releases.

## Unreleased

- program: cleanup, removal of unused dependencies

## [0.1.1] 22-02-2023

- dependencies: pin to solana 1.14.16 since issues building with other versions of CLI.

## [0.1.0] 22-02-2023

- program: Add support for new DEX APIs: place_order_v4, place_perp_order_v2, close_open_orders, close_margin_account ([#2](https://github.com/zetamarkets/zeta-abi/pull/2))
- program: add APT asset
- program: MarginAccount helper functions: get_initial_margin(), get_maintenance_margin(), get_unrealized_pnl()
- abi-wrapper: introducing sample program that utilizes ABI CPIs
- program: First version with support for initialization of margin and open orders accounts, deposit, withdraw, order placement and cancellation
