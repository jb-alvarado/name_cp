pub mod args_parse;
pub mod file;
pub mod logging;

pub fn is_close(a: f64, b: f64, to: f64) -> bool {
    (a - b).abs() < to
}
