mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const WETH_TRACKED_CONTRACT: [u8; 20] = hex!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");

fn map_weth_events(blk: &eth::Block, events: &mut contract::Events) {
    events.weth_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::weth_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::WethApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            guy: event.guy,
                            src: event.src,
                            wad: event.wad.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.weth_deposits.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::weth_contract::events::Deposit::match_and_decode(log) {
                        return Some(contract::WethDeposit {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            dst: event.dst,
                            wad: event.wad.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.weth_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::weth_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::WethTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            dst: event.dst,
                            src: event.src,
                            wad: event.wad.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.weth_withdrawals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == WETH_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::weth_contract::events::Withdrawal::match_and_decode(log) {
                        return Some(contract::WethWithdrawal {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            src: event.src,
                            wad: event.wad.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}

fn db_weth_out(events: &contract::Events, tables: &mut DatabaseChangeTables) {
    // Loop over all the abis events to create table changes
    events.weth_approvals.iter().for_each(|evt| {
        tables
            .create_row("weth_approval", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("guy", Hex(&evt.guy).to_string())
            .set("src", Hex(&evt.src).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
    events.weth_deposits.iter().for_each(|evt| {
        tables
            .create_row("weth_deposit", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dst", Hex(&evt.dst).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
    events.weth_transfers.iter().for_each(|evt| {
        tables
            .create_row("weth_transfer", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dst", Hex(&evt.dst).to_string())
            .set("src", Hex(&evt.src).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
    events.weth_withdrawals.iter().for_each(|evt| {
        tables
            .create_row("weth_withdrawal", [("evt_tx_hash", evt.evt_tx_hash.to_string()),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("src", Hex(&evt.src).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
}


fn graph_weth_out(events: &contract::Events, tables: &mut EntityChangesTables) {
    // Loop over all the abis events to create table changes
    events.weth_approvals.iter().for_each(|evt| {
        tables
            .create_row("weth_approval", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("guy", Hex(&evt.guy).to_string())
            .set("src", Hex(&evt.src).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
    events.weth_deposits.iter().for_each(|evt| {
        tables
            .create_row("weth_deposit", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dst", Hex(&evt.dst).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
    events.weth_transfers.iter().for_each(|evt| {
        tables
            .create_row("weth_transfer", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("dst", Hex(&evt.dst).to_string())
            .set("src", Hex(&evt.src).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
    events.weth_withdrawals.iter().for_each(|evt| {
        tables
            .create_row("weth_withdrawal", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", &evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.as_ref().unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("src", Hex(&evt.src).to_string())
            .set("wad", BigDecimal::from_str(&evt.wad).unwrap());
    });
}

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_weth_events(&blk, &mut events);
    Ok(events)
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = DatabaseChangeTables::new();
    db_weth_out(&events, &mut tables);
    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize Database Changes container
    let mut tables = EntityChangesTables::new();
    graph_weth_out(&events, &mut tables);
    Ok(tables.to_entity_changes())
}
