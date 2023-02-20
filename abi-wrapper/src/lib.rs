use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use zeta_abi::*;

declare_id!("z6ikFPqPGi16hCFfytMBP2Ao2iNGaN57mBQHvFGAit7");

#[program]
pub mod abi_wrapper {

    use super::*;

    pub fn initialize_margin_account(ctx: Context<InitializeMarginAccounts>) -> Result<()> {
        msg!("In abi_wrapper::initialize_margin_account");
        let init_margin_accs = zeta_abi::cpi::accounts::InitializeMarginAccount {
            margin_account: ctx.accounts.margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            zeta_program: ctx.accounts.zeta_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
        };
        let init_margin_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            init_margin_accs,
        );
        zeta_abi::cpi::initialize_margin_account(init_margin_ctx)?;
        Ok(())
    }

    pub fn initialize_open_orders(ctx: Context<InitializeOpenOrdersAccounts>) -> Result<()> {
        let init_open_orders_accs = zeta_abi::cpi::accounts::InitializeOpenOrders {
            state: ctx.accounts.state.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
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
        zeta_abi::cpi::initialize_open_orders(init_open_orders_ctx)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<DepositAccounts>, amount: u64) -> Result<()> {
        let deposit_accs = zeta_abi::cpi::accounts::Deposit {
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
            user_token_account: ctx.accounts.user_token_account.to_account_info(),
            socialized_loss_account: ctx.accounts.socialized_loss_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            state: ctx.accounts.state.to_account_info(),
            greeks: ctx.accounts.greeks.to_account_info(),
        };
        let deposit_ctx =
            CpiContext::new(ctx.accounts.zeta_program.to_account_info(), deposit_accs);
        zeta_abi::cpi::deposit(deposit_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<WithdrawAccounts>, amount: u64) -> Result<()> {
        let withdraw_accs = zeta_abi::cpi::accounts::Withdraw {
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
            user_token_account: ctx.accounts.user_token_account.to_account_info(),
            socialized_loss_account: ctx.accounts.socialized_loss_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            state: ctx.accounts.state.to_account_info(),
            oracle: ctx.accounts.oracle.to_account_info(),
            greeks: ctx.accounts.greeks.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
        };
        let withdraw_ctx =
            CpiContext::new(ctx.accounts.zeta_program.to_account_info(), withdraw_accs);
        zeta_abi::cpi::withdraw(withdraw_ctx, amount)?;
        Ok(())
    }

    pub fn place_order_v4(
        ctx: Context<PlaceOrderAccounts>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>,
    ) -> Result<()> {
        let place_order_accs = zeta_abi::cpi::accounts::PlaceOrder {
            state: ctx.accounts.state.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            serum_authority: ctx.accounts.serum_authority.to_account_info(),
            greeks: ctx.accounts.greeks.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            market_accounts: market_accs!(ctx),
            oracle: ctx.accounts.oracle.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
            market_node: ctx.accounts.market_node.to_account_info(),
            market_mint: ctx.accounts.market_mint.to_account_info(),
            mint_authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let place_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            place_order_accs,
        );
        zeta_abi::cpi::place_order_v4(
            place_order_ctx,
            price,
            size,
            side.into(),
            order_type.into(),
            client_order_id,
            tag,
            tif_offset,
        )?;
        Ok(())
    }

    pub fn place_perp_order_v2(
        ctx: Context<PlacePerpOrderAccounts>,
        price: u64,
        size: u64,
        side: Side,
        order_type: OrderType,
        client_order_id: Option<u64>,
        tag: Option<String>,
        tif_offset: Option<u16>,
    ) -> Result<()> {
        let place_perp_order_accs = zeta_abi::cpi::accounts::PlacePerpOrder {
            state: ctx.accounts.state.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            serum_authority: ctx.accounts.serum_authority.to_account_info(),
            greeks: ctx.accounts.greeks.to_account_info(),
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
        zeta_abi::cpi::place_perp_order_v2(
            place_perp_order_ctx,
            price,
            size,
            side.into(),
            order_type.into(),
            client_order_id,
            tag,
            tif_offset,
        )?;
        Ok(())
    }

    pub fn cancel_order(
        ctx: Context<CancelOrderAccounts>,
        side: Side,
        order_id: u128,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order(cancel_order_ctx, side.into(), order_id)?;
        Ok(())
    }

    pub fn cancel_order_no_error(
        ctx: Context<CancelOrderAccounts>,
        side: Side,
        order_id: u128,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order_no_error(cancel_order_ctx, side.into(), order_id)?;
        Ok(())
    }

    pub fn cancel_all_market_orders(ctx: Context<CancelOrderAccounts>) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_all_market_orders(cancel_order_ctx)?;
        Ok(())
    }

    pub fn cancel_order_by_client_order_id(
        ctx: Context<CancelOrderAccounts>,
        client_order_id: u64,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order_by_client_order_id(cancel_order_ctx, client_order_id)?;
        Ok(())
    }

    pub fn cancel_order_by_client_order_id_no_error(
        ctx: Context<CancelOrderAccounts>,
        client_order_id: u64,
    ) -> Result<()> {
        let cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            cancel_order_accs!(ctx.accounts),
        );
        zeta_abi::cpi::cancel_order_by_client_order_id_no_error(cancel_order_ctx, client_order_id)?;
        Ok(())
    }

    pub fn force_cancel_orders(ctx: Context<ForceCancelOrdersAccounts>) -> Result<()> {
        let force_cancel_order_accs = zeta_abi::cpi::accounts::ForceCancelOrders {
            greeks: ctx.accounts.greeks.to_account_info(),
            oracle: ctx.accounts.oracle.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
            cancel_accounts: cancel_accs!(ctx.accounts.cancel_accounts),
        };
        let force_cancel_order_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            force_cancel_order_accs,
        );
        zeta_abi::cpi::force_cancel_orders(force_cancel_order_ctx)?;
        Ok(())
    }

    pub fn close_margin(ctx: Context<CloseMarginAccounts>) -> Result<()> {
        let close_margin_account_accs = zeta_abi::cpi::accounts::CloseMarginAccount {
            margin_account: ctx.accounts.margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
        };
        let close_margin_account_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            close_margin_account_accs,
        );
        zeta_abi::cpi::close_margin_account(close_margin_account_ctx)?;
        Ok(())
    }

    pub fn close_open_orders(ctx: Context<CloseOpenOrdersAccounts>, map_nonce: u8) -> Result<()> {
        let close_open_orders_accs = zeta_abi::cpi::accounts::CloseOpenOrders {
            state: ctx.accounts.state.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            dex_program: ctx.accounts.dex_program.to_account_info(),
            open_orders: ctx.accounts.open_orders.to_account_info(),
            margin_account: ctx.accounts.margin_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            serum_authority: ctx.accounts.serum_authority.to_account_info(),
            open_orders_map: ctx.accounts.open_orders_map.to_account_info(),
        };
        let close_open_orders_ctx = CpiContext::new(
            ctx.accounts.zeta_program.to_account_info(),
            close_open_orders_accs,
        );
        zeta_abi::cpi::close_open_orders(close_open_orders_ctx, map_nonce)?;
        Ok(())
    }

    pub fn liquidate(ctx: Context<LiquidateAccounts>, amount: u64) -> Result<()> {
        let liquidate_accs = zeta_abi::cpi::accounts::Liquidate {
            state: ctx.accounts.state.to_account_info(),
            liquidator: ctx.accounts.liquidator.to_account_info(),
            liquidator_margin_account: ctx.accounts.liquidator_margin_account.to_account_info(),
            greeks: ctx.accounts.greeks.to_account_info(),
            oracle: ctx.accounts.oracle.to_account_info(),
            oracle_backup_feed: ctx.accounts.oracle_backup_feed.to_account_info(),
            oracle_backup_program: ctx.accounts.oracle_backup_program.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            zeta_group: ctx.accounts.zeta_group.to_account_info(),
            liquidated_margin_account: ctx.accounts.liquidated_margin_account.to_account_info(),
        };
        let liquidate_ctx =
            CpiContext::new(ctx.accounts.zeta_program.to_account_info(), liquidate_accs);
        zeta_abi::cpi::liquidate(liquidate_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMarginAccounts<'info> {
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub payer: Signer<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
    pub system_program: Program<'info, System>,
    pub zeta_group: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersAccounts<'info> {
    pub state: UncheckedAccount<'info>,
    pub zeta_group: UncheckedAccount<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
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
    pub zeta_group: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub socialized_loss_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub state: UncheckedAccount<'info>,
    pub greeks: UncheckedAccount<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
    pub zeta_group: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub socialized_loss_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub state: UncheckedAccount<'info>,
    pub greeks: UncheckedAccount<'info>,
    #[account(mut)]
    pub oracle: UncheckedAccount<'info>,
    pub oracle_backup_feed: UncheckedAccount<'info>,
    pub oracle_backup_program: Program<'info, id::Chainlink>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct PlaceOrderAccounts<'info> {
    pub state: UncheckedAccount<'info>,
    pub zeta_group: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub token_program: Program<'info, Token>,
    pub serum_authority: UncheckedAccount<'info>,
    pub greeks: UncheckedAccount<'info>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    pub rent: UncheckedAccount<'info>,
    pub market_accounts: MarketAccounts<'info>,
    pub oracle: UncheckedAccount<'info>,
    pub oracle_backup_feed: UncheckedAccount<'info>,
    pub oracle_backup_program: Program<'info, id::Chainlink>,
    #[account(mut)]
    pub market_node: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_mint: Box<Account<'info, Mint>>,
    pub mint_authority: UncheckedAccount<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderAccounts<'info> {
    pub state: UncheckedAccount<'info>,
    pub zeta_group: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub token_program: Program<'info, Token>,
    pub serum_authority: UncheckedAccount<'info>,
    pub greeks: UncheckedAccount<'info>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    pub rent: UncheckedAccount<'info>,
    pub market_accounts: MarketAccounts<'info>,
    pub oracle: UncheckedAccount<'info>,
    pub oracle_backup_feed: UncheckedAccount<'info>,
    pub oracle_backup_program: Program<'info, id::Chainlink>,
    #[account(mut)]
    pub market_mint: Box<Account<'info, Mint>>,
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub perp_sync_queue: UncheckedAccount<'info>,
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
    pub greeks: UncheckedAccount<'info>,
    pub oracle: UncheckedAccount<'info>,
    pub oracle_backup_feed: UncheckedAccount<'info>,
    pub oracle_backup_program: Program<'info, id::Chainlink>,
    pub cancel_accounts: CancelAccounts<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct CloseMarginAccounts<'info> {
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub zeta_group: UncheckedAccount<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct CloseOpenOrdersAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    pub zeta_group: UncheckedAccount<'info>,
    pub dex_program: Program<'info, zeta_abi::id::Dex>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub market: UncheckedAccount<'info>,
    #[account(mut)]
    pub serum_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub open_orders_map: UncheckedAccount<'info>,
    pub zeta_program: Program<'info, zeta_abi::id::ZetaProgram>,
}

#[derive(Accounts)]
pub struct LiquidateAccounts<'info> {
    pub state: AccountLoader<'info, State>,
    pub liquidator: Signer<'info>,
    #[account(mut)]
    pub liquidator_margin_account: AccountLoader<'info, MarginAccount>,
    pub greeks: AccountLoader<'info, Greeks>,
    pub oracle: UncheckedAccount<'info>,
    pub oracle_backup_feed: UncheckedAccount<'info>,
    pub oracle_backup_program: Program<'info, id::Chainlink>,
    pub market: UncheckedAccount<'info>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    #[account(mut)]
    pub liquidated_margin_account: AccountLoader<'info, MarginAccount>,
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
    pub zeta_group: UncheckedAccount<'info>,
    pub state: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: AccountLoader<'info, MarginAccount>,
    pub dex_program: Program<'info, id::Dex>,
    pub serum_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    #[account(mut)]
    pub bids: UncheckedAccount<'info>,
    #[account(mut)]
    pub asks: UncheckedAccount<'info>,
    #[account(mut)]
    pub event_queue: UncheckedAccount<'info>,
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
            zeta_group: $cancel_accs.zeta_group.to_account_info(),
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
