use bitcoin::{impl_consensus_encoding, io, ScriptBuf, VarInt};
use dftx_macro::ConsensusEncoding;

use super::common::CompactVec;

// CBalances
#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct TokenBalanceUInt32 {
    pub token: u32,
    pub amount: i64,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ScriptBalances {
    pub script: ScriptBuf,
    pub balances: CompactVec<TokenBalanceUInt32>,
}

// CTokenAmount
#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct TokenBalanceVarInt {
    pub token: VarInt,
    pub amount: i64,
}
