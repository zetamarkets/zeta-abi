use crate::*;

pub fn get_maintenance_margin_per_lot(
    spot: u64,
    strike: u64,
    mark: u64,
    product: Kind,
    long: bool,
    margin_parameters: &MarginParameters,
) -> Result<u64> {
    let maintenance_margin: u128 = match product {
        Kind::Future | Kind::Perp => (spot as u128)
            .checked_mul(margin_parameters.future_margin_maintenance.into())
            .unwrap()
            .checked_div(NATIVE_PRECISION_DENOMINATOR)
            .unwrap(),
        Kind::Call | Kind::Put => {
            if long {
                (spot as u128)
                    .checked_mul(
                        margin_parameters
                            .option_spot_percentage_long_maintenance
                            .into(),
                    )
                    .unwrap()
                    .checked_div(NATIVE_PRECISION_DENOMINATOR)
                    .unwrap()
                    .min(
                        (mark as u128)
                            .checked_mul(
                                margin_parameters
                                    .option_mark_percentage_long_maintenance
                                    .into(),
                            )
                            .unwrap()
                            .checked_div(NATIVE_PRECISION_DENOMINATOR)
                            .unwrap(),
                    )
            } else {
                let otm_amount: u128 = get_otm_amount(spot, strike, product)?.into();
                let otm_pct = otm_amount
                    .checked_mul(NATIVE_PRECISION_DENOMINATOR)
                    .unwrap()
                    .checked_div(spot.into())
                    .unwrap();

                let dynamic_margin_pct: u128 =
                    (margin_parameters.option_dynamic_percentage_short_maintenance as u128)
                        .checked_sub(otm_pct)
                        .unwrap_or(0);

                let margin_pct = dynamic_margin_pct.max(
                    margin_parameters
                        .option_spot_percentage_short_maintenance
                        .into(),
                );

                margin_pct
                    .checked_mul(spot.into())
                    .unwrap()
                    .checked_div(NATIVE_PRECISION_DENOMINATOR)
                    .unwrap()
            }
        }
        _ => return Err(error!(ZetaError::UnsupportedKind)),
    };

    if product == Kind::Put && !long {
        let sell_put_cap_margin = (strike as u128)
            .checked_mul(margin_parameters.option_short_put_cap_percentage as u128)
            .unwrap()
            .checked_div(NATIVE_PRECISION_DENOMINATOR)
            .unwrap();

        return Ok(u64::try_from(maintenance_margin.min(sell_put_cap_margin)).unwrap());
    }

    Ok(u64::try_from(maintenance_margin).unwrap())
}

pub fn get_otm_amount(spot: u64, strike: u64, product: Kind) -> Result<u64> {
    match product {
        Kind::Call => Ok((strike as i128)
            .checked_sub(spot as i128)
            .unwrap()
            .max(0)
            .try_into()
            .unwrap()),
        Kind::Put => Ok((spot as i128)
            .checked_sub(strike as i128)
            .unwrap()
            .max(0)
            .try_into()
            .unwrap()),
        _ => return Err(error!(ZetaError::UnsupportedKind)),
    }
}

/// Initial margin for single product
// we maintain 6 d.p and 2 d.p for percentage calculations
// keep this consistent, could maybe make a function get_native_percentage?
// would just return percentage * 10^N, where N = 2 = percentage precision.....
pub fn get_initial_margin_per_lot(
    spot: u64,
    strike: u64,
    mark: u64,
    product: Kind,
    side: Side,
    margin_parameters: &MarginParameters,
) -> Result<u64> {
    let initial_margin: u128 = match product {
        // 15% of Spot
        Kind::Future | Kind::Perp => (spot as u128)
            .checked_mul(margin_parameters.future_margin_initial.into())
            .unwrap()
            .checked_div(NATIVE_PRECISION_DENOMINATOR)
            .unwrap(),
        Kind::Call | Kind::Put => {
            match side {
                Side::Bid => {
                    // min(100% * mark price, 15% of spot)
                    // Place holder calcs in place for 100% * mark price
                    (spot as u128)
                        .checked_mul(margin_parameters.option_spot_percentage_long_initial.into())
                        .unwrap()
                        .checked_div(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                        .min(
                            (mark as u128)
                                .checked_mul(
                                    margin_parameters.option_mark_percentage_long_initial.into(),
                                )
                                .unwrap()
                                .checked_div(NATIVE_PRECISION_DENOMINATOR)
                                .unwrap(),
                        )
                }
                Side::Ask => {
                    let otm_amount: u128 = get_otm_amount(spot, strike, product)?.into();
                    // max(25% - OTM Amount/spot, 10%)
                    let otm_pct = otm_amount
                        .checked_mul(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                        .checked_div(spot.into())
                        .unwrap();

                    let dynamic_margin_pct =
                        (margin_parameters.option_dynamic_percentage_short_initial as u128)
                            .checked_sub(otm_pct)
                            .unwrap_or(0);

                    let margin_pct = dynamic_margin_pct.max(
                        margin_parameters
                            .option_spot_percentage_short_initial
                            .into(),
                    );
                    margin_pct
                        .checked_mul(spot.into())
                        .unwrap()
                        .checked_div(NATIVE_PRECISION_DENOMINATOR)
                        .unwrap()
                }
                Side::Uninitialized => unreachable!(),
            }
        }
        _ => return Err(error!(ZetaError::UnsupportedKind)),
    };

    if product == Kind::Put && side == Side::Ask {
        let sell_put_cap_margin = (strike as u128)
            .checked_mul(margin_parameters.option_short_put_cap_percentage as u128)
            .unwrap()
            .checked_div(NATIVE_PRECISION_DENOMINATOR)
            .unwrap();

        return Ok(u64::try_from(initial_margin.min(sell_put_cap_margin)).unwrap());
    }

    Ok(u64::try_from(initial_margin).unwrap())
}
