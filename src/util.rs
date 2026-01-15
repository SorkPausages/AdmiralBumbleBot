use std::env;

use rand::Rng;

pub fn get_id_from_env(key: &str) -> u64 {
    env::var(key)
        .unwrap_or_else(|_| panic!("{}", format!("Error getting {} from env", key).to_string()))
        .parse()
        .unwrap_or_else(|_| panic!("{}", format!("Error parsing {} from env", key).to_string()))
}

pub fn roll_dice(notation: &str) -> Result<u32, String> {
    let s = notation.to_lowercase();

    let (count_str, sides_str) = s
        .split_once('d')
        .ok_or_else(|| "Invalid format: missing 'd'".to_string())?;

    let count: u32 = if count_str.is_empty() {
        1
    } else {
        count_str
            .parse()
            .map_err(|_| "Invalid number of dice".to_string())?
    };

    let sides: u32 = sides_str
        .parse()
        .map_err(|_| "Invalid number of sides".to_string())?;

    if sides == 0 {
        return Err("Dice cannot have 0 sides".to_string());
    }

    let mut rng = rand::thread_rng();
    let mut total = 0;

    for _ in 0..count {
        total += rng.gen_range(1..=sides);
    }

    Ok(total)
}
