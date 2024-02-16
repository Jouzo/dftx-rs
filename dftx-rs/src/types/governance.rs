use bitcoin::{
    consensus::{Decodable, Encodable, ReadExt},
    impl_consensus_encoding,
    io::{self},
    ScriptBuf, Txid, VarInt,
};
use dftx_macro::ConsensusEncoding;

use super::common::CompactVec;
use crate::types::common::Maybe;

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct LiqPoolSplit {
    pub token_id: u32,
    pub value: i64,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct LpDailyReward {
    pub key: String,
    pub value: i64,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct LpSplits {
    pub key: String,
    pub value: CompactVec<LiqPoolSplit>,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct LpUnmapped {
    pub key: String,
    pub value: String,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct LoanTokenSplit {
    pub token_id: VarInt,
    pub value: i64,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct LpLoanTokenSplits {
    pub key: String,
    pub value: CompactVec<LoanTokenSplit>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GovernanceVar {
    LpDailyReward(LpDailyReward),
    LpSplits(LpSplits),
    LpLoanTokenSplits(LpLoanTokenSplits),
    Unmapped(LpUnmapped),
}

impl Encodable for GovernanceVar {
    fn consensus_encode<W: io::Write + ?Sized>(&self, writer: &mut W) -> Result<usize, io::Error> {
        match self {
            GovernanceVar::LpDailyReward(data) => data.consensus_encode(writer),
            GovernanceVar::LpSplits(data) => data.consensus_encode(writer),
            GovernanceVar::LpLoanTokenSplits(data) => data.consensus_encode(writer),
            GovernanceVar::Unmapped(data) => data.consensus_encode(writer),
        }
    }
}

impl Decodable for GovernanceVar {
    fn consensus_decode<R: io::Read + ?Sized>(
        reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        let r#type = String::consensus_decode(reader)?;
        match r#type.as_str() {
            "LP_DAILY_DFI_REWARD" => Ok(Self::LpDailyReward(LpDailyReward {
                key: r#type,
                value: i64::consensus_decode(reader)?,
            })),
            "LP_SPLITS" => Ok(Self::LpSplits(LpSplits {
                key: r#type,
                value: <CompactVec<LiqPoolSplit>>::consensus_decode(reader)?,
            })),
            "LP_LOAN_TOKEN_SPLITS" => {
                println!("r#type : {:?}", r#type);
                return Ok(Self::LpLoanTokenSplits(LpLoanTokenSplits {
                    key: r#type,
                    value: <CompactVec<LoanTokenSplit>>::consensus_decode(reader)?,
                }));
            }
            _ => Ok(Self::Unmapped(LpUnmapped {
                key: r#type,
                value: String::consensus_decode(reader)?,
            })),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetGovernance {
    pub governance_vars: Vec<GovernanceVar>,
}

impl Encodable for SetGovernance {
    fn consensus_encode<W: io::Write + ?Sized>(&self, writer: &mut W) -> Result<usize, io::Error> {
        self.governance_vars.iter().try_fold(0, |acc, var| {
            var.consensus_encode(writer).map(|len| acc + len)
        })
    }
}

impl Decodable for SetGovernance {
    fn consensus_decode<R: io::Read + ?Sized>(
        reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        let mut govs = Vec::new();
        while let Maybe(Some(var)) = <Maybe<GovernanceVar>>::consensus_decode(reader)? {
            govs.push(var)
        }

        Ok(Self {
            governance_vars: govs,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetGovernanceHeight {
    pub var: GovernanceVar,
    pub activation_height: u32,
}

impl Encodable for SetGovernanceHeight {
    fn consensus_encode<W: io::Write + ?Sized>(&self, writer: &mut W) -> Result<usize, io::Error> {
        let mut len = self.var.consensus_encode(writer)?;
        len += self.activation_height.consensus_encode(writer)?;
        Ok(len)
    }
}

impl Decodable for SetGovernanceHeight {
    fn consensus_decode<R: io::Read + ?Sized>(
        reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(Self {
            var: GovernanceVar::consensus_decode(reader)?,
            activation_height: reader.read_u32()?,
        })
    }
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct CreateProposal {
    pub r#type: u8,
    pub address: ScriptBuf,
    pub n_amount: i64,
    pub n_cycles: u8,
    pub title: String,
    pub context: String,
    pub contexthash: String,
    pub options: u8,
}

#[derive(ConsensusEncoding, Debug, PartialEq, Eq)]
pub struct Vote {
    pub proposal_id: Txid,
    pub masternode_id: Txid,
    pub vote_decision: u8,
}
