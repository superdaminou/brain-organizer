mod lib;
mod export;
mod import;

pub use lib::construct_path;
pub use lib::ensuring_storage;
pub use export::export;
pub use import::import;
pub use export::ModeExport;


pub trait ToCsv {
    fn to_csv(&self) -> String;
}


pub trait ToRis {
    fn to_ris(&self) -> String;
}