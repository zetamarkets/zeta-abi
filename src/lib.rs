#![doc = include_str!("../README.md")]

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::pubkey;

pub mod account;
pub mod constants;
pub mod context;
pub mod errors;
pub mod id;
pub mod utils;

pub use crate::account::*;
pub use crate::constants::*;
pub use crate::context::*;
pub use crate::errors::*;
pub use crate::id::*;
pub use crate::utils::*;

#[program]
mod zeta_abi {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(clippy::too_many_arguments)]

    use super::*;

    pub(crate) fn initialize_cross_margin_account_manager(
        ctx: Context<InitializeCrossMarginAccountManager>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn initialize_cross_margin_account(
        ctx: Context<InitializeCrossMarginAccount>,
        subaccount_index: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn initialize_open_orders_v3(
        ctx: Context<InitializeOpenOrdersV3>,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn deposit_v2(ctx: Context<DepositV2>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn withdraw_v2(ctx: Context<WithdrawV2>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn place_perp_order_v3(
        ctx: Context<PlacePerpOrderV3>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>, // Not stored, only used when sniffing the transactions
        tif_offset: Option<u16>,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order(
        ctx: Context<CancelOrder>,
        side: Side,
        order_id: u128,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order_no_error(
        ctx: Context<CancelOrder>,
        side: Side,
        order_id: u128,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_all_market_orders(ctx: Context<CancelOrder>, asset: Asset) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order_by_client_order_id(
        ctx: Context<CancelOrder>,
        client_order_id: u64,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order_by_client_order_id_no_error(
        ctx: Context<CancelOrder>,
        client_order_id: u64,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn force_cancel_orders_v2(
        ctx: Context<ForceCancelOrdersV2>,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn liquidate_v2(ctx: Context<LiquidateV2>, size: u64, asset: Asset) -> Result<()> {
        Ok(())
    }

    pub(crate) fn close_open_orders_v3(
        ctx: Context<CloseOpenOrdersV3>,
        _map_nonce: u8,
        asset: Asset,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn close_cross_margin_account_manager(
        ctx: Context<CloseCrossMarginAccountManager>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn close_cross_margin_account(
        ctx: Context<CloseCrossMarginAccount>,
        subaccount_index: u8,
    ) -> Result<()> {
        Ok(())
    }
}
