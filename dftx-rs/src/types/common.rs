use bitcoin::consensus::{Decodable, Encodable};
use bitcoin::io::ErrorKind;
use bitcoin::VarInt;

#[derive(Debug, PartialEq, Eq)]
pub struct CompactVec<T>(Vec<T>);

impl<T: Encodable + std::fmt::Debug> Encodable for CompactVec<T> {
    fn consensus_encode<W: bitcoin::io::Write + ?Sized>(
        &self,
        w: &mut W,
    ) -> Result<usize, bitcoin::io::Error> {
        let mut len = VarInt(self.0.len() as u64).consensus_encode(w)?;
        for item in self.0.iter() {
            len += item.consensus_encode(w)?;
        }
        Ok(len)
    }
}

impl<T: Decodable + std::fmt::Debug> Decodable for CompactVec<T> {
    fn consensus_decode<R: bitcoin::io::Read + ?Sized>(
        r: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        let len = VarInt::consensus_decode(r)?.0;
        let mut ret = Vec::with_capacity(len as usize);
        for _ in 0..len {
            ret.push(Decodable::consensus_decode(r)?);
        }
        Ok(CompactVec(ret))
    }
}

impl<T> From<Vec<T>> for CompactVec<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Maybe<T>(pub Option<T>);
impl<T: Encodable + std::fmt::Debug> Encodable for Maybe<T> {
    fn consensus_encode<W: bitcoin::io::Write + ?Sized>(
        &self,
        w: &mut W,
    ) -> Result<usize, bitcoin::io::Error> {
        match &self.0 {
            Some(v) => v.consensus_encode(w),
            None => Ok(0),
        }
    }
}

impl<T: Decodable + std::fmt::Debug> Decodable for Maybe<T> {
    fn consensus_decode<R: bitcoin::io::Read + ?Sized>(
        r: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        match T::consensus_decode(r) {
            Ok(v) => Ok(Self(Some(v))),
            Err(bitcoin::consensus::encode::Error::Io(e))
                if e.kind() == ErrorKind::UnexpectedEof =>
            {
                Ok(Self(None))
            }
            Err(e) => Err(e),
        }
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(v: Option<T>) -> Self {
        Self(v)
    }
}
