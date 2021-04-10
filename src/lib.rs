#[cfg(feature = "database")]
#[macro_use]
extern crate diesel;

// Diesel schema
#[cfg(feature = "database")]
pub mod schema;

// Models
pub mod student;
