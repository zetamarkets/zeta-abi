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

    pub(crate) fn initialize_margin_account(ctx: Context<InitializeMarginAccount>) -> Result<()> {
        Ok(())
    }

    pub(crate) fn initialize_open_orders(ctx: Context<InitializeOpenOrders>) -> Result<()> {
        Ok(())
    }

    pub(crate) fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn place_order_v4(
        ctx: Context<PlaceOrder>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>, // Not stored, only used when sniffing the transactions
        tif_offset: Option<u16>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn place_perp_order_v2(
        ctx: Context<PlacePerpOrder>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>, // Not stored, only used when sniffing the transactions
        tif_offset: Option<u16>,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order(
        ctx: Context<CancelOrder>,
        side: Side,
        order_id: u128,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order_no_error(
        ctx: Context<CancelOrder>,
        side: Side,
        order_id: u128,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_all_market_orders(ctx: Context<CancelOrder>) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order_by_client_order_id(
        ctx: Context<CancelOrder>,
        client_order_id: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_order_by_client_order_id_no_error(
        ctx: Context<CancelOrder>,
        client_order_id: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn force_cancel_orders(ctx: Context<ForceCancelOrders>) -> Result<()> {
        Ok(())
    }

    pub(crate) fn liquidate(ctx: Context<Liquidate>, size: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn close_open_orders(ctx: Context<CloseOpenOrders>, _map_nonce: u8) -> Result<()> {
        Ok(())
    }

    pub(crate) fn close_margin_account(_ctx: Context<CloseMarginAccount>) -> Result<()> {
        Ok(())
    }
}
