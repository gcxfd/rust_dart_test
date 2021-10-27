use dart_sdk_sys::{Dart_FinalizableHandle, Dart_Handle, Dart_HandleFinalizer};
use std::ffi::c_void;

#[link(name = "dart")]
extern "C" {
  pub fn Dart_InitializeApiDL(obj: *mut c_void) -> usize;
  pub fn Dart_NewFinalizableHandle_DL(
    handle: Dart_Handle,
    peer: *mut c_void,
    external_allocation_size: usize,
    callback: Dart_HandleFinalizer,
  ) -> Dart_FinalizableHandle;
}

#[no_mangle]
pub unsafe extern "C" fn initDart(obj: *mut c_void) -> usize {
  return Dart_InitializeApiDL(obj);
}

#[no_mangle]
pub unsafe extern "C" fn do_free(isolate_callback_data: *mut c_void, peer: *mut c_void) {
  println!("free!!!!!!");
}

#[no_mangle]
pub unsafe extern "C" fn gcBind(handle: Dart_Handle) {
  let test: [u8; 2] = [1, 2];
  let peer = Box::into_raw(Box::new(test)) as *mut std::ffi::c_void;
  Dart_NewFinalizableHandle_DL(handle, peer, 2, Some(do_free));
}
