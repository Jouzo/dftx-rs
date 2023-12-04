use bitcoin::{impl_consensus_encoding, ScriptBuf};
use bitcoin::{io, VarInt};
use dftx_macro::ConsensusEncoding;

use super::balance::{ScriptBalances, TokenBalanceUInt32, TokenBalanceVarInt};
use super::common::CompactVec;

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct UtxosToAccount {
    pub to: CompactVec<ScriptBalances>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct AccountToUtxos {
    pub from: ScriptBuf,
    pub balances: CompactVec<TokenBalanceUInt32>,
    pub minting_outputs_start: VarInt,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct AccountToAccount {
    pub from: ScriptBuf,
    pub to: CompactVec<ScriptBalances>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct AnyAccountsToAccounts {
    pub from: CompactVec<ScriptBalances>,
    pub to: CompactVec<ScriptBalances>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct TransferDomainItem {
    pub address: ScriptBuf,
    pub amount: TokenBalanceVarInt,
    pub domain: u8,
    pub data: Vec<u8>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct TransferDomainPair {
    pub src: TransferDomainItem,
    pub dst: TransferDomainItem,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct TransferDomain {
    pub items: CompactVec<TransferDomainPair>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct SetFutureSwap {
    pub owner: ScriptBuf,
    pub source: TokenBalanceVarInt,
    pub destination: u32,
    pub withdraw: bool,
}
