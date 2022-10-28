use crate::*;

use solana_program::pubkey::Pubkey;

pub const STATE_SEED: &[u8] = b"state";
pub const GREEKS_SEED: &[u8] = b"greeks";
pub const MARKET_NODE_SEED: &[u8] = b"market-node";
pub const OPEN_ORDERS_SEED: &[u8] = b"open-orders";
pub const VAULT_SEED: &[u8] = b"vault";
pub const SERUM_VAULT_SEED: &[u8] = b"serum-vault";
pub const ZETA_VAULT_SEED: &[u8] = b"zeta-vault";
pub const ZETA_GROUP_SEED: &[u8] = b"zeta-group";
pub const WHITELIST_DEPOSIT_SEED: &[u8] = b"whitelist-deposit";
pub const ZETA_INSURANCE_VAULT_SEED: &[u8] = b"zeta-insurance-vault";
pub const ZETA_TREASURY_WALLET_SEED: &[u8] = b"zeta-treasury-wallet";
pub const ZETA_REFERRALS_REWARDS_WALLET_SEED: &[u8] = b"zeta-referrals-rewards-wallet";
pub const WHITELIST_INSURANCE_SEED: &[u8] = b"whitelist-insurance";
pub const USER_INSURANCE_DEPOSIT_SEED: &[u8] = b"user-insurance-deposit";
pub const WHITELIST_TRADING_FEES_SEED: &[u8] = b"whitelist-trading-fees";
pub const SETTLEMENT_SEED: &[u8] = b"settlement";
pub const MARGIN_SEED: &[u8] = b"margin";
pub const SPREAD_SEED: &[u8] = b"spread";
pub const UNDERLYING_SEED: &[u8] = b"underlying";
pub const SERUM_SEED: &[u8] = b"serum";
pub const MINT_AUTH_SEED: &[u8] = b"mint-auth";
pub const BASE_MINT_SEED: &[u8] = b"base-mint";
pub const QUOTE_MINT_SEED: &[u8] = b"quote-mint";
pub const MARKET_SEED: &[u8] = b"market";
pub const MARKET_INDEXES_SEED: &[u8] = b"market-indexes";
pub const SOCIALIZED_LOSS_SEED: &[u8] = b"socialized-loss";
pub const REFERRER_SEED: &[u8] = b"referrer";
pub const REFERRAL_SEED: &[u8] = b"referral";
pub const REFERRER_ALIAS_SEED: &[u8] = b"referrer-alias";
pub const PERP_SYNC_QUEUE_SEED: &[u8] = b"perp-sync-queue";

pub fn get_state_address() -> Pubkey {
    Pubkey::find_program_address(&[STATE_SEED], &ID).0
}

pub fn get_mint_authority_address() -> Pubkey {
    Pubkey::find_program_address(&[MINT_AUTH_SEED], &ID).0
}

pub fn get_serum_authority_address() -> Pubkey {
    Pubkey::find_program_address(&[SERUM_SEED], &ID).0
}

pub fn get_zeta_group_address(asset: Asset) -> Pubkey {
    let underlying_mint = asset.to_underlying_mint();
    Pubkey::find_program_address(&[ZETA_GROUP_SEED, underlying_mint.as_ref()], &ID).0
}

pub fn get_greeks_address(zeta_group_address: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[GREEKS_SEED, zeta_group_address.as_ref()], &ID).0
}

pub fn get_perp_sync_queue_address(zeta_group_address: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PERP_SYNC_QUEUE_SEED, zeta_group_address.as_ref()], &ID).0
}

pub fn get_vault_address(zeta_group_address: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[VAULT_SEED, zeta_group_address.as_ref()], &ID).0
}

pub fn get_socialized_loss_account_address(zeta_group_address: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[SOCIALIZED_LOSS_SEED, zeta_group_address.as_ref()], &ID).0
}
