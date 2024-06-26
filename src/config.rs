use once_cell::sync::Lazy;


pub struct Config {
    pub api_key: String,

    pub encrypt_salt: String,
    pub secret_ket: String,

    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: u32,
    pub postgres_db: String,
}


fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}


impl Config {
    pub fn load() -> Config {
        Config {
            api_key: get_env("API_KEY"),

            encrypt_salt: get_env("ENCRYPT_SALT"),
            secret_ket: get_env("SECRET_KEY"),

            postgres_user: get_env("POSTGRES_USER"),
            postgres_password: get_env("POSTGRES_PASSWORD"),
            postgres_host: get_env("POSTGRES_HOST"),
            postgres_port: get_env("POSTGRES_PORT").parse().unwrap(),
            postgres_db: get_env("POSTGRES_DB"),
        }
    }
}


pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);
