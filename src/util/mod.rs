pub mod interaction;
pub mod option;
pub mod random;

pub fn seconds_pretty(seconds: i64) -> String {
    let s = seconds % 60;
    let m = (seconds / 60) % 60;
    let h = seconds / 3600;

    let mut hms = String::new();

    if h > 0 {
        hms += &(h.to_string() + "h ");
    }

    if m > 0 {
        hms += &(m.to_string() + "m ");
    }

    if s > 0 {
        hms += &(s.to_string() + "s ");
    }

    hms
}