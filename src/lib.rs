// Genesis Core Library Entry Point
// All main modules (PoF, PoI, PoP, Spatial) are initialized here.

pub mod pof;
pub mod poi;
pub mod pop;
pub mod spatial;
pub mod tee;
pub mod utils;

pub fn init() {
    println!("🚀 Genesis Core initialized.");
}
