#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pms7003SensorMeasurement {
    #[prost(float, tag = "1")]
    pub pm2_5: f32,
    #[prost(float, tag = "2")]
    pub pm10: f32,
    #[prost(message, optional, tag = "3")]
    pub measurement_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DummySensor {
    #[prost(string, tag = "1")]
    pub fake_payload: ::prost::alloc::string::String,
}
