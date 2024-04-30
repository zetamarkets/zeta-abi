use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use zeta_abi::*;

declare_id!("z6ikFPqPGi16hCFfytMBP2Ao2iNGaN57mBQHvFGAit7");

#[program]
pub mod abi_wrapper {

    use super::*;

    pub fn initialize_cross_margin_account_manager(
        ctx: Context<InitializeCrossMarginAccountManagerAccounts>,
    ) -> Result<()> {
        msg!("In abi_wrapper::initialize_cross_margin_account_manager");
        let init_cross_margin_account_manager_accs =
            zeta_abi::cpi::accounts::InitializeCrossMarginAccountManager {
                cross_margin_account_manager: ctx
                    .accounts
                    .cross_margin_account_manager
                    .to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                zeta_program: ctx.accounts.zeta_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            };
        let init_cross_margin_account_manager_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            init_cross_margin_account_manager_accs,
        );
        zeta_abi::cpi::initialize_cross_margin_account_manager(
            init_cross_margin_account_manager_ctx,
        )?;

        Ok(())
    }

    pub fn initialize_cross_margin_account(
        ctx: Context<InitializeCrossMarginAccounts>,
        subaccount_index: u8,
    ) -> Result<()> {
        msg!("In abi_wrapper::initialize_cross_margin_account");
        let init_cross_margin_account_accs =
            zeta_abi::cpi::accounts::InitializeCrossMarginAccount {
                cross_margin_account: ctx.accounts.cross_margin_account.to_account_info(),
                cross_margin_account_manager: ctx
                    .accounts
                    .cross_margin_account_manager
                    .to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                zeta_program: ctx.accounts.zeta_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            };
        let init_cross_margin_account_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            init_cross_margin_account_accs,
        );
        zeta_abi::cpi::initialize_cross_margin_account(
            init_cross_margin_account_ctx,
            subaccount_index,
        )?;

        Ok(())
    }

    pub fn initialize_open_orders_v3(
        ctx: Context<InitializeOpenOrdersAccounts>,
        asset: Asset,
    ) -> Result<()> {
        let init_open_orders_accs = zeta_abi::cpi::accounts::InitializeOpenOrdersV3 {
            state: ctx.accounts.state.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            cross_margin_account: ctx.accounts.cross_margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            serum_authority: ctx.accounts.serum_authority.to_account_info(),
            open_orders_map: ctx.accounts.open_orders_map.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let init_open_orders_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            init_open_orders_accs,
        );
        zeta_abi::cpi::initialize_open_orders_v3(init_open_orders_ctx, asset.into())?;
        Ok(())
    }

    pub fn deposit_v2(ctx: Context<DepositAccounts>, amount: u64) -> Result<()> {
        let deposit_accs = zeta_abi::cpi::accounts::DepositV2 {
            margin_account: ctx.accounts.margin_account.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
            user_token_account: ctx.accounts.user_token_account.to_account_info(),
            socialized_loss_account: ctx.accounts.socialized_loss_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            state: ctx.accounts.state.to_account_info(),
            pricing: ctx.accounts.pricing.to_account_info(),
        };
        let deposit_ctx =
            CpiContext::new(ctx.accounts.zeta_program.to_account_info(), deposit_accs);
        zeta_abi::cpi::deposit_v2(deposit_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw_v2(ctx: Context<WithdrawAccounts>, amount: u64) -> Result<()> {
        let withdraw_accs = zeta_abi::cpi::accounts::WithdrawV2 {
            state: ctx.accounts.state.to_account_info(),
            pricing: ctx.accounts.pricing.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            user_token_account: ctx.accounts.user_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            socialized_loss_account: ctx.accounts.socialized_loss_account.to_account_info(),
        };
        let withdraw_ctx =
            CpiContext::new(ctx.accounts.zeta_program.to_account_info(), withdraw_accs);
        zeta_abi::cpi::withdraw_v2(withdraw_ctx, amount)?;
        Ok(())
    }

    pub fn place_perp_order_v3(
        ctx: Context<PlacePerpOrderAccounts>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>,
        asset: Asset,
    ) -> Result<()> {
        let place_perp_order_accs = zeta_abi::cpi::accounts::PlacePerpOrderV3 {
            state: ctx.accounts.state.to_account_info(),
            pricing: ctx.accounts.pricing.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            serum_authority: ctx.accounts.serum_authority.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            oracle: ctx.accounts.oracle.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
            market_mint: ctx.accounts.market_mint.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
            market_accounts: market_accs!(ctx),
            perp_sync_queue: ctx.accounts.perp_sync_queue.to_account_info(),
        };
        let place_perp_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            place_perp_order_accs,
        );
        zeta_abi::cpi::place_perp_order_v3(
            place_perp_order_ctx,
            price,
            size,
            side.into(),
            order_type.into(),
            client_order_id,
            tag,
            tif_offset,
            asset.into(),
        )?;
        Ok(())
    }

    pub fn cancel_order(
        ctx: Context<CancelOrderAccounts>,
        side: Side,
        order_id: u128,
        asset: Asset,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order(cancel_order_ctx, side.into(), order_id, asset.into())?;
        Ok(())
    }

    pub fn cancel_order_no_error(
        ctx: Context<CancelOrderAccounts>,
        side: Side,
        order_id: u128,
        asset: Asset,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order_no_error(
            cancel_order_ctx,
            side.into(),
            order_id,
            asset.into(),
        )?;
        Ok(())
    }

    pub fn cancel_all_market_orders(ctx: Context<CancelOrderAccounts>, asset: Asset) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_all_market_orders(cancel_order_ctx, asset.into())?;
        Ok(())
    }

    pub fn cancel_order_by_client_order_id(
        ctx: Context<CancelOrderAccounts>,
        client_order_id: u64,
        asset: Asset,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order_by_client_order_id(
            cancel_order_ctx,
            client_order_id,
            asset.into(),
        )?;
        Ok(())
    }

    pub fn cancel_order_by_client_order_id_no_error(
        ctx: Context<CancelOrderAccounts>,
        client_order_id: u64,
        asset: Asset,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order_by_client_order_id_no_error(
            cancel_order_ctx,
            client_order_id,
            asset.into(),
        )?;
        Ok(())
    }

    pub fn force_cancel_orders_v2(
        ctx: Context<ForceCancelOrdersAccounts>,
        asset: Asset,
    ) -> Result<()> {
        let force_cancel_order_accs = zeta_abi::cpi::accounts::ForceCancelOrdersV2 {
            pricing: ctx.accounts.pricing.to_account_info(),
            oracle: ctx.accounts.oracle.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
            cancel_accounts: cancel_accs!(ctx.accounts.cancel_accounts),
        };
        let force_cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            force_cancel_order_accs,
        );
        zeta_abi::cpi::force_cancel_orders_v2(force_cancel_order_ctx, asset.into())?;
        Ok(())
    }

    pub fn close_cross_margin(
        ctx: Context<CloseCrossMarginAccounts>,
        subaccount_index: u8,
    ) -> Result<()> {
        let close_margin_account_accs = zeta_abi::cpi::accounts::CloseCrossMarginAccount {
            cross_margin_account: ctx.accounts.cross_margin_account.to_account_info(),
            cross_margin_account_manager: ctx
                .accounts
                .cross_margin_account_manager
                .to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let close_margin_account_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            close_margin_account_accs,
        );
        zeta_abi::cpi::close_cross_margin_account(close_margin_account_ctx, subaccount_index)?;
        Ok(())
    }

    pub fn close_cross_margin_manager(
        ctx: Context<CloseCrossMarginAccountManagerAccounts>,
    ) -> Result<()> {
        let close_margin_account_manager_accs =
            zeta_abi::cpi::accounts::CloseCrossMarginAccountManager {
                cross_margin_account_manager: ctx
                    .accounts
                    .cross_margin_account_manager
                    .to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
        let close_margin_account_manager_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            close_margin_account_manager_accs,
        );
        zeta_abi::cpi::close_cross_margin_account_manager(close_margin_account_manager_ctx)?;
        Ok(())
    }

    pub fn close_open_orders_v3(
        ctx: Context<CloseOpenOrdersAccounts>,
        map_nonce: u8,
        asset: Asset,
    ) -> Result<()> {
        let close_open_orders_accs = zeta_abi::cpi::accounts::CloseOpenOrdersV3 {
            state: ctx.accounts.state.to_account_info(),
            pricing: ctx.accounts.pricing.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            cross_margin_account: ctx.accounts.cross_margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            serum_authority: ctx.accounts.serum_authority.to_account_info(),
            open_orders_map: ctx.accounts.open_orders_map.to_account_info(),
        };
        let close_open_orders_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            close_open_orders_accs,
        );
        zeta_abi::cpi::close_open_orders_v3(close_open_orders_ctx, map_nonce, asset.into())?;
        Ok(())
    }

    pub fn liquidate_v2(ctx: Context<LiquidateAccounts>, amount: u64, asset: Asset) -> Result<()> {
        let liquidate_accs = zeta_abi::cpi::accounts::LiquidateV2 {
            state: ctx.accounts.state.to_account_info(),
            liquidator: ctx.accounts.liquidator.to_account_info(),
            liquidator_account: ctx.accounts.liquidator_account.to_account_info(),
            pricing: ctx.accounts.pricing.to_account_info(),
            oracle: ctx.accounts.oracle.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            liquidated_account: ctx.accounts.liquidated_account.to_account_info(),
        };
        let liquidate_ctx =
            CpiContext::new(ctx.accounts.zeta_program.to_account_info(), liquidate_accs);
        zeta_abi::cpi::liquidate_v2(liquidate_ctx, amount, asset.into())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccountManagerAccounts<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub zeta_program: Program<'info, id::ZetaProgram>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccounts<'info> {
    #[account(mut)]
    pub cross_margin_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub cross_margin_account_manager: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub zeta_program: Program<'info, id::ZetaProgram>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersAccounts<'info> {
    pub state: UncheckedAccount<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub cross_margin_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub payer: Signer<'info>,
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    pub serum_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub open_orders_map: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct DepositAccounts<'info> {
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>, // Either CrossMarginAccount or MarginAccount
    #[account(mut)]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub socialized_loss_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub pricing: AccountLoader<'info, Pricing>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub pricing: AccountLoader<'info, Pricing>,
    #[account(mut)]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>, // Either CrossMarginAccount or MarginAccount
    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub socialized_loss_account: UncheckedAccount<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    pub pricing: AccountLoader<'info, Pricing>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>, // Either CrossMarginAccount or MarginAccount
    pub authority: Signer<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub token_program: Program<'info, Token>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub market_accounts: MarketAccounts<'info>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    #[account(mut)]
    pub market_mint: Box<Account<'info, Mint>>,
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub perp_sync_queue: AccountLoader<'info, PerpSyncQueue>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct CancelOrderAccounts<'info> {
    pub authority: Signer<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct ForceCancelOrdersAccounts<'info> {
    pub pricing: AccountLoader<'info, Pricing>,
    pub oracle: UncheckedAccount<'info>,
    pub oracle_backup_feed: UncheckedAccount<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccountManagerAccounts<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: AccountLoader<'info, CrossMarginAccountManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccounts<'info> {
    #[account(mut)]
    pub cross_margin_account: AccountLoader<'info, CrossMarginAccount>,
    #[account(mut)]
    pub cross_margin_account_manager: AccountLoader<'info, CrossMarginAccountManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    pub pricing: AccountLoader<'info, Pricing>,
    pub dex_program: Program<'info, id::Dex>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account: AccountLoader<'info, CrossMarginAccount>,
    // Marked mutable since it is receiving lamports
    #[account(mut)]
    pub authority: Signer<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: Box<Account<'info, CrossOpenOrdersMap>>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct LiquidateAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    pub liquidator: Signer<'info>,
    #[account(mut)]
    pub liquidator_account: UncheckedAccount<'info>, // Either CrossMarginAccount or MarginAccount
    pub pricing: AccountLoader<'info, Pricing>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub liquidated_account: UncheckedAccount<'info>, // Either CrossMarginAccount or MarginAccount
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts, Clone)]
pub struct MarketAccounts<'info> {
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    #[account(mut)]
    pub request_queue: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_queue: UncheckedAccount<'info>,
    #[account(mut)]
    pub bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub order_payer_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub coin_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pc_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub coin_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pc_wallet: Box<Account<'info, TokenAccount>>,
}

#[derive(Accounts)]
pub struct CancelAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
}

#[macro_export]
macro_rules! cancel_order_accs {
    ($accs:expr) => {
        zeta_abi::cpi::accounts::CancelOrder {
            authority: $accs.authority.to_account_info(),
            cancel_accounts: cancel_accs!($accs.cancel_accounts),
        }
    };
}

#[macro_export]
macro_rules! cancel_accs {
    ($cancel_accs:expr) => {
        zeta_abi::cpi::accounts::CancelAccounts {
            state: $cancel_accs.state.to_account_info(),
            margin_account: $cancel_accs.margin_account.to_account_info(),
            dex_program: $cancel_accs.dex_program.to_account_info(),
            serum_authority: $cancel_accs.serum_authority.to_account_info(),
            open_orders: $cancel_accs.open_orders.to_account_info(),
            market: $cancel_accs.market.to_account_info(),
            bids: $cancel_accs.bids.to_account_info(),
            asks: $cancel_accs.asks.to_account_info(),
            event_queue: $cancel_accs.event_queue.to_account_info(),
        }
    };
}

#[macro_export]
macro_rules! market_accs {
    ($ctx:expr) => {
        zeta_abi::cpi::accounts::MarketAccounts {
            market: $ctx.accounts.market_accounts.market.to_account_info(),
            request_queue: $ctx
                .accounts
                .market_accounts
                .request_queue
                .to_account_info(),
            event_queue: $ctx.accounts.market_accounts.event_queue.to_account_info(),
            bids: $ctx.accounts.market_accounts.bids.to_account_info(),
            asks: $ctx.accounts.market_accounts.asks.to_account_info(),
            order_payer_token_account: $ctx
                .accounts
                .market_accounts
                .order_payer_token_account
                .to_account_info(),
            coin_vault: $ctx.accounts.market_accounts.coin_vault.to_account_info(),
            pc_vault: $ctx.accounts.market_accounts.pc_vault.to_account_info(),
            coin_wallet: $ctx.accounts.market_accounts.coin_wallet.to_account_info(),
            pc_wallet: $ctx.accounts.market_accounts.pc_wallet.to_account_info(),
        }
    };
}

// Needed for IDL
#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Uninitialized = 0,
    Bid = 1,
    Ask = 2,
}

impl From<Side> for zeta_abi::Side {
    fn from(value: Side) -> zeta_abi::Side {
        match value {
            Side::Uninitialized => zeta_abi::Side::Uninitialized,
            Side::Bid => zeta_abi::Side::Bid,
            Side::Ask => zeta_abi::Side::Ask,
        }
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum Asset {
    SOL = 0,
    BTC = 1,
    ETH = 2,
    APT = 3,
    ARB = 4,
    UNDEFINED = 255,
}

impl From<Asset> for zeta_abi::Asset {
    fn from(value: Asset) -> zeta_abi::Asset {
        match value {
            Asset::SOL => zeta_abi::Asset::SOL,
            Asset::BTC => zeta_abi::Asset::BTC,
            Asset::ETH => zeta_abi::Asset::ETH,
            Asset::APT => zeta_abi::Asset::APT,
            Asset::ARB => zeta_abi::Asset::ARB,
            Asset::UNDEFINED => zeta_abi::Asset::UNDEFINED,
        }
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum OrderType {
    Limit = 0,
    PostOnly = 1,
    FillOrKill = 2,
    ImmediateOrCancel = 3,
    PostOnlySlide = 4,
}

impl From<OrderType> for zeta_abi::OrderType {
    fn from(value: OrderType) -> zeta_abi::OrderType {
        match value {
            OrderType::Limit => zeta_abi::OrderType::Limit,
            OrderType::PostOnly => zeta_abi::OrderType::PostOnly,
            OrderType::FillOrKill => zeta_abi::OrderType::FillOrKill,
            OrderType::ImmediateOrCancel => zeta_abi::OrderType::ImmediateOrCancel,
            OrderType::PostOnlySlide => zeta_abi::OrderType::PostOnlySlide,
        }
    }
}
