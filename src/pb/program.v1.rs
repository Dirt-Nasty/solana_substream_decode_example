// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instructions {
    #[prost(message, repeated, tag="1")]
    pub message_transmitter_send_messages: ::prost::alloc::vec::Vec<MessageTransmitterSendMessage>,
    #[prost(message, repeated, tag="2")]
    pub message_transmitter_send_message_with_callers: ::prost::alloc::vec::Vec<MessageTransmitterSendMessageWithCaller>,
    #[prost(message, repeated, tag="3")]
    pub message_transmitter_receive_messages: ::prost::alloc::vec::Vec<MessageTransmitterReceiveMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageTransmitterSendMessage {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub tx_gas_fee: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub tx_caller: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub destination_domain: u32,
    #[prost(bytes="vec", tag="8")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub message_body: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageTransmitterSendMessageWithCaller {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub tx_gas_fee: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub tx_caller: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub destination_domain: u32,
    #[prost(bytes="vec", tag="8")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub message_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub destination_caller: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageTransmitterReceiveMessage {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub tx_gas_fee: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub tx_caller: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub attestation: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
