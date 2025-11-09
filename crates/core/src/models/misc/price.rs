use iso_currency::Currency;

#[derive(Debug, Clone, Copy)]
pub struct Price {
    /// The ISO 4217 currency code (e.g., "USD", "EUR").
    pub currency: Currency,

    /// The amount of the price in the smallest currency unit (e.g., cents for USD).
    pub amount: u64,
}
