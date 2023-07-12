use crate::*;

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
    if product != Kind::Perp {
        return Err(error!(ZetaError::UnsupportedKind));
    }

    let initial_margin = (spot as u128)
        .checked_mul(margin_parameters.future_margin_initial.into())
        .unwrap()
        .checked_div(NATIVE_PRECISION_DENOMINATOR)
        .unwrap();

    Ok(u64::try_from(initial_margin).unwrap())
}

/// Maintenance margin for single product
pub fn get_maintenance_margin_per_lot(
    spot: u64,
    strike: u64,
    mark: u64,
    product: Kind,
    long: bool,
    margin_parameters: &MarginParameters,
) -> Result<u64> {
    if product != Kind::Perp {
        return Err(error!(ZetaError::UnsupportedKind));
    }
    let maintenance_margin: u128 = (spot as u128)
        .checked_mul(margin_parameters.future_margin_maintenance.into())
        .unwrap()
        .checked_div(NATIVE_PRECISION_DENOMINATOR)
        .unwrap();

    Ok(u64::try_from(maintenance_margin).unwrap())
}
