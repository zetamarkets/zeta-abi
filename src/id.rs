use crate::*;

#[derive(Clone)]
pub struct ZetaProgram;

#[cfg(feature = "mainnet")]
declare_id!("ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD");
#[cfg(not(feature = "mainnet"))]
declare_id!("BG3oRikW8d16YjUEmX3ZxHm9SiJzrGtMhsSR8aCw1Cd7");

impl anchor_lang::Id for ZetaProgram {
    fn id() -> Pubkey {
        ID
    }
}
