use crate::*;

#[derive(Clone)]
pub struct Dex;

impl anchor_lang::Id for Dex {
    fn id() -> Pubkey {
        match cfg!(feature = "mainnet") {
            true => pubkey!("zDEXqXEG7gAyxb1Kg9mK5fPnUdENCGKzWrM21RMdWRq"),
            false => pubkey!("5CmWtUihvSrJpaUrpJ3H1jUa9DRjYz4v2xs6c3EgQWMf"),
        }
    }
}

#[derive(Clone)]
pub struct Chainlink;

impl anchor_lang::Id for Chainlink {
    fn id() -> Pubkey {
        chainlink_solana::ID
    }
}
