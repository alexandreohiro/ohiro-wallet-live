use password_auth::{generate_hash, verify_password};
use std::collections::HashSet;

use crate::{
    app::AppState,
    errors::AppError,
    models::{AssetRecord, AssetView, PurchaseRecord, PurchaseView, UserRecord},
    utils::{change_cents, format_money, format_quantity, format_signed_money},
};

#[derive(Debug, Default)]
pub struct MemoryStore {
    next_user_id: i64,
    next_asset_id: i64,
    next_purchase_id: i64,
    users: Vec<UserRecord>,
    assets: Vec<AssetRecord>,
    purchases: Vec<PurchaseRecord>,
}

impl MemoryStore {
    pub fn seed_demo(&mut self) {
        if !self.users.is_empty() {
            return;
        }

        let user = UserRecord {
            id: self.user_id(),
            name: "Alexandre".to_string(),
            username: "alexandre".to_string(),
            password_hash: generate_hash("rust123"),
        };

        self.users.push(user.clone());

        let bitcoin = self.asset_id();
        self.assets.push(AssetRecord {
            id: bitcoin,
            user_id: user.id,
            name: "Bitcoin".to_string(),
            unit_value_cents: 1_000,
        });
        self.add_seed_purchase(bitcoin, 5_000, 1_500, "2026-03-22 15:24");
        self.add_seed_purchase(bitcoin, 10_000, 500, "2026-03-22 15:24");

        let real = self.asset_id();
        self.assets.push(AssetRecord {
            id: real,
            user_id: user.id,
            name: "Real".to_string(),
            unit_value_cents: 100,
        });
        self.add_seed_purchase(real, 100_000, 100, "2026-03-22 16:25");
        self.add_seed_purchase(real, 250_000, 75, "2026-03-22 16:25");

        let dollar = self.asset_id();
        self.assets.push(AssetRecord {
            id: dollar,
            user_id: user.id,
            name: "Dolar".to_string(),
            unit_value_cents: 525,
        });
        self.add_seed_purchase(dollar, 500_000, 550, "2026-03-22 18:27");
    }

    fn add_seed_purchase(
        &mut self,
        asset_id: i64,
        quantity_milli: i64,
        bought_for_cents: i64,
        purchase_date: &str,
    ) {
        let id = self.purchase_id();
        self.purchases.push(PurchaseRecord {
            id,
            asset_id,
            quantity_milli,
            bought_for_cents,
            purchase_date: purchase_date.to_string(),
        });
    }

    fn user_id(&mut self) -> i64 {
        self.next_user_id += 1;
        self.next_user_id
    }

    fn asset_id(&mut self) -> i64 {
        self.next_asset_id += 1;
        self.next_asset_id
    }

    fn purchase_id(&mut self) -> i64 {
        self.next_purchase_id += 1;
        self.next_purchase_id
    }
}

pub async fn create_user(
    state: &AppState,
    name: &str,
    username: &str,
    password: &str,
) -> Result<UserRecord, AppError> {
    let name = name.trim();
    let username = username.trim().to_lowercase();

    if name.len() < 2 {
        return Err(AppError::InvalidInput(
            "nome deve ter pelo menos 2 caracteres".to_string(),
        ));
    }
    if username.len() < 3 {
        return Err(AppError::InvalidInput(
            "username deve ter pelo menos 3 caracteres".to_string(),
        ));
    }
    if password.len() < 6 {
        return Err(AppError::InvalidInput(
            "senha deve ter pelo menos 6 caracteres".to_string(),
        ));
    }

    let mut memory = state.memory.lock().map_err(|_| AppError::MemoryPoisoned)?;

    if memory.users.iter().any(|user| user.username == username) {
        return Err(AppError::Conflict("username ja cadastrado".to_string()));
    }

    let user = UserRecord {
        id: memory.user_id(),
        name: name.to_string(),
        username,
        password_hash: generate_hash(password),
    };

    memory.users.push(user.clone());
    Ok(user)
}

pub async fn find_user_by_username(
    state: &AppState,
    username: &str,
) -> Result<Option<UserRecord>, AppError> {
    let username = username.trim().to_lowercase();
    let memory = state.memory.lock().map_err(|_| AppError::MemoryPoisoned)?;
    Ok(memory
        .users
        .iter()
        .find(|user| user.username == username)
        .cloned())
}

pub async fn find_user_by_id(
    state: &AppState,
    user_id: i64,
) -> Result<Option<UserRecord>, AppError> {
    let memory = state.memory.lock().map_err(|_| AppError::MemoryPoisoned)?;
    Ok(memory.users.iter().find(|user| user.id == user_id).cloned())
}

pub async fn verify_user_password(
    state: &AppState,
    username: &str,
    password: &str,
) -> Result<UserRecord, AppError> {
    let user = find_user_by_username(state, username)
        .await?
        .ok_or(AppError::Unauthorized)?;
    verify_password(password, &user.password_hash).map_err(|_| AppError::Unauthorized)?;
    Ok(user)
}

pub async fn create_asset(
    state: &AppState,
    user_id: i64,
    name: &str,
    unit_value_cents: i64,
) -> Result<AssetRecord, AppError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::InvalidInput(
            "nome do ativo nao pode ser vazio".to_string(),
        ));
    }
    if unit_value_cents <= 0 {
        return Err(AppError::InvalidInput(
            "valor unitario deve ser positivo".to_string(),
        ));
    }

    let mut memory = state.memory.lock().map_err(|_| AppError::MemoryPoisoned)?;
    if memory
        .assets
        .iter()
        .any(|asset| asset.user_id == user_id && asset.name.eq_ignore_ascii_case(name))
    {
        return Err(AppError::Conflict(
            "ativo ja cadastrado para esse usuario".to_string(),
        ));
    }

    let asset = AssetRecord {
        id: memory.asset_id(),
        user_id,
        name: name.to_string(),
        unit_value_cents,
    };
    memory.assets.push(asset.clone());
    Ok(asset)
}

pub async fn create_purchase(
    state: &AppState,
    user_id: i64,
    asset_id: i64,
    quantity_milli: i64,
    bought_for_cents: i64,
) -> Result<(), AppError> {
    if quantity_milli <= 0 {
        return Err(AppError::InvalidInput(
            "quantidade deve ser positiva".to_string(),
        ));
    }
    if bought_for_cents <= 0 {
        return Err(AppError::InvalidInput(
            "valor de compra deve ser positivo".to_string(),
        ));
    }

    let mut memory = state.memory.lock().map_err(|_| AppError::MemoryPoisoned)?;

    if !memory
        .assets
        .iter()
        .any(|asset| asset.id == asset_id && asset.user_id == user_id)
    {
        return Err(AppError::NotFound);
    }

    let purchase = PurchaseRecord {
        id: memory.purchase_id(),
        asset_id,
        quantity_milli,
        bought_for_cents,
        purchase_date: "agora".to_string(),
    };

    memory.purchases.push(purchase);
    Ok(())
}

pub async fn portfolio_for_user(
    state: &AppState,
    user_id: i64,
) -> Result<Vec<AssetView>, AppError> {
    let memory = state.memory.lock().map_err(|_| AppError::MemoryPoisoned)?;
    let assets: Vec<AssetRecord> = memory
        .assets
        .iter()
        .filter(|asset| asset.user_id == user_id)
        .cloned()
        .collect();
    let valid_asset_ids: HashSet<i64> = assets.iter().map(|asset| asset.id).collect();
    let purchases: Vec<PurchaseRecord> = memory
        .purchases
        .iter()
        .filter(|purchase| valid_asset_ids.contains(&purchase.asset_id))
        .cloned()
        .collect();
    drop(memory);

    let mut view = Vec::with_capacity(assets.len());

    for asset in assets {
        let mut total_quantity_milli = 0_i64;
        let mut total_change_cents = 0_i64;
        let mut purchase_views = Vec::new();

        for purchase in purchases
            .iter()
            .filter(|purchase| purchase.asset_id == asset.id)
        {
            let change = change_cents(
                asset.unit_value_cents,
                purchase.bought_for_cents,
                purchase.quantity_milli,
            );
            total_quantity_milli += purchase.quantity_milli;
            total_change_cents += change;
            purchase_views.push(PurchaseView {
                id: purchase.id,
                purchase_date: purchase.purchase_date.clone(),
                quantity: format_quantity(purchase.quantity_milli),
                bought_for: format_money(purchase.bought_for_cents),
                change: format_signed_money(change),
                change_is_positive: change >= 0,
            });
        }

        view.push(AssetView {
            id: asset.id,
            name: asset.name,
            quantity: format_quantity(total_quantity_milli),
            unit_value: format_money(asset.unit_value_cents),
            change: format_signed_money(total_change_cents),
            change_is_positive: total_change_cents >= 0,
            purchases: purchase_views,
        });
    }

    Ok(view)
}
