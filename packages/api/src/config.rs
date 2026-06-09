use std::env;

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_jwt_secret: String,
    pub razorpay_key_id: String,
    pub razorpay_key_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            supabase_url: env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
            supabase_anon_key: env::var("SUPABASE_ANON_KEY")
                .expect("SUPABASE_ANON_KEY must be set"),
            supabase_jwt_secret: env::var("SUPABASE_JWT_SECRET")
                .expect("SUPABASE_JWT_SECRET must be set"),
            razorpay_key_id: env::var("RAZORPAY_KEY_ID").unwrap_or_default(),
            razorpay_key_secret: env::var("RAZORPAY_KEY_SECRET").unwrap_or_default(),
        }
    }
}

pub fn get() -> Config {
    static CONFIG: std::sync::OnceLock<Config> = std::sync::OnceLock::new();
    CONFIG.get_or_init(Config::new)
}
