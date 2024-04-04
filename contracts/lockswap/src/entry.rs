// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::{load_cell_capacity, load_cell_data, load_cell_lock, load_script, QueryIter},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let maker_lockscript: Bytes = script.args().unpack();

    let data = load_cell_data(0, Source::GroupInput)?;
    if data.len() < 24 {
        return Err(Error::InsufficientCellDataSize);
    }
    let order_ckb = u64::from_le_bytes(data[16..24].try_into().unwrap());

    // check `maker_lockscript` appeared in tx's outputs
    let find_taker = QueryIter::new(load_cell_lock, Source::Output)
        .enumerate()
        .any(|(i, lock_script)| {
            let offer_ckb = load_cell_capacity(i, Source::Output).unwrap();
            lock_script.as_bytes() == maker_lockscript && order_ckb <= offer_ckb
        });
    if !find_taker {
        return Err(Error::InvalidTakerFormat);
    }

    Ok(())
}
