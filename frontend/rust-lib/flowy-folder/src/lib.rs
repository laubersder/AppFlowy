pub mod entities;
pub mod event_map;
pub mod services;

#[macro_use]
mod macros;

#[macro_use]
extern crate flowy_sqlite;

pub mod manager;
mod notification;
pub mod protobuf;
mod util;

pub mod prelude {
  pub use crate::{errors::*, event_map::*};
}

pub mod errors {
  pub use flowy_error::{internal_error, ErrorCode, FlowyError, FlowyResult};
}
