pub mod app;
pub mod components;
pub mod data;

// Imports WASM seulement côté client
#[cfg(target_arch = "wasm32")]
use leptos::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

// Fonction d'hydratation côté client SEULEMENT
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn hydrate() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    log::info!("🚀 Solana Dashboard - Hydratation côté client...");

    leptos::mount_to_body(app::App);
}

// Export pour SSR
#[cfg(feature = "ssr")]
pub use app::App;