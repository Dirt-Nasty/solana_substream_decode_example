// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration="MessageType", tag="1")]
    pub message_type: i32,
    #[prost(string, tag="2")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub block_number: u64,
    #[prost(string, tag="5")]
    pub caller: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub source_domain: u64,
    #[prost(uint64, tag="7")]
    pub destination_domain: u64,
    #[prost(uint64, tag="8")]
    pub nonce: u64,
    #[prost(string, tag="9")]
    pub sender: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub raw_message_body: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="11")]
    pub decoded_message_body: ::core::option::Option<MessageBody>,
    #[prost(string, tag="12")]
    pub tx_fee: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub tx_caller: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageBody {
    #[prost(uint32, tag="1")]
    pub version: u32,
    #[prost(string, tag="2")]
    pub burn_token: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint_recipient: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub message_sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Messages {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    Sent = 0,
    Recieved = 1,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::Sent => "Sent",
            MessageType::Recieved => "Recieved",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Sent" => Some(Self::Sent),
            "Recieved" => Some(Self::Recieved),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
