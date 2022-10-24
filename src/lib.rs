mod abi;
mod pb;
use hex_literal::hex;
use substreams::{log, store::StoreAddInt64, Hex};
use substreams_ethereum::{pb::eth::v2 as eth};
use crate::pb::ens;

// ETH Registrar Controller Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("283Af0B28c62C092C9727F1Ee09c02CA627EB7F5");

substreams_ethereum::init!();

/// Extract Registration Events
#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<ens::Registrations, substreams::errors::Error> {
    Ok(ens::Registrations {
        registrations: blk
            .events::<abi::ens::functions::RegisterWithConfig>(&[&TRACKED_CONTRACT])
            .map(|(registration, log)| {
                substreams::log::info!("ENS registration seen");

                ens::Registration {
                    trx_hash: log.receipt.transaction.hash.clone(),
                    from: registration.owner,
                    ens: registration.name,
                }
            })
            .collect(),
    })
}