use std::fmt::Display;

use byte_slice_cast::AsByteSlice;
use primitive_types::U256;
use ring::digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy)]
pub struct NodeID(U256);

impl NodeID {
    pub fn random() -> Self {
        Self(rand::random::<[u8; 32]>().into())
    }

    pub fn zero() -> Self {
        Self(U256::zero())
    }
}

impl Display for NodeID {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#018x}", self.0.as_ref()[0],)
    }
}

impl From<NodeSecret> for NodeID {
    fn from(value: NodeSecret) -> Self {
        Self(
            digest::digest(&digest::SHA256, value.0.as_byte_slice())
                .as_ref()
                .into(),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct NodeSecret(U256);

impl NodeSecret {
    pub fn random() -> Self {
        Self(rand::random::<[u8; 32]>().into())
    }

    pub fn zero() -> Self {
        Self(U256::zero())
    }
}

impl Display for NodeSecret {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#018x}", self.0.as_ref()[0],)
    }
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    derive_more::AddAssign,
    derive_more::SubAssign,
    PartialEq,
    PartialOrd,
    Copy,
)]
pub struct Mana(U256);

impl Display for Mana {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0.as_u128(),)
    }
}

impl From<u128> for Mana {
    fn from(value: u128) -> Self {
        Self(value.into())
    }
}

impl Mana {
    pub fn zero() -> Self {
        Mana(U256::zero())
    }
}
