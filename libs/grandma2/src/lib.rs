mod client;
pub mod interface;
mod types;

mod error;

pub use error::{Ma2Error, Result};
pub use interface::GrandMa2;
pub use types::{Executor, FaderExecutor, ButtonExecutor};