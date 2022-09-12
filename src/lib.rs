pub(crate) mod cli_display;
pub mod error;
pub mod screens;
/// make public to crate only later
pub mod terminal;
pub(crate) mod text;
pub mod debug;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
