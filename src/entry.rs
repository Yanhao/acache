use std::fmt::Debug;

use bytes::Buf;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::codec::Codec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry<K, V> {
    pub key: K,
    pub value: Option<V>,
    pub expire_at_ms: i64,
}

pub trait EntryTrait<K> {
    fn is_outdated(&self) -> bool;
    fn get_key(&self) -> K;
}

impl<K, V> EntryTrait<K> for Entry<K, V>
where
    K: Clone,
{
    fn is_outdated(&self) -> bool {
        Utc.timestamp_millis_opt(self.expire_at_ms).unwrap() < Utc::now()
    }

    fn get_key(&self) -> K {
        self.key.clone()
    }
}

impl<K, V> Codec for Entry<K, V>
where
    K: Codec + Clone,
    V: Codec,
{
    fn decode(data: bytes::Bytes) -> anyhow::Result<Self> {
        let eni: EntryInner<K> = serde_json::from_reader(data.reader())?;

        let value = serde_json::from_reader(eni.value_data.reader())?;

        Ok(Self {
            key: eni.key,
            value,
            expire_at_ms: eni.expire_at_ms,
        })
    }

    fn encode(&self) -> anyhow::Result<bytes::Bytes> {
        let value_data = serde_json::to_vec(&self.value)?;
        let eni = EntryInner {
            key: self.key.clone(),
            value_data,
            expire_at_ms: self.expire_at_ms,
        };

        Ok(serde_json::to_vec(&eni)?.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EntryInner<K> {
    key: K,
    value_data: Vec<u8>,
    expire_at_ms: i64,
}