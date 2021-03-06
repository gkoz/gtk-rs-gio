// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use ActionGroup;
use ActionMap;
use ApplicationFlags;
use ffi;
#[cfg(feature = "v2_44")]
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Application(Object<ffi::GApplication>): ActionGroup, ActionMap;

    match fn {
        get_type => || ffi::g_application_get_type(),
    }
}

impl Application {
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Application {
        unsafe {
            from_glib_full(ffi::g_application_new(application_id.to_glib_none().0, flags.to_glib()))
        }
    }

    pub fn get_default() -> Option<Application> {
        unsafe {
            from_glib_none(ffi::g_application_get_default())
        }
    }

    pub fn id_is_valid(application_id: &str) -> bool {
        unsafe {
            from_glib(ffi::g_application_id_is_valid(application_id.to_glib_none().0))
        }
    }
}

pub trait ApplicationExt {
    fn activate(&self);

    //#[cfg(feature = "v2_42")]
    //fn add_main_option(&self, long_name: &str, short_name: /*Unimplemented*/Fundamental: Char, flags: /*Ignored*/glib::OptionFlags, arg: /*Ignored*/glib::OptionArg, description: &str, arg_description: Option<&str>);

    //#[cfg(feature = "v2_40")]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]);

    //#[cfg(feature = "v2_40")]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup);

    #[cfg(feature = "v2_44")]
    fn bind_busy_property<T: IsA<glib::Object>>(&self, object: &T, property: &str);

    fn get_application_id(&self) -> Option<String>;

    //#[cfg(feature = "v2_34")]
    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection>;

    #[cfg(feature = "v2_34")]
    fn get_dbus_object_path(&self) -> Option<String>;

    fn get_flags(&self) -> ApplicationFlags;

    fn get_inactivity_timeout(&self) -> u32;

    #[cfg(feature = "v2_44")]
    fn get_is_busy(&self) -> bool;

    fn get_is_registered(&self) -> bool;

    fn get_is_remote(&self) -> bool;

    #[cfg(feature = "v2_42")]
    fn get_resource_base_path(&self) -> Option<String>;

    fn hold(&self);

    #[cfg(feature = "v2_38")]
    fn mark_busy(&self);

    //fn open(&self, files: /*Ignored*/&[File], n_files: i32, hint: &str);

    fn quit(&self);

    //fn register(&self, cancellable: /*Ignored*/Option<&Cancellable>) -> Result<(), Error>;

    fn release(&self);

    fn run(&self, argc: i32, argv: &[&str]) -> i32;

    //#[cfg(feature = "v2_40")]
    //fn send_notification(&self, id: Option<&str>, notification: /*Ignored*/&Notification);

    fn set_action_group<T: IsA<ActionGroup>>(&self, action_group: Option<&T>);

    fn set_application_id(&self, application_id: Option<&str>);

    fn set_default(&self);

    fn set_flags(&self, flags: ApplicationFlags);

    fn set_inactivity_timeout(&self, inactivity_timeout: u32);

    #[cfg(feature = "v2_42")]
    fn set_resource_base_path(&self, resource_path: Option<&str>);

    #[cfg(feature = "v2_44")]
    fn unbind_busy_property<T: IsA<glib::Object>>(&self, object: &T, property: &str);

    #[cfg(feature = "v2_38")]
    fn unmark_busy(&self);

    #[cfg(feature = "v2_40")]
    fn withdraw_notification(&self, id: &str);
}

impl<O: IsA<Application>> ApplicationExt for O {
    fn activate(&self) {
        unsafe {
            ffi::g_application_activate(self.to_glib_none().0);
        }
    }

    //#[cfg(feature = "v2_42")]
    //fn add_main_option(&self, long_name: &str, short_name: /*Unimplemented*/Fundamental: Char, flags: /*Ignored*/glib::OptionFlags, arg: /*Ignored*/glib::OptionArg, description: &str, arg_description: Option<&str>) {
    //    unsafe { TODO: call ffi::g_application_add_main_option() }
    //}

    //#[cfg(feature = "v2_40")]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]) {
    //    unsafe { TODO: call ffi::g_application_add_main_option_entries() }
    //}

    //#[cfg(feature = "v2_40")]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup) {
    //    unsafe { TODO: call ffi::g_application_add_option_group() }
    //}

    #[cfg(feature = "v2_44")]
    fn bind_busy_property<T: IsA<glib::Object>>(&self, object: &T, property: &str) {
        unsafe {
            ffi::g_application_bind_busy_property(self.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    fn get_application_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_application_id(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_34")]
    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection> {
    //    unsafe { TODO: call ffi::g_application_get_dbus_connection() }
    //}

    #[cfg(feature = "v2_34")]
    fn get_dbus_object_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_dbus_object_path(self.to_glib_none().0))
        }
    }

    fn get_flags(&self) -> ApplicationFlags {
        unsafe {
            from_glib(ffi::g_application_get_flags(self.to_glib_none().0))
        }
    }

    fn get_inactivity_timeout(&self) -> u32 {
        unsafe {
            ffi::g_application_get_inactivity_timeout(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v2_44")]
    fn get_is_busy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_busy(self.to_glib_none().0))
        }
    }

    fn get_is_registered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_registered(self.to_glib_none().0))
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_remote(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_42")]
    fn get_resource_base_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_resource_base_path(self.to_glib_none().0))
        }
    }

    fn hold(&self) {
        unsafe {
            ffi::g_application_hold(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_38")]
    fn mark_busy(&self) {
        unsafe {
            ffi::g_application_mark_busy(self.to_glib_none().0);
        }
    }

    //fn open(&self, files: /*Ignored*/&[File], n_files: i32, hint: &str) {
    //    unsafe { TODO: call ffi::g_application_open() }
    //}

    fn quit(&self) {
        unsafe {
            ffi::g_application_quit(self.to_glib_none().0);
        }
    }

    //fn register(&self, cancellable: /*Ignored*/Option<&Cancellable>) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_application_register() }
    //}

    fn release(&self) {
        unsafe {
            ffi::g_application_release(self.to_glib_none().0);
        }
    }

    fn run(&self, argc: i32, argv: &[&str]) -> i32 {
        unsafe {
            ffi::g_application_run(self.to_glib_none().0, argc, argv.to_glib_none().0)
        }
    }

    //#[cfg(feature = "v2_40")]
    //fn send_notification(&self, id: Option<&str>, notification: /*Ignored*/&Notification) {
    //    unsafe { TODO: call ffi::g_application_send_notification() }
    //}

    fn set_action_group<T: IsA<ActionGroup>>(&self, action_group: Option<&T>) {
        unsafe {
            ffi::g_application_set_action_group(self.to_glib_none().0, action_group.to_glib_none().0);
        }
    }

    fn set_application_id(&self, application_id: Option<&str>) {
        unsafe {
            ffi::g_application_set_application_id(self.to_glib_none().0, application_id.to_glib_none().0);
        }
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_application_set_default(self.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: ApplicationFlags) {
        unsafe {
            ffi::g_application_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn set_inactivity_timeout(&self, inactivity_timeout: u32) {
        unsafe {
            ffi::g_application_set_inactivity_timeout(self.to_glib_none().0, inactivity_timeout);
        }
    }

    #[cfg(feature = "v2_42")]
    fn set_resource_base_path(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::g_application_set_resource_base_path(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_44")]
    fn unbind_busy_property<T: IsA<glib::Object>>(&self, object: &T, property: &str) {
        unsafe {
            ffi::g_application_unbind_busy_property(self.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_38")]
    fn unmark_busy(&self) {
        unsafe {
            ffi::g_application_unmark_busy(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_40")]
    fn withdraw_notification(&self, id: &str) {
        unsafe {
            ffi::g_application_withdraw_notification(self.to_glib_none().0, id.to_glib_none().0);
        }
    }
}
