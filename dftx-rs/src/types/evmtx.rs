use bitcoin::{impl_consensus_encoding, io};
use dftx_macro::ConsensusEncoding;

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct EvmTx {
    pub raw: Vec<u8>,
}
