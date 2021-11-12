use crate::utils::interplanetary::context::InterplanetaryContext;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::kinds::recursive_data::RecursiveData;
use crate::utils::interplanetary::kinds::recursive_data_envelope::RecursiveDataEnvelope;
use crate::utils::interplanetary::kinds::scalar_data::ScalarData;
use crate::utils::interplanetary::kinds::scalar_data_envelope::ScalarDataEnvelope;
use anyhow::Result;
use cid::Cid;
use sk_cbor::Value;
use std::convert::TryFrom;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to read HoliumCBOR data")]
    FailedToReadHoliumCborData,
    #[error("failed to write HoliumCBOR data")]
    FailedToWriteHoliumCborData,
}

pub struct HoliumInterplanetaryNodeData(pub Value);

impl HoliumInterplanetaryNodeData {
    pub fn new(data_vec: Vec<u8>) -> Result<Self> {
        let data = sk_cbor::read(&data_vec).map_err(|_| Error::FailedToReadHoliumCborData)?;
        Ok(HoliumInterplanetaryNodeData(data))
    }

    pub fn from_cid(cid: &Cid, ip_context: &InterplanetaryContext) -> Result<Self> {
        let value = Self::recursively_read_from_ip_area(cid, ip_context)?;
        Ok(HoliumInterplanetaryNodeData(value))
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut data_vec: Vec<u8> = Vec::new();
        sk_cbor::write(self.0.clone(), &mut data_vec)
            .map_err(|_| Error::FailedToWriteHoliumCborData)?;
        Ok(data_vec)
    }

    fn recursively_read_from_ip_area(
        cid: &Cid,
        ip_context: &InterplanetaryContext,
    ) -> Result<Value> {
        let block = Value::read_from_ip_area(cid, ip_context)?;
        if let Ok(scalar_data_envelope) = ScalarDataEnvelope::try_from(*block.clone()) {
            // fetch scalar data from ip area
            let scalar_data =
                ScalarData::read_from_ip_area(&scalar_data_envelope.scalar_data_cid, ip_context)?;
            // unwrap scalar data
            let data = sk_cbor::read(&scalar_data.content)
                .map_err(|_| Error::FailedToReadHoliumCborData)?;
            Ok(data)
        } else if let Ok(recursive_data_envelope) = RecursiveDataEnvelope::try_from(*block) {
            // fetch recursive data from ip area
            let recursive_data_block =
                Value::read_from_ip_area(&recursive_data_envelope.recursive_data_cid, ip_context)?;
            let recursive_data = RecursiveData::try_from(*recursive_data_block)?;
            // recursively read elements from ip area
            let elements = recursive_data
                .elements_cids
                .iter()
                .map(|element_cid| Self::recursively_read_from_ip_area(element_cid, ip_context))
                .collect::<Result<Vec<Value>>>()?;
            let data = Value::Array(elements);
            Ok(data)
        } else {
            Err(Error::FailedToReadHoliumCborData.into())
        }
    }

    pub fn recursively_write_to_ip_area(&self, ip_context: &InterplanetaryContext) -> Result<Cid> {
        match &self.0 {
            Value::Array(vec) => {
                // recursively write elements to ip area
                let elements_cids = vec
                    .iter()
                    .map(|item| {
                        HoliumInterplanetaryNodeData(item.clone())
                            .recursively_write_to_ip_area(ip_context)
                    })
                    .collect::<Result<Vec<Cid>>>()?;
                // write recursive data to ip area
                let rec_data = RecursiveData { elements_cids };
                let rec_data_cid = Value::from(rec_data).write_to_ip_area(&ip_context)?;
                // write recursive data envelope to ip area
                let rec_data_envelope = RecursiveDataEnvelope {
                    recursive_data_cid: rec_data_cid,
                };
                let rec_data_envelope_cid =
                    Value::from(rec_data_envelope).write_to_ip_area(&ip_context)?;
                // return recursive data envelope cid
                Ok(rec_data_envelope_cid)
            }
            _ => {
                // write scalar data to ip area
                let mut data_vec: Vec<u8> = Vec::new();
                sk_cbor::write(self.0.clone(), &mut data_vec)
                    .map_err(|_| Error::FailedToWriteHoliumCborData)?;
                let scalar_data = ScalarData { content: data_vec };
                let scalar_data_cid = scalar_data.write_to_ip_area(&ip_context)?;
                // write scalar data envelope to ip area
                let scalar_data_envelope = ScalarDataEnvelope { scalar_data_cid };
                let scalar_data_envelope_cid =
                    Value::from(scalar_data_envelope).write_to_ip_area(&ip_context)?;
                // return scalar data envelope cid
                Ok(scalar_data_envelope_cid)
            }
        }
    }
}
