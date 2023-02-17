#![allow(dead_code)]

use crate::*;

#[account(zero_copy)]
#[repr(packed)]
pub struct State {
    // Admin authority
    pub admin: Pubkey,                                   // 32
    pub state_nonce: u8,                                 // 1
    pub serum_nonce: u8,                                 // 1
    pub mint_auth_nonce: u8,                             // 1
    pub num_underlyings: u8,                             // 1
    pub _null: u64,                                      // 8
    pub strike_initialization_threshold_seconds: u32,    // 4
    pub pricing_frequency_seconds: u32,                  // 4
    pub liquidator_liquidation_percentage: u32,          // 4
    pub insurance_vault_liquidation_percentage: u32,     // 4
    pub native_d1_trade_fee_percentage: u64,             // 8
    pub native_d1_underlying_fee_percentage: u64,        // 8
    pub native_whitelist_underlying_fee_percentage: u64, // 8
    pub native_deposit_limit: u64,                       // 8
    pub expiration_threshold_seconds: u32,               // 4
    pub position_movement_fee_bps: u8,                   // 1
    pub margin_concession_percentage: u8,                // 1
    pub treasury_wallet_nonce: u8,                       // 1
    pub native_option_trade_fee_percentage: u64,         // 8
    pub native_option_underlying_fee_percentage: u64,    // 8
    pub referrals_admin: Pubkey,                         // 32
    pub referrals_rewards_wallet_nonce: u8,              // 1
    pub _padding: [u8; 107],                             // 107
} // 255

#[account(zero_copy)]
#[repr(packed)]
pub struct ZetaGroup {
    pub nonce: u8,                                // 1
    pub vault_nonce: u8,                          // 1
    pub insurance_vault_nonce: u8,                // 1
    pub front_expiry_index: u8,                   // 1
    pub halt_state: HaltState,                    // 167
    pub underlying_mint: Pubkey,                  // 32
    pub oracle: Pubkey,                           // 32
    pub greeks: Pubkey,                           // 32
    pub pricing_parameters: PricingParameters,    // 112
    pub margin_parameters: MarginParameters,      // 120
    pub products: [Product; 46],                  // 137 * 43 = 5891
    pub products_padding: [Product; 91],          //
    pub perp: Product,                            // 43
    pub expiry_series: [ExpirySeries; 2],         // 32 * 6 = 192
    pub expiry_series_padding: [ExpirySeries; 4], //
    pub total_insurance_vault_deposits: u64,      // 8
    pub asset: Asset,                             // 1
    pub expiry_interval_seconds: u32,             // 4
    pub new_expiry_threshold_seconds: u32,        // 4
    pub perp_parameters: PerpParameters,          // 24
    pub perp_sync_queue: Pubkey,                  // 32
    pub oracle_backup_feed: Pubkey,               // 32
    pub padding: [u8; 966],                       // 966
} // 7696

#[zero_copy]
#[repr(packed)]
pub struct Product {
    // Serum market
    pub market: Pubkey,
    pub strike: Strike,
    // Tracks whether the market has been wiped after expiration
    pub dirty: bool,
    pub kind: Kind,
} // 32 + 9 + 1 + 1 = 43 bytes

#[zero_copy]
#[repr(packed)]
pub struct Strike {
    is_set: bool,
    value: u64,
}

impl Strike {
    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn get_strike(&self) -> Result<u64> {
        if !self.is_set() {
            return Err(error!(ZetaError::ProductStrikeUninitialized));
        }
        Ok(self.value)
    }

    pub fn set(&mut self, strike: u64) -> Result<()> {
        // There shouldn't be a case where you set a strike without resetting it first.
        if self.is_set() {
            return Err(error!(ZetaError::CannotSetInitializedStrike));
        }
        self.is_set = true;
        self.value = strike;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        if !self.is_set() {
            return Err(error!(ZetaError::CannotResetUninitializedStrike));
        }
        self.value = 0;
        self.is_set = false;
        Ok(())
    }
}

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct PricingParameters {
    pub option_trade_normalizer: AnchorDecimal, // 16
    pub future_trade_normalizer: AnchorDecimal, // 16
    pub max_volatility_retreat: AnchorDecimal,  // 16
    pub max_interest_retreat: AnchorDecimal,    // 16
    pub max_delta: u64,                         // 8
    pub min_delta: u64,                         // 8
    pub min_volatility: u64,                    // 8
    pub max_volatility: u64,                    // 8
    pub min_interest_rate: i64,                 // 8
    pub max_interest_rate: i64,                 // 8
} // 112

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct MarginParameters {
    // Futures
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,

    // Options initial
    pub option_mark_percentage_long_initial: u64,
    pub option_spot_percentage_long_initial: u64,
    pub option_spot_percentage_short_initial: u64,
    pub option_dynamic_percentage_short_initial: u64,

    // Options maintenance
    pub option_mark_percentage_long_maintenance: u64,
    pub option_spot_percentage_long_maintenance: u64,
    pub option_spot_percentage_short_maintenance: u64,
    pub option_dynamic_percentage_short_maintenance: u64,

    // Other parameters
    pub option_short_put_cap_percentage: u64,
    pub padding: [u8; 32],
} // 120 bytes.

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct PerpParameters {
    pub min_funding_rate_percent: i64, // 8
    pub max_funding_rate_percent: i64, // 8
    pub impact_cash_delta: u64,        // 8
} // 24

#[zero_copy]
#[repr(packed)]
pub struct ExpirySeries {
    pub active_ts: u64,
    pub expiry_ts: u64,
    pub dirty: bool,
    pub padding: [u8; 15], // 32 - 17
} // 32

#[account(zero_copy)]
#[repr(packed)]
pub struct MarginAccount {
    pub authority: Pubkey,                             // 32
    pub nonce: u8,                                     // 1
    pub balance: u64,                                  // 8
    pub force_cancel_flag: bool,                       // 1
    pub open_orders_nonce: [u8; 138],                  // 138
    pub series_expiry: [u64; 5],                       // 48
    pub _series_expiry_padding: u64,                   //
    pub product_ledgers: [ProductLedger; 46],          // 138 * 40 = 5520
    pub _product_ledgers_padding: [ProductLedger; 91], //
    pub perp_product_ledger: ProductLedger,            //
    pub rebalance_amount: i64,                         // 8
    pub asset: Asset,                                  // 1
    pub account_type: MarginAccountType,               // 1
    pub last_funding_delta: AnchorDecimal,             // 16
    pub delegated_pubkey: Pubkey,                      // 32
    pub _padding: [u8; 338],                           // 338
} // 6144

impl MarginAccount {
    // Calculates the total initial margin for all open orders and positions.
    pub fn get_initial_margin(&self, greeks: &Greeks, zeta_group: &ZetaGroup, spot: u64) -> u64 {
        let initial_margin_requirement: u64 = self
            .product_ledgers
            .iter()
            .enumerate()
            .map(|(i, ledger)| {
                ledger.get_initial_margin(
                    greeks.mark_prices[i],
                    &zeta_group.products[i],
                    spot,
                    &zeta_group.margin_parameters,
                )
            })
            .sum();

        // Perps have a different layout
        let perp_margin_requirement: u64 = self.perp_product_ledger.get_initial_margin(
            spot,
            &zeta_group.perp,
            spot,
            &zeta_group.margin_parameters,
        );

        msg!(
            "get_initial_margin {} {}",
            initial_margin_requirement,
            perp_margin_requirement
        );

        initial_margin_requirement
            .checked_add(perp_margin_requirement)
            .unwrap()
    }

    pub fn get_maintenance_margin(
        &self,
        greeks: &Greeks,
        zeta_group: &ZetaGroup,
        spot: u64,
    ) -> u64 {
        let maintenance_margin_requirement: u64 = self
            .product_ledgers
            .iter()
            .enumerate()
            .map(|(i, product_ledgers)| {
                product_ledgers.get_maintenance_margin(
                    greeks.mark_prices[i],
                    &zeta_group.products[i],
                    spot,
                    &zeta_group.margin_parameters,
                )
            })
            .sum();

        // Perps have a different layout
        let perp_margin_requirement: u64 = self.perp_product_ledger.get_maintenance_margin(
            spot,
            &zeta_group.perp,
            spot,
            &zeta_group.margin_parameters,
        );

        maintenance_margin_requirement
            .checked_add(perp_margin_requirement)
            .unwrap()
    }

    pub fn get_unrealized_pnl(&self, greeks: &Greeks, spot: u64) -> i64 {
        let pnl: i64 = self
            .product_ledgers
            .iter()
            .enumerate()
            .map(|(i, product_ledger)| {
                (product_ledger
                    .position
                    .get_unrealized_pnl(greeks.mark_prices[i]) as i128) as i64
            })
            .sum();

        // Perps have a different layout
        let perp_pnl: i64 = self.perp_product_ledger.position.get_unrealized_pnl(spot);

        pnl.checked_add(perp_pnl).unwrap()
    }
}

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct ProductGreeks {
    pub delta: u64,
    pub vega: AnchorDecimal,
    pub volatility: AnchorDecimal,
} // 40

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct AnchorDecimal {
    pub flags: u32,
    pub hi: u32,
    pub lo: u32,
    pub mid: u32,
} // 16

#[account(zero_copy)]
#[repr(packed)]
pub struct Greeks {
    pub nonce: u8,                                       // 1
    pub mark_prices: [u64; 46],                          // 8 * 46 = 368
    pub _mark_prices_padding: [u64; 91],                 // 8 * 91 =  728
    pub perp_mark_price: u64,                            // 8
    pub product_greeks: [ProductGreeks; 22],             // 22 * 40 = 880
    pub _product_greeks_padding: [ProductGreeks; 44],    // 44 * 40 = 1760
    pub update_timestamp: [u64; 2],                      // 16
    pub _update_timestamp_padding: [u64; 4],             // 32
    pub retreat_expiration_timestamp: [u64; 2],          // 16
    pub _retreat_expiration_timestamp_padding: [u64; 4], // 32
    pub interest_rate: [i64; 2],                         // 16
    pub _interest_rate_padding: [i64; 4],                // 32
    pub nodes: [u64; 5],                                 // 40
    pub volatility: [u64; 10],                           // 80
    pub _volatility_padding: [u64; 20],                  // 160
    pub node_keys: [Pubkey; 138],                        // 138 * 32 = 4416
    pub halt_force_pricing: [bool; 6],                   // 6
    pub perp_update_timestamp: u64,                      // 8
    pub perp_funding_delta: AnchorDecimal,               // 16
    pub perp_latest_funding_rate: AnchorDecimal,         // 16
    pub perp_latest_midpoint: u64,                       // 8
    pub _padding: [u8; 1593],                            // 1593
} // 10232

#[account]
#[derive(Default)]
pub struct OpenOrdersMap {
    pub user_key: Pubkey,
}

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct Position {
    pub size: i64,
    pub cost_of_trades: u64,
} // 16

impl Position {
    pub fn size_abs(&self) -> u64 {
        self.size.abs() as u64
    }

    pub fn get_unrealized_pnl(&self, mark_price: u64) -> i64 {
        if self.size == 0 {
            0
        } else if self.size > 0 {
            (self.size as i128)
                .checked_mul(mark_price as i128)
                .unwrap()
                .checked_div(POSITION_PRECISION_DENOMINATOR as i128)
                .unwrap()
                .checked_sub(self.cost_of_trades as i128)
                .unwrap()
                .try_into()
                .unwrap()
        } else {
            (self.size as i128)
                .checked_mul(mark_price as i128)
                .unwrap()
                .checked_div(POSITION_PRECISION_DENOMINATOR as i128)
                .unwrap()
                .checked_add(self.cost_of_trades as i128)
                .unwrap()
                .try_into()
                .unwrap()
        }
    }
}

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct OrderState {
    pub closing_orders: u64,
    pub opening_orders: [u64; 2],
} // 24

#[zero_copy]
#[derive(Default)]
#[repr(packed)]
pub struct ProductLedger {
    pub position: Position,
    pub order_state: OrderState,
} // 40

impl ProductLedger {
    pub fn get_initial_margin(
        &self,
        mark_price: u64,
        product: &Product,
        spot: u64,
        margin_parameters: &MarginParameters,
    ) -> u64 {
        let strike: u64 = match product.kind == Kind::Perp {
            true => 0,
            false => match product.strike.get_strike() {
                Ok(strike) => strike,
                Err(_) => return 0,
            },
        };

        let mut long_lots: u64 = self.order_state.opening_orders[BID_ORDERS_INDEX];
        let mut short_lots: u64 = self.order_state.opening_orders[ASK_ORDERS_INDEX];
        if self.position.size > 0 {
            long_lots = long_lots.checked_add(self.position.size_abs()).unwrap();
        } else if self.position.size < 0 {
            short_lots = short_lots.checked_add(self.position.size_abs()).unwrap();
        }

        let mut long_initial_margin: u128 = 0;
        let mut short_initial_margin: u128 = 0;

        if long_lots > 0 {
            long_initial_margin = (long_lots as u128)
                .checked_mul(
                    get_initial_margin_per_lot(
                        spot,
                        strike,
                        mark_price,
                        product.kind,
                        Side::Bid,
                        margin_parameters,
                    )
                    .unwrap()
                    .try_into()
                    .unwrap(),
                )
                .unwrap();
        }

        if short_lots > 0 {
            short_initial_margin = (short_lots as u128)
                .checked_mul(
                    get_initial_margin_per_lot(
                        spot,
                        strike,
                        mark_price,
                        product.kind,
                        Side::Ask,
                        margin_parameters,
                    )
                    .unwrap()
                    .try_into()
                    .unwrap(),
                )
                .unwrap();
        }

        if product.kind == Kind::Future || product.kind == Kind::Perp {
            if long_lots > short_lots {
                return long_initial_margin
                    .checked_div(POSITION_PRECISION_DENOMINATOR)
                    .unwrap()
                    .try_into()
                    .unwrap();
            } else {
                return short_initial_margin
                    .checked_div(POSITION_PRECISION_DENOMINATOR)
                    .unwrap()
                    .try_into()
                    .unwrap();
            }
        }

        long_initial_margin
            .checked_add(short_initial_margin)
            .unwrap()
            .checked_div(POSITION_PRECISION_DENOMINATOR)
            .unwrap()
            .try_into()
            .unwrap()
    }

    pub fn get_maintenance_margin(
        &self,
        mark_price: u64,
        product: &Product,
        spot: u64,
        margin_parameters: &MarginParameters,
    ) -> u64 {
        if self.position.size == 0 {
            return 0;
        }

        let strike: u64 = match product.kind == Kind::Perp {
            true => 0,
            false => match product.strike.get_strike() {
                Ok(strike) => strike,
                Err(_) => return 0,
            },
        };

        let maintenance_margin_per_lot = get_maintenance_margin_per_lot(
            spot,
            strike,
            mark_price,
            product.kind,
            self.position.size >= 0,
            margin_parameters,
        )
        .unwrap();

        (self.position.size_abs() as u128)
            .checked_mul(maintenance_margin_per_lot as u128)
            .unwrap()
            .checked_div(POSITION_PRECISION_DENOMINATOR)
            .unwrap() as u64
    }
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum Side {
    Uninitialized = 0,
    Bid = 1,
    Ask = 2,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum MarginAccountType {
    Normal = 0,
    MarketMaker = 1,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum ExpirySeriesStatus {
    Uninitialized = 0, // Still in default state
    Initialized = 1,   // Initialized but not active yet
    Live = 2,          // Active and trading.
    Expired = 3,       // Intermediate state after active trading
    ExpiredDirty = 4,  // State when series has expired but markets haven't been cleaned
}

#[zero_copy]
#[repr(packed)]
pub struct HaltState {
    halted: bool,                             // 1
    spot_price: u64,                          // 8
    timestamp: u64,                           // 8
    mark_prices_set: [bool; 2],               // 2
    _mark_prices_set_padding: [bool; 3],      // 3
    perp_mark_price_set: bool,                // 1
    market_nodes_cleaned: [bool; 2],          // 2
    _market_nodes_cleaned_padding: [bool; 4], // 4
    market_cleaned: [bool; 46],               // 46
    _market_cleaned_padding: [bool; 91],      // 91
    perp_market_cleaned: bool,                // 1
} // 167

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum Kind {
    Uninitialized = 0,
    Call = 1,
    Put = 2,
    Future = 3,
    Perp = 4,
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

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum Asset {
    SOL = 0,
    BTC = 1,
    ETH = 2,
    UNDEFINED = 255,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Clone, Copy)]
pub enum OrderCompleteType {
    Cancel = 0,
    Fill = 1,
    Booted = 2,
}
