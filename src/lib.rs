#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub use crate::ashtadhyayi::{Ashtadhyayi, AshtadhyayiBuilder};
pub use crate::prakriya::{Prakriya, Rule, RuleChoice, Step};

// Public modules.
// - `args` defines the API contract.
// - `dhatupatha` defines convenience functions for reading our version of the Dhatupatha.
//   These functions are used only in our binaries (in `src/bin`).
pub mod args;
pub mod dhatupatha;

// Data structures
mod char_view;
mod prakriya;
mod sounds;
mod tag;
mod term;

// Utility functions
mod filters;
mod operators;

// Rules
mod abhyasasya;
mod ac_sandhi;
mod angasya;
mod ardhadhatuka;
mod ashtadhyayi;
mod asiddhavat;
mod atidesha;
mod atmanepada;
mod dhatu_gana;
mod dhatu_karya;
mod dvitva;
mod guna_vrddhi;
mod it_agama;
mod it_samjna;
mod la_karya;
mod pratipadika_karya;
mod samjna;
mod samprasarana;
mod sanadi;
mod stem_gana;
mod sup_adesha;
mod sup_karya;
mod tin_pratyaya;
mod tripadi;
mod vikarana;
