use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
pub struct Giveaway {
    pub gleam_id: String,
    pub entry_count: Option<u64>,
    pub start_date: u64,
    pub end_date: u64,
    pub update_date: u64,
    pub name: String,
    pub description: String,
}

impl Giveaway {
    /// Return the url
    pub fn get_url(&self) -> String {
        format!("https://gleam.io/{}/-", self.gleam_id)
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct MeiliResponse {
    pub hits: Vec<Giveaway>,
    pub offset: usize,
    pub limit: usize,
    pub processingTimeMs: usize,
    pub query: String
}

#[wasm_bindgen(module = "/src/js.js")]
extern "C" {
    fn get_timestamp_js() -> u32;
}

pub fn get_timestamp() -> u64 {
    get_timestamp_js() as u64
}

pub fn seconds_to_string(mut seconds: i64, compact: bool) -> String {
    let ago = seconds < 0;

    let mut remaining = seconds%86400;
    let mut days = (seconds - remaining) / 86400;
    seconds = remaining;
    remaining = seconds%3600;
    let mut hours = (seconds - remaining) / 3600;
    seconds = remaining;
    remaining = seconds%60;
    let mut minutes = (seconds - remaining) / 60;
    seconds = remaining;

    if ago {
        days *= -1;
        hours *= -1;
        minutes *= -1;
        seconds *= -1;
    }

    let mut result = String::new();
    if days != 0 {
        result.push_str(&days.to_string());
        result.push_str(" days");
        if hours != 0 && !compact {
            result.push_str(" and ");
            result.push_str(&hours.to_string());
            result.push_str(" hours");
        }
    } else if hours != 0 {
        result.push_str(&hours.to_string());
        result.push_str(" hours");
        if minutes != 0 && !compact {
            result.push_str(" and ");
            result.push_str(&minutes.to_string());
            result.push_str(" minutes");
        }
    } else if minutes != 0 {
        result.push_str(&minutes.to_string());
        result.push_str(" minutes");
        if seconds != 0 && !compact {
            result.push_str(" and ");
            result.push_str(&seconds.to_string());
            result.push_str(" seconds");
        }
    } else if seconds != 0 {
        result.push_str(&seconds.to_string());
        result.push_str(" seconds");
    } else {
        result.push_str("now");
    }

    if ago {
        result.push_str(" ago");
    }

    result
}