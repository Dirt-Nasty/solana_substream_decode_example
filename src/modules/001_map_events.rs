use crate::idl::message_transmitter::Instruction;
use crate::pb::program::v1 as program;
use crate::pb::sf::solana::transactions::v1::Output;
use message_transmitter::instruction as idl;
use prost_types::Timestamp;

#[substreams::handlers::map]
fn map_events(blk: Output) -> Result<program::Instructions, substreams::errors::Error> {

    let program_address = "CCTPmbSD7gX1bxKPAmg77w8oFzNFpaQiQUWD43TKaecd".to_string();

    let mut events = program::Instructions::default();
    
    events.message_transmitter_receive_messages.append(&mut blk
        .data
        .iter()
        .flat_map(|tx_stat| {
            tx_stat.instructions.iter()
            .flat_map(|inst| {
                let mut events = vec![];
    
                if inst.executing_account == program_address {
                    if let Some(event) = idl::ReceiveMessage::match_and_decode(&inst.data) {
                        events.push(program::MessageTransmitterReceiveMessage {
                            evt_tx_hash: tx_stat.id.clone(),
                            evt_index: tx_stat.index,
                            evt_block_time: Some(Timestamp { seconds: tx_stat.block_time as i64, nanos: 0 }),
                            evt_block_number: tx_stat.block_slot as u64,
                            tx_gas_fee: tx_stat.fees.to_string(),
                            tx_caller: inst.account_arguments[1].clone(),
                            message: event.params.message,
                            attestation: event.params.attestation
                        });
                    }
                }
    
                events.extend(inst.inner_instructions.iter()
                    .filter(|inner_inst| inner_inst.executing_account == program_address)
                    .filter_map(|inner_inst| {
                        if let Some(event) = idl::ReceiveMessage::match_and_decode(&inner_inst.data) {
                            return Some(program::MessageTransmitterReceiveMessage {
                                evt_tx_hash: tx_stat.id.clone(),
                                evt_index: tx_stat.index,
                                evt_block_time: Some(Timestamp { seconds: tx_stat.block_time as i64, nanos: 0 }),
                                evt_block_number: tx_stat.block_slot as u64,
                                tx_gas_fee: tx_stat.fees.to_string(),
                                tx_caller: inst.account_arguments[1].clone(),
                                message: event.params.message,
                                attestation: event.params.attestation
                            });
                        }
                        None
                    })
                );
    
                events
            })
        })
        .collect());

    events.message_transmitter_send_messages.append(&mut blk
        .data
        .iter()
        .flat_map(|tx_stat| {
            tx_stat.instructions.iter()
            .flat_map(|inst| {
                let mut events = vec![];
    
                // Process outer instructions
                if inst.executing_account == program_address {
                    if let Some(event) = idl::SendMessage::match_and_decode(&inst.data) {
                        events.push(program::MessageTransmitterSendMessage {
                            evt_tx_hash: tx_stat.id.clone(),
                            evt_index: tx_stat.index,
                            evt_block_time: Some(Timestamp { seconds: tx_stat.block_time as i64, nanos: 0 }),
                            evt_block_number: tx_stat.block_slot as u64,
                            destination_domain: event.params.destination_domain,
                            recipient: vec![],
                            message_body: event.params.message_body.into(),
                            tx_gas_fee: tx_stat.fees.to_string(),
                            tx_caller: inst.account_arguments[1].clone()
                        });
                    }
                }
    
                // Process inner instructions
                events.extend(inst.inner_instructions.iter()
                    .filter(|inner_inst| inner_inst.executing_account == program_address)
                    .filter_map(|inner_inst| {
                        if let Some(event) = idl::SendMessage::match_and_decode(&inner_inst.data) {
                            return Some(program::MessageTransmitterSendMessage {
                                evt_tx_hash: tx_stat.id.clone(),
                                evt_index: tx_stat.index,
                                evt_block_time: Some(Timestamp { seconds: tx_stat.block_time as i64, nanos: 0 }),
                                evt_block_number: tx_stat.block_slot as u64,
                                destination_domain: event.params.destination_domain,
                                recipient: event.params.recipient.to_bytes().to_vec(),
                                message_body: event.params.message_body.into(),
                                tx_gas_fee: tx_stat.fees.to_string(),
                                tx_caller: inst.account_arguments[1].clone()
                            });
                        }
                        None
                    })
                );
    
                events
            })
        })
        .collect());

    events.message_transmitter_send_message_with_callers.append(&mut blk
        .data
        .iter()
        .flat_map(|tx_stat| {
            tx_stat.instructions.iter()
            .flat_map(|inst| {
                let mut events = vec![];
    
                if inst.executing_account == program_address {
                    if let Some(event) = idl::SendMessageWithCaller::match_and_decode(&inst.data) {
                        events.push(program::MessageTransmitterSendMessageWithCaller {
                            evt_tx_hash: tx_stat.id.clone(),
                            evt_index: tx_stat.index,
                            evt_block_time: Some(Timestamp { seconds: tx_stat.block_time as i64, nanos: 0 }),
                            evt_block_number: tx_stat.block_slot as u64,
                            tx_gas_fee: tx_stat.fees.to_string(),
                            tx_caller: inst.account_arguments[1].clone(),
                            destination_domain: event.params.destination_domain,
                            recipient: event.params.recipient.to_bytes().to_vec(),
                            message_body: event.params.message_body.into(),
                            destination_caller: event.params.destination_caller.to_bytes().to_vec()
                        });
                    }
                }
    
                events.extend(inst.inner_instructions.iter()
                    .filter(|inner_inst| inner_inst.executing_account == program_address)
                    .filter_map(|inner_inst| {
                        if let Some(event) = idl::SendMessageWithCaller::match_and_decode(&inner_inst.data) {
                            return Some(program::MessageTransmitterSendMessageWithCaller {
                                evt_tx_hash: tx_stat.id.clone(),
                                evt_index: tx_stat.index,
                                evt_block_time: Some(Timestamp { seconds: tx_stat.block_time as i64, nanos: 0 }),
                                evt_block_number: tx_stat.block_slot as u64,
                                tx_gas_fee: tx_stat.fees.to_string(),
                                tx_caller: inst.account_arguments[1].clone(),
                                destination_domain: event.params.destination_domain,
                                recipient: event.params.recipient.to_bytes().to_vec(),
                                message_body: event.params.message_body.into(),
                                destination_caller: event.params.destination_caller.to_bytes().to_vec()
                            });
                        }
                        None
                    })
                );
    
                events
            })
        })
        .collect());

    Ok(events)
}