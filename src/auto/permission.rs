// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Permission(Object<ffi::GPermission>);

    match fn {
        get_type => || ffi::g_permission_get_type(),
    }
}

pub trait PermissionExt {
    //fn acquire(&self, cancellable: /*Ignored*/Option<&Cancellable>) -> Result<(), Error>;

    //fn acquire_async(&self, cancellable: /*Ignored*/Option<&Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Fundamental: Pointer);

    //fn acquire_finish<T: IsA</*Ignored*/AsyncResult>>(&self, result: &T) -> Result<(), Error>;

    fn get_allowed(&self) -> bool;

    fn get_can_acquire(&self) -> bool;

    fn get_can_release(&self) -> bool;

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool);

    //fn release(&self, cancellable: /*Ignored*/Option<&Cancellable>) -> Result<(), Error>;

    //fn release_async(&self, cancellable: /*Ignored*/Option<&Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Fundamental: Pointer);

    //fn release_finish<T: IsA</*Ignored*/AsyncResult>>(&self, result: &T) -> Result<(), Error>;
}

impl<O: IsA<Permission>> PermissionExt for O {
    //fn acquire(&self, cancellable: /*Ignored*/Option<&Cancellable>) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_permission_acquire() }
    //}

    //fn acquire_async(&self, cancellable: /*Ignored*/Option<&Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::g_permission_acquire_async() }
    //}

    //fn acquire_finish<T: IsA</*Ignored*/AsyncResult>>(&self, result: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_permission_acquire_finish() }
    //}

    fn get_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_allowed(self.to_glib_none().0))
        }
    }

    fn get_can_acquire(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_acquire(self.to_glib_none().0))
        }
    }

    fn get_can_release(&self) -> bool {
        unsafe {
            from_glib(ffi::g_permission_get_can_release(self.to_glib_none().0))
        }
    }

    fn impl_update(&self, allowed: bool, can_acquire: bool, can_release: bool) {
        unsafe {
            ffi::g_permission_impl_update(self.to_glib_none().0, allowed.to_glib(), can_acquire.to_glib(), can_release.to_glib());
        }
    }

    //fn release(&self, cancellable: /*Ignored*/Option<&Cancellable>) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_permission_release() }
    //}

    //fn release_async(&self, cancellable: /*Ignored*/Option<&Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::g_permission_release_async() }
    //}

    //fn release_finish<T: IsA</*Ignored*/AsyncResult>>(&self, result: &T) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_permission_release_finish() }
    //}
}
