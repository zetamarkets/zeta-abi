use crate::*;

#[derive(Clone)]
pub struct ZetaProgram;

#[cfg(feature = "mainnet")]
declare_id!("z6ikFPqPGi16hCFfytMBP2Ao2iNGaN57mBQHvFGAit7");
#[cfg(not(feature = "mainnet"))]
declare_id!("zG36tRvqtkCtFEoXmQAq9aV19XCLn53QSceLmqorVFA");

impl anchor_lang::Id for ZetaProgram {
    fn id() -> Pubkey {
        match cfg!(feature = "mainnet") {
            true => pubkey!("ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD"),
            false => pubkey!("BG3oRikW8d16YjUEmX3ZxHm9SiJzrGtMhsSR8aCw1Cd7"),
        }
    }
}

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
