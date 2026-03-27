#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec, Map, String,
};

// =========================
// DATA STRUCTURES
// =========================

#[contracttype]
#[derive(Clone)]
pub struct ColorProfile {
    pub skin_tone: String,
    pub undertone: String,
    pub eye_color: String,
    pub hair_color: String,
    pub recommended_palette: Vec<String>,
}

#[contracttype]
#[derive(Clone)]
pub struct PaletteRating {
    pub palette_name: String,
    pub rating: u32, // 1–5
}

// Storage keys
#[contracttype]
pub enum DataKey {
    Profile(Address),
    Ratings(Address),
}

// =========================
// CONTRACT
// =========================

#[contract]
pub struct ColorTestContract;

#[contractimpl]
impl ColorTestContract {

    // -------------------------
    // SAVE USER PROFILE
    // -------------------------
    pub fn set_profile(
        env: Env,
        user: Address,
        skin_tone: String,
        undertone: String,
        eye_color: String,
        hair_color: String,
        recommended_palette: Vec<String>,
    ) {
        user.require_auth();

        let profile = ColorProfile {
            skin_tone,
            undertone,
            eye_color,
            hair_color,
            recommended_palette,
        };

        env.storage().instance().set(&DataKey::Profile(user), &profile);
    }

    // -------------------------
    // GET USER PROFILE
    // -------------------------
    pub fn get_profile(env: Env, user: Address) -> Option<ColorProfile> {
        env.storage().instance().get(&DataKey::Profile(user))
    }

    // -------------------------
    // RATE A PALETTE
    // -------------------------
    pub fn rate_palette(
        env: Env,
        user: Address,
        palette_name: String,
        rating: u32,
    ) {
        user.require_auth();

        if rating < 1 || rating > 5 {
            panic!("Rating must be between 1 and 5");
        }

        let key = DataKey::Ratings(user.clone());

        let mut ratings: Vec<PaletteRating> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        ratings.push_back(PaletteRating {
            palette_name,
            rating,
        });

        env.storage().instance().set(&key, &ratings);
    }

    // -------------------------
    // GET USER RATINGS
    // -------------------------
    pub fn get_ratings(env: Env, user: Address) -> Vec<PaletteRating> {
        env.storage()
            .instance()
            .get(&DataKey::Ratings(user))
            .unwrap_or(Vec::new(&env))
    }

    // -------------------------
    // DELETE PROFILE (optional)
    // -------------------------
    pub fn delete_profile(env: Env, user: Address) {
        user.require_auth();
        env.storage().instance().remove(&DataKey::Profile(user.clone()));
        env.storage().instance().remove(&DataKey::Ratings(user));
    }
}
