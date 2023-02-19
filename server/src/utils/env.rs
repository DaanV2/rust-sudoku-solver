use std::env;

pub fn get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("{}: {}", key, e),
    }
}

pub fn get_env_var_default(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}

pub fn str_to_bool(s: String) -> bool {
    match s.as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("{}: not a boolean", s),
    }
}

pub fn str_to_u32(s: String) -> u32 {
    match s.parse::<u32>() {
        Ok(val) => val,
        Err(e) => panic!("{}: {}", s, e),
    }
}
