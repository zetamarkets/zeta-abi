use crate::*;

#[derive(Accounts)]
pub struct InitializeCrossMarginAccountManager<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub zeta_program: Program<'info, id::ZetaProgram>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseCrossMarginAccountManager<'info> {
    #[account(mut)]
    pub cross_margin_account_manager: AccountLoader<'info, CrossMarginAccountManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeCrossMarginAccount<'info> {
    #[account(mut)]
    pub cross_margin_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub cross_margin_account_manager: AccountLoader<'info, CrossMarginAccountManager>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub zeta_program: Program<'info, id::ZetaProgram>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(subaccount_index: u8)]
pub struct CloseCrossMarginAccount<'info> {
    #[account(mut)]
    pub cross_margin_account: AccountLoader<'info, CrossMarginAccount>,
    #[account(mut)]
    pub cross_margin_account_manager: AccountLoader<'info, CrossMarginAccountManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrdersV3<'info> {
    pub state: AccountLoader<'info, State>,
    pub dex_program: Program<'info, id::Dex>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub cross_margin_account: AccountLoader<'info, CrossMarginAccount>,
    pub authority: Signer<'info>,
    // Marked mutable since it pays
    #[account(mut)]
    pub payer: Signer<'info>,
    pub market: AccountInfo<'info>,
    pub serum_authority: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders_map: Box<Account<'info, CrossOpenOrdersMap>>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct PlacePerpOrderV3<'info> {
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
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositV2<'info> {
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
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawV2<'info> {
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
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    pub authority: Signer<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrdersV2<'info> {
    pub pricing: AccountLoader<'info, Pricing>,
    pub oracle: AccountInfo<'info>,
    pub oracle_backup_feed: AccountInfo<'info>,
    pub oracle_backup_program: AccountInfo<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
}

#[derive(Accounts)]
pub struct LiquidateV2<'info> {
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
    pub order_payer_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub coin_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub pc_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub coin_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub pc_wallet: UncheckedAccount<'info>,
}

// Shared accounts required for cancel order
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

#[derive(Accounts)]
#[instruction(map_nonce: u8)]
pub struct CloseOpenOrdersV3<'info> {
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
}
