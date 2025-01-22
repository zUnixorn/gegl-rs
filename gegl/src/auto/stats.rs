// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    ///
    ///
    /// ## Properties
    ///
    ///
    /// #### `active-threads`
    ///  Readable
    ///
    ///
    /// #### `assigned-threads`
    ///  Readable
    ///
    ///
    /// #### `scratch-total`
    ///  Readable
    ///
    ///
    /// #### `swap-busy`
    ///  Readable
    ///
    ///
    /// #### `swap-file-size`
    ///  Readable
    ///
    ///
    /// #### `swap-queue-full`
    ///  Readable
    ///
    ///
    /// #### `swap-queue-stalls`
    ///  Readable
    ///
    ///
    /// #### `swap-queued-total`
    ///  Readable
    ///
    ///
    /// #### `swap-read-total`
    ///  Readable
    ///
    ///
    /// #### `swap-reading`
    ///  Readable
    ///
    ///
    /// #### `swap-total`
    ///  Readable
    ///
    ///
    /// #### `swap-total-uncompressed`
    ///  Readable
    ///
    ///
    /// #### `swap-write-total`
    ///  Readable
    ///
    ///
    /// #### `swap-writing`
    ///  Readable
    ///
    ///
    /// #### `tile-alloc-total`
    ///  Readable
    ///
    ///
    /// #### `tile-cache-hits`
    ///  Readable
    ///
    ///
    /// #### `tile-cache-misses`
    ///  Readable
    ///
    ///
    /// #### `tile-cache-total`
    ///  Readable
    ///
    ///
    /// #### `tile-cache-total-max`
    ///  Readable
    ///
    ///
    /// #### `tile-cache-total-uncompressed`
    ///  Readable
    ///
    ///
    /// #### `zoom-total`
    ///  Readable
    #[doc(alias = "GeglStats")]
    pub struct Stats(Object<ffi::GeglStats>);

    match fn {
        type_ => || ffi::gegl_stats_get_type(),
    }
}

impl Stats {
    #[doc(alias = "active-threads")]
    pub fn active_threads(&self) -> i32 {
        ObjectExt::property(self, "active-threads")
    }

    #[doc(alias = "assigned-threads")]
    pub fn assigned_threads(&self) -> i32 {
        ObjectExt::property(self, "assigned-threads")
    }

    #[doc(alias = "scratch-total")]
    pub fn scratch_total(&self) -> u64 {
        ObjectExt::property(self, "scratch-total")
    }

    #[doc(alias = "swap-busy")]
    pub fn is_swap_busy(&self) -> bool {
        ObjectExt::property(self, "swap-busy")
    }

    #[doc(alias = "swap-file-size")]
    pub fn swap_file_size(&self) -> u64 {
        ObjectExt::property(self, "swap-file-size")
    }

    #[doc(alias = "swap-queue-full")]
    pub fn is_swap_queue_full(&self) -> bool {
        ObjectExt::property(self, "swap-queue-full")
    }

    #[doc(alias = "swap-queue-stalls")]
    pub fn swap_queue_stalls(&self) -> i32 {
        ObjectExt::property(self, "swap-queue-stalls")
    }

    #[doc(alias = "swap-queued-total")]
    pub fn swap_queued_total(&self) -> u64 {
        ObjectExt::property(self, "swap-queued-total")
    }

    #[doc(alias = "swap-read-total")]
    pub fn swap_read_total(&self) -> u64 {
        ObjectExt::property(self, "swap-read-total")
    }

    #[doc(alias = "swap-reading")]
    pub fn is_swap_reading(&self) -> bool {
        ObjectExt::property(self, "swap-reading")
    }

    #[doc(alias = "swap-total")]
    pub fn swap_total(&self) -> u64 {
        ObjectExt::property(self, "swap-total")
    }

    #[doc(alias = "swap-total-uncompressed")]
    pub fn swap_total_uncompressed(&self) -> u64 {
        ObjectExt::property(self, "swap-total-uncompressed")
    }

    #[doc(alias = "swap-write-total")]
    pub fn swap_write_total(&self) -> u64 {
        ObjectExt::property(self, "swap-write-total")
    }

    #[doc(alias = "swap-writing")]
    pub fn is_swap_writing(&self) -> bool {
        ObjectExt::property(self, "swap-writing")
    }

    #[doc(alias = "tile-alloc-total")]
    pub fn tile_alloc_total(&self) -> u64 {
        ObjectExt::property(self, "tile-alloc-total")
    }

    #[doc(alias = "tile-cache-hits")]
    pub fn tile_cache_hits(&self) -> i32 {
        ObjectExt::property(self, "tile-cache-hits")
    }

    #[doc(alias = "tile-cache-misses")]
    pub fn tile_cache_misses(&self) -> i32 {
        ObjectExt::property(self, "tile-cache-misses")
    }

    #[doc(alias = "tile-cache-total")]
    pub fn tile_cache_total(&self) -> u64 {
        ObjectExt::property(self, "tile-cache-total")
    }

    #[doc(alias = "tile-cache-total-max")]
    pub fn tile_cache_total_max(&self) -> u64 {
        ObjectExt::property(self, "tile-cache-total-max")
    }

    #[doc(alias = "tile-cache-total-uncompressed")]
    pub fn tile_cache_total_uncompressed(&self) -> u64 {
        ObjectExt::property(self, "tile-cache-total-uncompressed")
    }

    #[doc(alias = "zoom-total")]
    pub fn zoom_total(&self) -> u64 {
        ObjectExt::property(self, "zoom-total")
    }

    #[doc(alias = "active-threads")]
    pub fn connect_active_threads_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_threads_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::active-threads".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_active_threads_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "assigned-threads")]
    pub fn connect_assigned_threads_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_assigned_threads_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::assigned-threads".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_assigned_threads_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "scratch-total")]
    pub fn connect_scratch_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scratch_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::scratch-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_scratch_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-busy")]
    pub fn connect_swap_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_busy_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-busy".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_busy_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-file-size")]
    pub fn connect_swap_file_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_file_size_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-file-size".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_file_size_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-queue-full")]
    pub fn connect_swap_queue_full_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_queue_full_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-queue-full".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_queue_full_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-queue-stalls")]
    pub fn connect_swap_queue_stalls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_queue_stalls_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-queue-stalls".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_queue_stalls_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-queued-total")]
    pub fn connect_swap_queued_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_queued_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-queued-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_queued_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-read-total")]
    pub fn connect_swap_read_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_read_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-read-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_read_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-reading")]
    pub fn connect_swap_reading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_reading_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-reading".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_reading_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-total")]
    pub fn connect_swap_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-total-uncompressed")]
    pub fn connect_swap_total_uncompressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_total_uncompressed_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-total-uncompressed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_total_uncompressed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-write-total")]
    pub fn connect_swap_write_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_write_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-write-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_write_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "swap-writing")]
    pub fn connect_swap_writing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_swap_writing_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::swap-writing".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_swap_writing_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "tile-alloc-total")]
    pub fn connect_tile_alloc_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_alloc_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::tile-alloc-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_tile_alloc_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "tile-cache-hits")]
    pub fn connect_tile_cache_hits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_cache_hits_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::tile-cache-hits".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_tile_cache_hits_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "tile-cache-misses")]
    pub fn connect_tile_cache_misses_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_cache_misses_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::tile-cache-misses".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_tile_cache_misses_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "tile-cache-total")]
    pub fn connect_tile_cache_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_cache_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::tile-cache-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_tile_cache_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "tile-cache-total-max")]
    pub fn connect_tile_cache_total_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_cache_total_max_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::tile-cache-total-max".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_tile_cache_total_max_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "tile-cache-total-uncompressed")]
    pub fn connect_tile_cache_total_uncompressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_cache_total_uncompressed_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::tile-cache-total-uncompressed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_tile_cache_total_uncompressed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "zoom-total")]
    pub fn connect_zoom_total_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_zoom_total_trampoline<F: Fn(&Stats) + 'static>(this: *mut ffi::GeglStats, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, c"notify::zoom-total".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_zoom_total_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
