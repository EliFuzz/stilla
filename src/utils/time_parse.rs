use std::time::Duration;

pub fn time_parse(s: &str) -> Duration {
    let period = s.trim_start_matches(|c: char| c.is_ascii_digit());
    let time = s[..s.len() - period.len()].parse().unwrap_or(1);
    Duration::from_secs(match period.chars().next() {
        Some('s') => time,
        Some('m') => time * 60,
        Some('h') => time * 3600,
        Some('d') => time * 86400,
        _ => 60,
    })
}
