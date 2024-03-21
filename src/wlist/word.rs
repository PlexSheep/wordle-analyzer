use std::collections::HashMap;
use std::fmt::{write, Display};
use std::iter::Sum;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// NOTE: We might need a different implementation for more precision
// NOTE: This struct requires a custom Serialize and Deserialize implementation
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Frequency {
    inner: f64,
}

// PERF: Hash for String is probably a bottleneck
pub type Word = String;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WordMap {
    #[serde(flatten)]
    inner: HashMap<Word, Frequency>,
}

impl WordMap {
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, Frequency> {
        self.inner.keys()
    }
    pub fn values(&self) -> std::collections::hash_map::Values<'_, String, Frequency> {
        self.inner.values()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Frequency> {
        self.inner.iter()
    }
}

// We need custom Serialize and Deserialize of Frequency, because they are only primitive types.
// Serde does not support serializing directly to and from primitives (such as floats)
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Frequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FrequencyVisitor;
        impl<'v> serde::de::Visitor<'v> for FrequencyVisitor {
            type Value = Frequency;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "a floating-point number")
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Frequency { inner: v })
            }
        }

        deserializer.deserialize_any(FrequencyVisitor)
    }
}
#[cfg(feature = "serde")]
impl Serialize for Frequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.inner)
    }
}

impl From<Frequency> for f64 {
    fn from(value: Frequency) -> Self {
        value.inner
    }
}

impl From<f64> for Frequency {
    fn from(value: f64) -> Self {
        Frequency { inner: value }
    }
}

impl Sum for Frequency {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self { inner: 0.0 }, |a, b| Self {
            inner: a.inner + b.inner,
        })
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("{}", self.inner))
    }
}
