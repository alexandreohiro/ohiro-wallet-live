use askama::Template;

use crate::models::AssetView;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: String,
    pub has_error: bool,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub error: String,
    pub has_error: bool,
}

#[derive(Template)]
#[template(path = "assets.html")]
pub struct AssetsTemplate {
    pub user_name: String,
    pub assets: Vec<AssetView>,
    pub error: String,
    pub has_error: bool,
}
