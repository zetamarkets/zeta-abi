use crate::*;

use bytemuck::try_from_bytes;

#[cfg(target_endian = "little")]
unsafe impl Zeroable for MarketStateV2 {}

#[cfg(target_endian = "little")]
unsafe impl Pod for MarketStateV2 {}

#[derive(Copy, Clone)]
#[cfg_attr(target_endian = "little", derive(Debug))]
#[repr(packed)]
pub struct MarketStateV2 {
    _head_pad: [u8; 5],
    pub inner: MarketState,
    pub open_orders_authority: Pubkey,
    pub prune_authority: Pubkey,
    pub consume_events_authority: Pubkey,
    // Unused bytes for future upgrades.
    padding: [u8; 992],
    _tail_pad: [u8; 7],
}

impl Default for MarketStateV2 {
    fn default() -> MarketStateV2 {
        MarketStateV2 {
            _head_pad: [0; 5],
            inner: MarketState::default(),
            open_orders_authority: Pubkey::default(),
            prune_authority: Pubkey::default(),
            consume_events_authority: Pubkey::default(),
            padding: [0; 992],
            _tail_pad: [0; 7],
        }
    }
}

#[derive(Copy, Clone, Default)]
#[cfg_attr(target_endian = "little", derive(Debug))]
#[repr(packed)]
pub struct MarketState {
    // 0
    pub account_flags: u64, // Initialized, Market

    // 1
    pub own_address: [u64; 4],

    // 5
    pub vault_signer_nonce: u64,
    // 6
    pub coin_mint: [u64; 4],
    // 10
    pub pc_mint: [u64; 4],

    // 14
    pub coin_vault: [u64; 4],
    // 18
    pub coin_deposits_total: u64,
    // 19
    pub coin_fees_accrued: u64,

    // 20
    pub pc_vault: [u64; 4],
    // 24
    pub pc_deposits_total: u64,
    // 25
    pub pc_fees_accrued: u64,

    // 26
    pub pc_dust_threshold: u64,

    // 27
    pub req_q: [u64; 4],
    // 31
    pub event_q: [u64; 4],

    // 35
    pub bids: [u64; 4],
    // 39
    pub asks: [u64; 4],

    // 43
    pub coin_lot_size: u64,
    // 44
    pub pc_lot_size: u64,

    // 45
    pub fee_rate_bps: u64,
    // 46
    pub referrer_rebates_accrued: u64,
}

impl MarketStateV2 {
    pub fn deserialize(buf: &[u8]) -> Result<Self> {
        const FLAGS: u64 = (AccountFlag::Initialized as u64)
            | (AccountFlag::Market as u64)
            | (AccountFlag::Permissioned as u64);

        let r: &Self = bytemuck::try_from_bytes(buf).unwrap();

        if r._head_pad[..] != *"serum".as_bytes()
            || r._tail_pad[..] != *"padding".as_bytes()
            || r.inner.account_flags != FLAGS
        {
            panic!("Invalid buffer for market");
        }

        Ok(*r)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u64)]
pub enum AccountFlag {
    Initialized = 1u64 << 0,
    Market = 1u64 << 1,
    OpenOrders = 1u64 << 2,
    RequestQueue = 1u64 << 3,
    EventQueue = 1u64 << 4,
    Bids = 1u64 << 5,
    Asks = 1u64 << 6,
    Disabled = 1u64 << 7,
    Closed = 1u64 << 8,
    Permissioned = 1u64 << 9,
}
