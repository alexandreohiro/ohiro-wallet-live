use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub unit_value_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseRecord {
    pub id: i64,
    pub asset_id: i64,
    pub quantity_milli: i64,
    pub bought_for_cents: i64,
    pub purchase_date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PurchaseView {
    pub id: i64,
    pub purchase_date: String,
    pub quantity: String,
    pub bought_for: String,
    pub change: String,
    pub change_is_positive: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetView {
    pub id: i64,
    pub name: String,
    pub quantity: String,
    pub unit_value: String,
    pub change: String,
    pub change_is_positive: bool,
    pub purchases: Vec<PurchaseView>,
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AssetForm {
    pub name: String,
    pub unit_value: String,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseForm {
    pub asset_id: i64,
    pub unit_value: String,
    pub quantity: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiCreateAsset {
    pub name: String,
    pub unit_value_cents: i64,
}
