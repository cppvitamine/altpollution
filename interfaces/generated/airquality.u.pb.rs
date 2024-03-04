extern crate protobuf_upb as __pb;
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
    sensors_PMS7003Sensor_pm2_5(self.msg)
  } }

  pub fn r#pm10(&self) -> f32 { unsafe {
    sensors_PMS7003Sensor_pm10(self.msg)
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
    let arena = ::__pb::__runtime::Arena::new();
    Self {
      inner: ::__pb::__runtime::MessageInner {
        msg: unsafe { sensors_PMS7003Sensor_new(arena.raw()) },
        arena,
      }
    }
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    let arena = ::__pb::__runtime::Arena::new();
    let mut len = 0;
    unsafe {
      let data = sensors_PMS7003Sensor_serialize(self.inner.msg, arena.raw(), &mut len);
      ::__pb::__runtime::SerializedData::from_raw_parts(arena, data, len)
    }
  }
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let arena = ::__pb::__runtime::Arena::new();
    let msg = unsafe {
      sensors_PMS7003Sensor_parse(data.as_ptr(), data.len(), arena.raw())
    };

    match msg {
      None => Err(::__pb::ParseError),
      Some(msg) => {
        // This assignment causes self.arena to be dropped and to deallocate
        // any previous message pointed/owned to by self.inner.msg.
        self.inner.arena = arena;
        self.inner.msg = msg;
        Ok(())
      }
    }
  }

  // pm2_5: optional float
  pub fn r#pm2_5(&self) -> f32 {
    unsafe { sensors_PMS7003Sensor_pm2_5(self.inner.msg) }
  }
  pub fn r#pm2_5_mut(&mut self) -> ::__pb::PrimitiveMut<'_, f32> {
    static VTABLE: ::__pb::__internal::PrimitiveVTable<f32> =
      ::__pb::__internal::PrimitiveVTable::new(
        ::__pb::__internal::Private,
        sensors_PMS7003Sensor_pm2_5,
        sensors_PMS7003Sensor_set_pm2_5,
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
    unsafe { sensors_PMS7003Sensor_pm10(self.inner.msg) }
  }
  pub fn r#pm10_mut(&mut self) -> ::__pb::PrimitiveMut<'_, f32> {
    static VTABLE: ::__pb::__internal::PrimitiveVTable<f32> =
      ::__pb::__internal::PrimitiveVTable::new(
        ::__pb::__internal::Private,
        sensors_PMS7003Sensor_pm10,
        sensors_PMS7003Sensor_set_pm10,
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
    let submsg = unsafe { sensors_PMS7003Sensor_measurement_time(self.inner.msg) };
    // For upb, getters return null if the field is unset, so we need to
    // check for null and return the default instance manually. Note that
    // a null ptr received from upb manifests as Option::None
    match submsg {
        // TODO:(b/304357029)
        None => crate::google::protobuf::TimestampView::new(::__pb::__internal::Private, ::__pb::__runtime::ScratchSpace::zeroed_block()),
        Some(field) => crate::google::protobuf::TimestampView::new(::__pb::__internal::Private, field),
      }
  }


}  // impl PMS7003Sensor

impl ::__std::ops::Drop for PMS7003Sensor {
  fn drop(&mut self) {
  }
}

extern "C" {
  fn sensors_PMS7003Sensor_new(arena: ::__pb::__internal::RawArena) -> ::__pb::__internal::RawMessage;
  fn sensors_PMS7003Sensor_serialize(msg: ::__pb::__internal::RawMessage, arena: ::__pb::__internal::RawArena, len: &mut usize) -> ::__std::ptr::NonNull<u8>;
  fn sensors_PMS7003Sensor_parse(data: *const u8, size: usize, arena: ::__pb::__internal::RawArena) -> Option<::__pb::__internal::RawMessage>;

  fn sensors_PMS7003Sensor_pm2_5(raw_msg: ::__pb::__internal::RawMessage) -> f32;
  fn sensors_PMS7003Sensor_set_pm2_5(raw_msg: ::__pb::__internal::RawMessage, val: f32);
  fn sensors_PMS7003Sensor_clear_pm2_5(raw_msg: ::__pb::__internal::RawMessage);

  fn sensors_PMS7003Sensor_pm10(raw_msg: ::__pb::__internal::RawMessage) -> f32;
  fn sensors_PMS7003Sensor_set_pm10(raw_msg: ::__pb::__internal::RawMessage, val: f32);
  fn sensors_PMS7003Sensor_clear_pm10(raw_msg: ::__pb::__internal::RawMessage);

  fn sensors_PMS7003Sensor_measurement_time(raw_msg: ::__pb::__internal::RawMessage) -> Option<::__pb::__internal::RawMessage>;


}  // extern "C" for PMS7003Sensor


} // mod sensors
