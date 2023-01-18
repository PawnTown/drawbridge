use serde_json::Value;
use crate::storage;

pub fn get_settings() -> Option<Value> {
    let data_res = storage::load("settings".to_string());
    match data_res {
        Ok(data) => {
            let settings: Value = serde_json::from_str(&data).unwrap();
            return Some(settings);
        },

        Err(_) => {
            return None;
        }
    }
}

pub fn get_setting_str(key: String) -> Option<String> {
    match get_settings() {
        Some(settings) => {
            let val = settings[key].as_str();
            return Some(val.unwrap().to_string());
        },

        None => {
            return None;
        }
    }
}

pub fn get_setting_bool(key: String) -> Option<bool> {
    match get_settings() {
        Some(settings) => {
            let val = settings[key].as_bool();
            return Some(val.unwrap());
        },

        None => {
            return None;
        }
    }
}