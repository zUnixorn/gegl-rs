// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Buffer,Rectangle};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    ///
    ///
    /// ## Properties
    ///
    ///
    /// #### `chunksize`
    ///  Readable | Writeable | Construct Only
    ///
    ///
    /// #### `node`
    ///  Writeable | Construct
    ///
    ///
    /// #### `progress`
    ///  Readable | Writeable
    ///
    ///
    /// #### `rectangle`
    ///  Readable | Writeable
    #[doc(alias = "GeglProcessor")]
    pub struct Processor(Object<ffi::GeglProcessor>);

    match fn {
        type_ => || ffi::gegl_processor_get_type(),
    }
}

impl Processor {
            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Processor`] objects.
            ///
            /// This method returns an instance of [`ProcessorBuilder`](crate::builders::ProcessorBuilder) which can be used to create [`Processor`] objects.
            pub fn builder() -> ProcessorBuilder {
                ProcessorBuilder::new()
            }
        

    /// Returns the (cache) buffer the processor is rendering into, another way of
    /// getting to the same pixel data is calling gegl_node_blit with flags
    /// indicating that we want caching and accept dirty data.
    ///
    /// # Returns
    ///
    /// the [`Buffer`][crate::Buffer] rendered into.
    #[doc(alias = "gegl_processor_get_buffer")]
    #[doc(alias = "get_buffer")]
    pub fn buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_full(ffi::gegl_processor_get_buffer(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gegl_processor_set_level")]
    pub fn set_level(&self, level: i32) {
        unsafe {
            ffi::gegl_processor_set_level(self.to_glib_none().0, level);
        }
    }

    /// Change the rectangle a [`Processor`][crate::Processor] is working on.
    /// ## `rectangle`
    /// the new [`Rectangle`][crate::Rectangle] the processor shold work on or NULL
    /// to make it work on all data in the buffer.
    #[doc(alias = "gegl_processor_set_rectangle")]
    #[doc(alias = "rectangle")]
    pub fn set_rectangle(&self, rectangle: &Rectangle) {
        unsafe {
            ffi::gegl_processor_set_rectangle(self.to_glib_none().0, rectangle.to_glib_none().0);
        }
    }

    #[doc(alias = "gegl_processor_set_scale")]
    pub fn set_scale(&self, scale: f64) {
        unsafe {
            ffi::gegl_processor_set_scale(self.to_glib_none().0, scale);
        }
    }

    /// Do an iteration of work for the processor.
    ///
    /// Returns TRUE if there is more work to be done.
    ///
    /// ---
    /// GeglProcessor *processor = gegl_node_new_processor (node, &roi);
    /// double progress;
    ///
    /// while (gegl_processor_work (processor, &progress))
    ///  g_warning ("`f`%% complete", progress);
    /// g_object_unref (processor);
    ///
    /// # Returns
    ///
    ///
    /// ## `progress`
    /// a location to store the (estimated) percentage complete.
    #[doc(alias = "gegl_processor_work")]
    pub fn work(&self) -> Option<f64> {
        unsafe {
            let mut progress = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gegl_processor_work(self.to_glib_none().0, progress.as_mut_ptr()));
            if ret { Some(progress.assume_init()) } else { None }
        }
    }

    pub fn chunksize(&self) -> i32 {
        ObjectExt::property(self, "chunksize")
    }

    //pub fn set_node(&self, node: /*Ignored*/Option<&Node>) {
    //    ObjectExt::set_property(self,"node", node)
    //}

    pub fn progress(&self) -> f64 {
        ObjectExt::property(self, "progress")
    }

    pub fn set_progress(&self, progress: f64) {
        ObjectExt::set_property(self,"progress", progress)
    }

    //pub fn rectangle(&self) -> /*Unimplemented*/Basic: Pointer {
    //    ObjectExt::property(self, "rectangle")
    //}

    #[doc(alias = "node")]
    pub fn connect_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_node_trampoline<F: Fn(&Processor) + 'static>(this: *mut ffi::GeglProcessor, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::node".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_node_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "progress")]
    pub fn connect_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_progress_trampoline<F: Fn(&Processor) + 'static>(this: *mut ffi::GeglProcessor, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::progress".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_progress_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "rectangle")]
    pub fn connect_rectangle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rectangle_trampoline<F: Fn(&Processor) + 'static>(this: *mut ffi::GeglProcessor, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::rectangle".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_rectangle_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Processor`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ProcessorBuilder {
            builder: glib::object::ObjectBuilder<'static, Processor>,
        }

        impl ProcessorBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn chunksize(self, chunksize: i32) -> Self {
                            Self { builder: self.builder.property("chunksize", chunksize), }
                        }

                            //pub fn node(self, node: /*Ignored*/&Node) -> Self {
                        //    Self { builder: self.builder.property("node", node), }
                        //}

                            pub fn progress(self, progress: f64) -> Self {
                            Self { builder: self.builder.property("progress", progress), }
                        }

                            //pub fn rectangle(self, rectangle: /*Unimplemented*/Basic: Pointer) -> Self {
                        //    Self { builder: self.builder.property("rectangle", rectangle), }
                        //}

    // rustdoc-stripper-ignore-next
    /// Build the [`Processor`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Processor {
assert_initialized_main_thread!();
    self.builder.build() }
}
