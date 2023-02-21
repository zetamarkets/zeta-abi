use crate::*;

#[error_code]
pub enum ZetaError {
    #[msg("Product strike uninitialized")]
    ProductStrikeUninitialized,
    #[msg("Unsupported kind")]
    UnsupportedKind,
    #[msg("Cannot set initialized strike")]
    CannotSetInitializedStrike,
    #[msg("Cannot set initialized strike")]
    CannotResetUninitializedStrike,
}
