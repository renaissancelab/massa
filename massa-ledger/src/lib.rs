// Copyright (c) 2021 MASSA LABS <info@massa.net>

#![feature(map_first_last)]
#![feature(async_closure)]

#[macro_use]
extern crate massa_logging;

pub use config::LedgerConfig;
pub use error::LedgerError;
pub use ledger::{
    FinalLedger, LedgerChanges, LedgerEntry, LedgerEntryUpdate, SetOrDelete, SetOrKeep,
    SetUpdateOrDelete,
};

mod config;
mod error;
mod ledger;

#[cfg(test)]
mod tests;
