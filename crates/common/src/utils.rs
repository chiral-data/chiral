//! Utilities
//! 

const ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

pub fn generate_id(length: usize) -> String {
    nanoid::nanoid!(length, &ALPHABET)
}

pub fn get_env_var(env_var_name: &str) -> String {
    std::env::var_os(env_var_name).unwrap().to_string_lossy().to_string()
}