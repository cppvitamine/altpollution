#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pms7003SensorMeasurement {
    #[prost(int32, tag = "1")]
    pub pm1_c_0: i32,
    #[prost(int32, tag = "2")]
    pub pm2_c_5: i32,
    #[prost(int32, tag = "3")]
    pub pm10: i32,
    #[prost(int32, tag = "4")]
    pub pm1_c_0_atm: i32,
    #[prost(int32, tag = "5")]
    pub pm2_c_5_atm: i32,
    #[prost(int32, tag = "6")]
    pub pm10_atm: i32,
}
