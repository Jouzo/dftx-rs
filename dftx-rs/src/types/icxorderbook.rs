use bitcoin::{impl_consensus_encoding, ScriptBuf, Txid};
use bitcoin::{io, VarInt};
use dftx_macro::ConsensusEncoding;

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXCreateOrder {
    pub order_type: u8,
    pub token_id: VarInt,
    pub owner_address: ScriptBuf,
    pub receive_pubkey: String,
    pub amount_from: u64,
    pub amount_to_fill: u64,
    pub order_price: u64,
    pub expiry: u32,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXMakeOffer {
    pub order_tx: String,
    pub amount: i64,
    pub owner_address: ScriptBuf,
    pub receive_pubkey: String,
    pub expiry: u32,
    pub taker_fee: i64,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXSubmitDFCHTLC {
    pub offer_tx: Txid,
    pub amount: i64,
    pub hash: Txid,
    pub timeout: u32,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXSubmitEXTHTLC {
    pub offer_tx: Txid,
    pub amount: i64,
    pub hash: Txid,
    pub htlc_script_address: String,
    pub owner_pubkey: String,
    pub timeout: u32,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXClaimDFCHTLC {
    pub dfc_htlc_tx: Txid,
    pub seed: Vec<u8>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXCloseOrder {
    pub order_tx: Txid,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct ICXCloseOffer {
    pub offer_tx: Txid,
}
