use bitcoin::impl_consensus_encoding;
use bitcoin::io;
use bitcoin::ScriptBuf;
use bitcoin::VarInt;
use dftx_macro::ConsensusEncoding;

use super::balance::ScriptBalances;
use super::balance::TokenBalanceUInt32;
use super::balance::TokenBalanceVarInt;
use super::common::CompactVec;
use super::common::Maybe;

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct MaxPrice {
    integer: u64,
    fraction: u64,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct PoolSwap {
    pub from_script: ScriptBuf,
    pub from_token_id: VarInt,
    pub from_amount: i64,
    pub to_script: ScriptBuf,
    pub to_token_id: VarInt,
    pub max_price: MaxPrice,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct PoolId {
    pub id: VarInt,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct CompositeSwap {
    pub pool_swap: PoolSwap,
    pub pools: CompactVec<PoolId>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct PoolAddLiquidity {
    pub from: CompactVec<ScriptBalances>,
    pub share_address: ScriptBuf,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct PoolRemoveLiquidity {
    pub script: ScriptBuf,
    pub amount: TokenBalanceVarInt,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct PoolCreatePair {
    pub token_a: VarInt,
    pub token_b: VarInt,
    pub commission: i64,
    pub owner_address: ScriptBuf,
    pub status: u8,
    pub pair_symbol: String,
    pub custom_rewards: Maybe<CompactVec<TokenBalanceUInt32>>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct PoolUpdatePair {
    pub pool_id: VarInt,
    pub status: u32,
    pub commission: i64,
    pub owner_address: ScriptBuf,
    pub custom_rewards: Maybe<CompactVec<TokenBalanceUInt32>>,
}
