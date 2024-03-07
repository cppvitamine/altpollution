extern crate protobuf_cpp as __pb;
extern crate std as __std;
pub mod sensors {
#[allow(non_camel_case_types)]
// TODO: Implement support for debug redaction
#[derive(Debug)]
pub struct PMS7003Sensor {
  inner: ::__pb::__runtime::MessageInner
}

// SAFETY:
// - `PMS7003Sensor` does not provide shared mutation with its arena.
// - `PMS7003SensorMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena that would conflict with
//   field access is impossible.
unsafe impl Sync for PMS7003Sensor {}

impl ::__pb::Proxied for PMS7003Sensor {
  type View<'a> = PMS7003SensorView<'a>;
  type Mut<'a> = PMS7003SensorMut<'a>;
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct PMS7003SensorView<'a> {
  msg: ::__pb::__internal::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'a ()>,
}

impl<'a> PMS7003SensorView<'a> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__internal::RawMessage) -> Self {
    Self { msg, _phantom: std::marker::PhantomData }
  }
  pub fn r#pm2_5(&self) -> f32 { unsafe {
    __rust_proto_thunk__sensors_PMS7003Sensor_get_pm2_5(self.msg)
  } }

  pub fn r#pm10(&self) -> f32 { unsafe {
    __rust_proto_thunk__sensors_PMS7003Sensor_get_pm10(self.msg)
  } }

}

// SAFETY:
// - `PMS7003SensorView` does not perform any mutation.
// - While a `PMS7003SensorView` exists, a `PMS7003SensorMut` can't exist to mutate
//   the arena that would conflict with field access.
// - `PMS7003SensorMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PMS7003SensorView<'_> {}
unsafe impl Send for PMS7003SensorView<'_> {}

impl<'a> ::__pb::ViewProxy<'a> for PMS7003SensorView<'a> {
  type Proxied = PMS7003Sensor;

  fn as_view(&self) -> ::__pb::View<'a, PMS7003Sensor> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PMS7003Sensor> where 'a: 'shorter {
    self
  }
}

impl<'a> ::__pb::SettableValue<PMS7003Sensor> for PMS7003SensorView<'a> {
  fn set_on(self, _private: ::__pb::__internal::Private, _mutator: ::__pb::Mut<PMS7003Sensor>) {
    todo!()
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PMS7003SensorMut<'a> {
  inner: ::__pb::__runtime::MutatorMessageRef<'a>,
}

// SAFETY:
// - `PMS7003SensorMut` does not perform any shared mutation.
// - `PMS7003SensorMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PMS7003SensorMut<'_> {}

impl<'a> ::__pb::MutProxy<'a> for PMS7003SensorMut<'a> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, PMS7003Sensor> {
    PMS7003SensorMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, PMS7003Sensor> where 'a : 'shorter { self }
}

impl<'a> ::__pb::ViewProxy<'a> for PMS7003SensorMut<'a> {
  type Proxied = PMS7003Sensor;
  fn as_view(&self) -> ::__pb::View<'_, PMS7003Sensor> {
    PMS7003SensorView { msg: self.inner.msg(), _phantom: std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PMS7003Sensor> where 'a: 'shorter {
    PMS7003SensorView { msg: self.inner.msg(), _phantom: std::marker::PhantomData }
  }
}

impl PMS7003Sensor {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__sensors_PMS7003Sensor_new() } } }
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__sensors_PMS7003Sensor_serialize(self.inner.msg) }
  }
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__sensors_PMS7003Sensor_deserialize(self.inner.msg, data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }

  // pm2_5: optional float
  pub fn r#pm2_5(&self) -> f32 {
    unsafe { __rust_proto_thunk__sensors_PMS7003Sensor_get_pm2_5(self.inner.msg) }
  }
  pub fn r#pm2_5_mut(&mut self) -> ::__pb::PrimitiveMut<'_, f32> {
    static VTABLE: ::__pb::__internal::PrimitiveVTable<f32> =
      ::__pb::__internal::PrimitiveVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__sensors_PMS7003Sensor_get_pm2_5,
        __rust_proto_thunk__sensors_PMS7003Sensor_set_pm2_5,
      );

      ::__pb::PrimitiveMut::from_inner(
        ::__pb::__internal::Private,
        unsafe {
          ::__pb::__internal::RawVTableMutator::new(
            ::__pb::__internal::Private,
            ::__pb::__runtime::MutatorMessageRef::new(
              ::__pb::__internal::Private, &mut self.inner
            ),
            &VTABLE,
          )
        },
      )
  }

  // pm10: optional float
  pub fn r#pm10(&self) -> f32 {
    unsafe { __rust_proto_thunk__sensors_PMS7003Sensor_get_pm10(self.inner.msg) }
  }
  pub fn r#pm10_mut(&mut self) -> ::__pb::PrimitiveMut<'_, f32> {
    static VTABLE: ::__pb::__internal::PrimitiveVTable<f32> =
      ::__pb::__internal::PrimitiveVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__sensors_PMS7003Sensor_get_pm10,
        __rust_proto_thunk__sensors_PMS7003Sensor_set_pm10,
      );

      ::__pb::PrimitiveMut::from_inner(
        ::__pb::__internal::Private,
        unsafe {
          ::__pb::__internal::RawVTableMutator::new(
            ::__pb::__internal::Private,
            ::__pb::__runtime::MutatorMessageRef::new(
              ::__pb::__internal::Private, &mut self.inner
            ),
            &VTABLE,
          )
        },
      )
  }

  // measurement_time: optional message google.protobuf.Timestamp
  pub fn r#measurement_time(&self) -> crate::google::protobuf::TimestampView {
    // For C++ kernel, getters automatically return the
    // default_instance if the field is unset.
    let submsg = unsafe { __rust_proto_thunk__sensors_PMS7003Sensor_get_measurement_time(self.inner.msg) };
    crate::google::protobuf::TimestampView::new(::__pb::__internal::Private, submsg)
  }


}  // impl PMS7003Sensor

impl ::__std::ops::Drop for PMS7003Sensor {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__sensors_PMS7003Sensor_delete(self.inner.msg); }
  }
}

extern "C" {
  fn __rust_proto_thunk__sensors_PMS7003Sensor_new() -> ::__pb::__internal::RawMessage;
  fn __rust_proto_thunk__sensors_PMS7003Sensor_delete(raw_msg: ::__pb::__internal::RawMessage);
  fn __rust_proto_thunk__sensors_PMS7003Sensor_serialize(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__sensors_PMS7003Sensor_deserialize(raw_msg: ::__pb::__internal::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;

  fn __rust_proto_thunk__sensors_PMS7003Sensor_get_pm2_5(raw_msg: ::__pb::__internal::RawMessage) -> f32;
  fn __rust_proto_thunk__sensors_PMS7003Sensor_set_pm2_5(raw_msg: ::__pb::__internal::RawMessage, val: f32);
  fn __rust_proto_thunk__sensors_PMS7003Sensor_clear_pm2_5(raw_msg: ::__pb::__internal::RawMessage);

  fn __rust_proto_thunk__sensors_PMS7003Sensor_get_pm10(raw_msg: ::__pb::__internal::RawMessage) -> f32;
  fn __rust_proto_thunk__sensors_PMS7003Sensor_set_pm10(raw_msg: ::__pb::__internal::RawMessage, val: f32);
  fn __rust_proto_thunk__sensors_PMS7003Sensor_clear_pm10(raw_msg: ::__pb::__internal::RawMessage);

  fn __rust_proto_thunk__sensors_PMS7003Sensor_get_measurement_time(raw_msg: ::__pb::__internal::RawMessage) -> ::__pb::__internal::RawMessage;


}  // extern "C" for PMS7003Sensor


impl PMS7003Sensor {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__internal::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(&mut self) -> ::__pb::__internal::RawMessage {
    self.inner.msg
  }
}

} // mod sensors
