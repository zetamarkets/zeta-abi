use crate::*;

#[derive(Accounts)]
pub struct InitializeMarginAccount<'info> {
    #[account(mut)]
    pub margin_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub zeta_program: Program<'info, id::ZetaProgram>,
    pub system_program: Program<'info, System>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub state: AccountLoader<'info, State>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: AccountLoader<'info, MarginAccount>,
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
    pub greeks: AccountLoader<'info, Greeks>,
    pub oracle: UncheckedAccount<'info>,
    #[account(mut)]
    pub socialized_loss_account: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    #[account(mut)]
    pub margin_account: AccountLoader<'info, MarginAccount>,
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub socialized_loss_account: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub state: AccountLoader<'info, State>,
    pub greeks: AccountLoader<'info, Greeks>,
}

#[derive(Accounts)]
pub struct InitializeOpenOrders<'info> {
    pub state: AccountLoader<'info, State>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    pub dex_program: Program<'info, id::Dex>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    #[account(mut)]
    pub margin_account: AccountLoader<'info, MarginAccount>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub market: UncheckedAccount<'info>,
    pub serum_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub open_orders_map: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    pub state: AccountLoader<'info, State>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    #[account(mut)]
    pub margin_account: AccountLoader<'info, MarginAccount>,
    pub authority: Signer<'info>,
    pub dex_program: Program<'info, id::Dex>,
    pub token_program: Program<'info, Token>,
    pub serum_authority: UncheckedAccount<'info>,
    pub greeks: AccountLoader<'info, Greeks>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub market_accounts: MarketAccounts<'info>,
    pub oracle: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_node: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_mint: UncheckedAccount<'info>,
    pub mint_authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct PlacePerpOrder<'info> {
    pub state: AccountLoader<'info, State>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    #[account(mut)]
    pub margin_account: AccountLoader<'info, MarginAccount>,
    pub authority: Signer<'info>,
    // Programs.
    pub dex_program: Program<'info, id::Dex>,
    pub token_program: Program<'info, Token>,
    // Serum authority
    pub serum_authority: UncheckedAccount<'info>,
    pub greeks: AccountLoader<'info, Greeks>,
    #[account(mut)]
    pub open_orders: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub market_accounts: MarketAccounts<'info>,
    pub oracle: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_node: UncheckedAccount<'info>,
    #[account(mut)]
    pub market_mint: UncheckedAccount<'info>,
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub perp_sync_queue: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    pub authority: Signer<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
}

#[derive(Accounts)]
pub struct ForceCancelOrders<'info> {
    pub greeks: AccountLoader<'info, Greeks>,
    pub oracle: UncheckedAccount<'info>,
    pub cancel_accounts: CancelAccounts<'info>,
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    pub state: AccountLoader<'info, State>,
    pub liquidator: Signer<'info>,
    #[account(mut)]
    pub liquidator_margin_account: AccountLoader<'info, MarginAccount>,
    pub greeks: AccountLoader<'info, Greeks>,
    pub oracle: UncheckedAccount<'info>,
    pub market: UncheckedAccount<'info>,
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    #[account(mut)]
    pub liquidated_margin_account: AccountLoader<'info, MarginAccount>,
}

// Market accounts are the accounts used to place orders against the dex minus
// common accounts, i.e., program ids, sysvars, and the `pc_wallet`.
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
    // The `spl_token::Account` that funds will be taken from, i.e., transferred
    // from the user into the market's vault.
    //
    // For bids, this is the base currency. For asks, the quote.
    // This has to be owned by serum_authority PDA as serum checks that the owner
    // of open orders also owns this token account
    #[account(mut)]
    pub order_payer_token_account: UncheckedAccount<'info>,
    // Also known as the "base" currency. For a given A/B market,
    // this is the vault for the A mint.
    #[account(mut)]
    pub coin_vault: UncheckedAccount<'info>,
    // Also known as the "quote" currency. For a given A/B market,
    // this is the vault for the B mint.
    #[account(mut)]
    pub pc_vault: UncheckedAccount<'info>,
    // User wallets, used for settling.
    #[account(mut)]
    pub coin_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub pc_wallet: UncheckedAccount<'info>,
}

// Shared accounts required for cancel order
#[derive(Accounts)]
pub struct CancelAccounts<'info> {
    pub zeta_group: AccountLoader<'info, ZetaGroup>,
    pub state: AccountLoader<'info, State>,
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
    pub oracle: UncheckedAccount<'info>,
}
