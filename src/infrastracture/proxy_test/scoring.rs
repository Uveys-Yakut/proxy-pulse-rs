use crate::core::domain::ProxyAnonymity;

fn latency_score(latency_ms: u128) -> f64 {
    match latency_ms {
        0..=200 => 100.0,
        201..=400 => 90.0,
        401..=600 => 75.0,
        601..=900 => 55.0,
        901..=1300 => 35.0,
        1301..=2000 => 20.0,
        _ => 5.0,
    }
}

fn reliability_score(retries: u8) -> f64 {
    match retries {
        0 => 100.0,
        1 => 80.0,
        2 => 55.0,
        3 => 30.0,
        _ => 10.0,
    }
}

fn anonymity_score(a: &ProxyAnonymity, latency_ms: u128) -> f64 {
    let base = match a {
        ProxyAnonymity::Elite => 100.0,
        ProxyAnonymity::Anonymous => 70.0,
        ProxyAnonymity::Transparent => 30.0,
    };

    let latency_factor = match latency_ms {
        0..=500 => 1.0,
        501..=1000 => 0.85,
        1001..=2000 => 0.65,
        _ => 0.45,
    };

    base * latency_factor
}

pub fn calculate_score(latency_ms: u128, retries: u8, anonymity: &ProxyAnonymity) -> u8 {
    let latency = latency_score(latency_ms);
    let reliability = reliability_score(retries);
    let anonymity = anonymity_score(anonymity, latency_ms);

    let final_score = latency * 0.45 + reliability * 0.30 + anonymity * 0.25;

    final_score.round().clamp(1.0, 100.0) as u8
}
