// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Rectangle,TileSource};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "GeglTileHandler")]
    pub struct TileHandler(Object<ffi::GeglTileHandler, ffi::GeglTileHandlerClass>) @extends TileSource;

    match fn {
        type_ => || ffi::gegl_tile_handler_get_type(),
    }
}

impl TileHandler {
        pub const NONE: Option<&'static TileHandler> = None;
    

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`TileHandler`] objects.
            ///
            /// This method returns an instance of [`TileHandlerBuilder`](crate::builders::TileHandlerBuilder) which can be used to create [`TileHandler`] objects.
            pub fn builder() -> TileHandlerBuilder {
                TileHandlerBuilder::new()
            }
        
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`TileHandler`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TileHandlerBuilder {
            builder: glib::object::ObjectBuilder<'static, TileHandler>,
        }

        impl TileHandlerBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            //pub fn source(self, source: &impl IsA</*Ignored*/glib::Object>) -> Self {
                        //    Self { builder: self.builder.property("source", source.clone().upcast()), }
                        //}

    // rustdoc-stripper-ignore-next
    /// Build the [`TileHandler`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> TileHandler {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait TileHandlerExt: IsA<TileHandler> + 'static {
    //#[doc(alias = "gegl_tile_handler_create_tile")]
    //fn create_tile(&self, x: i32, y: i32, z: i32) -> /*Ignored*/Option<Tile> {
    //    unsafe { TODO: call ffi:gegl_tile_handler_create_tile() }
    //}

    #[doc(alias = "gegl_tile_handler_damage_rect")]
    fn damage_rect(&self, rect: &Rectangle) {
        unsafe {
            ffi::gegl_tile_handler_damage_rect(self.as_ref().to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[doc(alias = "gegl_tile_handler_damage_tile")]
    fn damage_tile(&self, x: i32, y: i32, z: i32, damage: u64) {
        unsafe {
            ffi::gegl_tile_handler_damage_tile(self.as_ref().to_glib_none().0, x, y, z, damage);
        }
    }

    //#[doc(alias = "gegl_tile_handler_dup_tile")]
    //fn dup_tile(&self, tile: /*Ignored*/&mut Tile, x: i32, y: i32, z: i32) -> /*Ignored*/Option<Tile> {
    //    unsafe { TODO: call ffi:gegl_tile_handler_dup_tile() }
    //}

    //#[doc(alias = "gegl_tile_handler_get_source_tile")]
    //#[doc(alias = "get_source_tile")]
    //fn source_tile(&self, x: i32, y: i32, z: i32, preserve_data: bool) -> /*Ignored*/Option<Tile> {
    //    unsafe { TODO: call ffi:gegl_tile_handler_get_source_tile() }
    //}

    //#[doc(alias = "gegl_tile_handler_get_tile")]
    //#[doc(alias = "get_tile")]
    //fn tile(&self, x: i32, y: i32, z: i32, preserve_data: bool) -> /*Ignored*/Option<Tile> {
    //    unsafe { TODO: call ffi:gegl_tile_handler_get_tile() }
    //}

    #[doc(alias = "gegl_tile_handler_lock")]
    fn lock(&self) {
        unsafe {
            ffi::gegl_tile_handler_lock(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gegl_tile_handler_set_source")]
    #[doc(alias = "source")]
    fn set_source(&self, source: &impl IsA<TileSource>) {
        unsafe {
            ffi::gegl_tile_handler_set_source(self.as_ref().to_glib_none().0, source.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gegl_tile_handler_unlock")]
    fn unlock(&self) {
        unsafe {
            ffi::gegl_tile_handler_unlock(self.as_ref().to_glib_none().0);
        }
    }

    //fn source(&self) -> /*Ignored*/Option<glib::Object> {
    //    ObjectExt::property(self.as_ref(), "source")
    //}

    #[doc(alias = "source")]
    fn connect_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<P: IsA<TileHandler>, F: Fn(&P) + 'static>(this: *mut ffi::GeglTileHandler, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(TileHandler::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::source".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_source_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<TileHandler>> TileHandlerExt for O {}
