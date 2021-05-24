use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::proto::common::NodeAddress;

const NODE_ADDRESS_FIELDS: &'static [&'static str] = &["kv_address", "raft_address"];

impl Serialize for NodeAddress {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut node_address =
            serializer.serialize_struct("NodeAddress", NODE_ADDRESS_FIELDS.len())?;
        node_address.serialize_field("kv_address", &self.kv_address)?;
        node_address.serialize_field("raft_address", &self.raft_address)?;
        node_address.end()
    }
}

impl<'de> Deserialize<'de> for NodeAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            KvAddress,
            RaftAddress,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`kv_address` or `raft_address`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "kv_address" => Ok(Field::KvAddress),
                            "raft_address" => Ok(Field::RaftAddress),
                            _ => Err(de::Error::unknown_field(value, NODE_ADDRESS_FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct NodeAddressVisitor;

        impl<'de> Visitor<'de> for NodeAddressVisitor {
            type Value = NodeAddress;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct NodeAddress")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<NodeAddress, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let kv_address = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let raft_address = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let mut node_address = NodeAddress::new();
                node_address.set_kv_address(kv_address);
                node_address.set_raft_address(raft_address);
                Ok(node_address)
            }

            fn visit_map<V>(self, mut map: V) -> Result<NodeAddress, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut kv_address = None;
                let mut raft_address = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::KvAddress => {
                            if kv_address.is_some() {
                                return Err(de::Error::duplicate_field("kv_address"));
                            }
                            kv_address = Some(map.next_value()?);
                        }
                        Field::RaftAddress => {
                            if raft_address.is_some() {
                                return Err(de::Error::duplicate_field("raft_address"));
                            }
                            raft_address = Some(map.next_value()?);
                        }
                    }
                }
                let kv_address =
                    kv_address.ok_or_else(|| de::Error::missing_field("kv_address"))?;
                let raft_address =
                    raft_address.ok_or_else(|| de::Error::missing_field("raft_address"))?;

                let mut node_address = NodeAddress::new();
                node_address.set_kv_address(kv_address);
                node_address.set_raft_address(raft_address);
                Ok(node_address)
            }
        }

        deserializer.deserialize_struct("NodeAddress", NODE_ADDRESS_FIELDS, NodeAddressVisitor)
    }
}
