pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state_or_province: String,
    pub postal_code: String,
    pub country: String, // Maybe use a crate for country codes later
}
