mod lib;
mod export;
mod import;

pub use lib::construct_path;
pub use lib::ensuring_storage;
pub use export::export;
pub use import::import;

mod dot_parser;
pub trait ToCsv {
    fn to_csv(&self) -> String;
}