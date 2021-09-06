/// Market type.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MarketType {
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,

    AmericanOption,
    EuropeanOption,

    QuantoFuture,
    QuantoSwap,

    Move,
    #[allow(clippy::upper_case_acronyms)]
    BVOL,
}

impl MarketType {
    // Converts C MarketType to Rust MarketType
    pub fn to_rust(self) -> crypto_contract_value::MarketType {
        match self {
            MarketType::Spot => crypto_contract_value::MarketType::Spot,
            MarketType::LinearFuture => crypto_contract_value::MarketType::LinearFuture,
            MarketType::InverseFuture => crypto_contract_value::MarketType::InverseFuture,
            MarketType::LinearSwap => crypto_contract_value::MarketType::LinearSwap,
            MarketType::InverseSwap => crypto_contract_value::MarketType::InverseSwap,
            MarketType::AmericanOption => crypto_contract_value::MarketType::AmericanOption,
            MarketType::EuropeanOption => crypto_contract_value::MarketType::EuropeanOption,
            MarketType::QuantoFuture => crypto_contract_value::MarketType::QuantoFuture,
            MarketType::QuantoSwap => crypto_contract_value::MarketType::QuantoSwap,
            MarketType::Move => crypto_contract_value::MarketType::Move,
            MarketType::BVOL => crypto_contract_value::MarketType::BVOL,
        }
    }

    // Converts Rust MarketType to C MarketType
    pub fn from_rust(market_type: crypto_contract_value::MarketType) -> Self {
        match market_type {
            crypto_contract_value::MarketType::Spot => MarketType::Spot,
            crypto_contract_value::MarketType::LinearFuture => MarketType::LinearFuture,
            crypto_contract_value::MarketType::InverseFuture => MarketType::InverseFuture,
            crypto_contract_value::MarketType::LinearSwap => MarketType::LinearSwap,
            crypto_contract_value::MarketType::InverseSwap => MarketType::InverseSwap,
            crypto_contract_value::MarketType::AmericanOption => MarketType::AmericanOption,
            crypto_contract_value::MarketType::EuropeanOption => MarketType::EuropeanOption,
            crypto_contract_value::MarketType::QuantoFuture => MarketType::QuantoFuture,
            crypto_contract_value::MarketType::QuantoSwap => MarketType::QuantoSwap,
            crypto_contract_value::MarketType::Move => MarketType::Move,
            crypto_contract_value::MarketType::BVOL => MarketType::BVOL,
        }
    }
}
