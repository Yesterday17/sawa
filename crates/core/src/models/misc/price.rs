#[cfg(feature = "schemars")]
use std::borrow::Cow;
use std::{ops::Deref, str::FromStr};

use serde::{Serialize, ser::SerializeStruct};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Price {
    /// The ISO 4217 currency code (e.g., "USD", "EUR").
    pub currency: Currency,

    /// The amount of the price in the smallest currency unit (e.g., cents for USD).
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Currency(iso_currency::Currency);

impl Currency {
    pub const JPY: Self = Self(iso_currency::Currency::JPY);
    pub const USD: Self = Self(iso_currency::Currency::USD);
}

/// Serialize as an object with:
/// - code: ISO 4217 currency code
/// - exponent: number of decimal places
/// - symbol: currency symbol
impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Currency", 3)?;
        state.serialize_field("code", &self.0.code())?;
        state.serialize_field("exponent", &self.0.exponent())?;
        state.serialize_field("symbol", &self.0.symbol().symbol)?;
        state.end()
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Currency {
    fn schema_name() -> Cow<'static, str> {
        "Currency".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "properties": {
                "code": { "type": "string" },
                "exponent": { "type": "integer" },
                "symbol": { "type": "string" }
            },
            "required": ["code", "exponent", "symbol"]
        })
    }
}

impl FromStr for Currency {
    type Err = iso_currency::ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let currency = iso_currency::Currency::from_str(s)?;
        Ok(Currency(currency))
    }
}

impl Deref for Currency {
    type Target = iso_currency::Currency;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
