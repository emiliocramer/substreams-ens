// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Registrations {
    #[prost(message, repeated, tag="1")]
    pub registrations: ::prost::alloc::vec::Vec<Registration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Registration {
    #[prost(bytes="vec", tag="1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub trx_hash: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
