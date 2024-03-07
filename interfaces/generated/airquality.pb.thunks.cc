#include "airquality.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// sensors.PMS7003Sensor
        // clang-format off
extern "C" {
void* __rust_proto_thunk__sensors_PMS7003Sensor_new() { return new ::sensors::PMS7003Sensor(); }
void __rust_proto_thunk__sensors_PMS7003Sensor_delete(void* ptr) { delete static_cast<::sensors::PMS7003Sensor*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__sensors_PMS7003Sensor_serialize(::sensors::PMS7003Sensor* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__sensors_PMS7003Sensor_deserialize(::sensors::PMS7003Sensor* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

float __rust_proto_thunk__sensors_PMS7003Sensor_get_pm2_5(::sensors::PMS7003Sensor* msg) { return msg->pm2_5(); }
void __rust_proto_thunk__sensors_PMS7003Sensor_set_pm2_5(::sensors::PMS7003Sensor* msg, float val) {
  msg->set_pm2_5(val);
}
void __rust_proto_thunk__sensors_PMS7003Sensor_clear_pm2_5(::sensors::PMS7003Sensor* msg) { msg->clear_pm2_5(); }
float __rust_proto_thunk__sensors_PMS7003Sensor_get_pm10(::sensors::PMS7003Sensor* msg) { return msg->pm10(); }
void __rust_proto_thunk__sensors_PMS7003Sensor_set_pm10(::sensors::PMS7003Sensor* msg, float val) {
  msg->set_pm10(val);
}
void __rust_proto_thunk__sensors_PMS7003Sensor_clear_pm10(::sensors::PMS7003Sensor* msg) { msg->clear_pm10(); }
const void* __rust_proto_thunk__sensors_PMS7003Sensor_get_measurement_time(::sensors::PMS7003Sensor* msg) {
  return static_cast<const void*>(&msg->measurement_time());
}

}  // extern "C"
// clang-format on


