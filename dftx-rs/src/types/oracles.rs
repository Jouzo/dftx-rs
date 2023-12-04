use bitcoin::hash_types::Txid;
use bitcoin::impl_consensus_encoding;
use bitcoin::io;
use bitcoin::ScriptBuf;
use dftx_macro::ConsensusEncoding;

use super::common::CompactVec;
use super::price::{CurrencyPair, TokenPrice};

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct SetOracleData {
    pub oracle_id: Txid,
    pub timestamp: i64,
    pub token_prices: CompactVec<TokenPrice>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct RemoveOracle {
    pub oracle_id: Txid,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct AppointOracle {
    pub script: ScriptBuf,
    pub weightage: u8,
    pub price_feeds: CompactVec<CurrencyPair>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct UpdateOracle {
    pub oracle_id: Txid,
    pub script: ScriptBuf,
    pub weightage: u8,
    pub price_feeds: CompactVec<CurrencyPair>,
}
