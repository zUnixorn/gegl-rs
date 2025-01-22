// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::{ffi,Config,Path,Rectangle,SplitStrategy,Stats};
use glib::{translate::*};


//#[doc(alias = "gegl_apply_op")]
//pub fn apply_op(buffer: &Buffer, operation_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
//    unsafe { TODO: call ffi:gegl_apply_op() }
//}

//#[doc(alias = "gegl_apply_op_valist")]
//pub fn apply_op_valist(buffer: &Buffer, operation_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
//    unsafe { TODO: call ffi:gegl_apply_op_valist() }
//}

//#[doc(alias = "gegl_calloc")]
//pub fn calloc(size: usize, n_memb: i32) -> /*Unimplemented*/Option<Basic: Pointer> {
//    unsafe { TODO: call ffi:gegl_calloc() }
//}

/// Disable OpenCL
#[doc(alias = "gegl_cl_disable")]
pub fn cl_disable() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gegl_cl_disable();
    }
}

/// Initialize and enable OpenCL, calling this function again
/// will re-enable OpenCL if it has been disabled.
///
/// # Returns
///
/// True if OpenCL was initialized
#[doc(alias = "gegl_cl_init")]
pub fn cl_init() -> Result<(), glib::Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::gegl_cl_init(&mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
    }
}

/// Check if OpenCL is enabled.
///
/// # Returns
///
/// True if OpenCL is initialized and enabled
#[doc(alias = "gegl_cl_is_accelerated")]
pub fn cl_is_accelerated() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gegl_cl_is_accelerated())
    }
}

/// Returns a GeglConfig object with properties that can be manipulated to control
/// GEGLs behavior.
///
/// # Returns
///
/// a [`Config`][crate::Config]
#[doc(alias = "gegl_config")]
pub fn config() -> Config {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gegl_config())
    }
}

//#[doc(alias = "gegl_create_chain")]
//pub fn create_chain(ops: &str, op_start: /*Ignored*/&Node, op_end: /*Ignored*/&Node, time: f64, rel_dim: i32, path_root: &str) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:gegl_create_chain() }
//}

/// Call this function when you're done using GEGL. It will clean up
/// caches and write/dump debug information if the correct debug flags
/// are set.
#[doc(alias = "gegl_exit")]
pub fn exit() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gegl_exit();
    }
}

//#[doc(alias = "gegl_filter_op")]
//pub fn filter_op(source_buffer: &Buffer, operation_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<Buffer> {
//    unsafe { TODO: call ffi:gegl_filter_op() }
//}

//#[doc(alias = "gegl_filter_op_valist")]
//pub fn filter_op_valist(source_buffer: &Buffer, operation_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<Buffer> {
//    unsafe { TODO: call ffi:gegl_filter_op_valist() }
//}

/// Returns a value sutable to pass to the GeglBuffer constructor
/// or any other property that expects a Babl format.
/// ## `format_name`
/// A Babl format name, e.g. "RGBA float"
///
/// # Returns
///
/// the format pointer
#[doc(alias = "gegl_format")]
pub fn format(format_name: &str) -> Option<glib::Value> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_format(format_name.to_glib_none().0))
    }
}

/// ## `format`
/// A Babl pointer
///
/// # Returns
///
/// the format name
#[doc(alias = "gegl_format_get_name")]
pub fn format_get_name(format: &mut glib::Value) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gegl_format_get_name(format.to_glib_none_mut().0))
    }
}

//#[doc(alias = "gegl_free")]
//pub fn free(mem: /*Unimplemented*/Option<Basic: Pointer>) {
//    unsafe { TODO: call ffi:gegl_free() }
//}

/// This function fetches the version of the GEGL library being used by
/// the running process.
///
/// # Returns
///
///
/// ## `major`
/// a pointer to a int where the major version number will be stored
///
/// ## `minor`
/// ditto for the minor version number
///
/// ## `micro`
/// ditto for the micro version number
#[doc(alias = "gegl_get_version")]
#[doc(alias = "get_version")]
pub fn version() -> (i32, i32, i32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut major = std::mem::MaybeUninit::uninit();
        let mut minor = std::mem::MaybeUninit::uninit();
        let mut micro = std::mem::MaybeUninit::uninit();
        ffi::gegl_get_version(major.as_mut_ptr(), minor.as_mut_ptr(), micro.as_mut_ptr());
        (major.assume_init(), minor.assume_init(), micro.assume_init())
    }
}

//#[doc(alias = "gegl_graph_dump_outputs")]
//pub fn graph_dump_outputs(node: /*Ignored*/&Node) {
//    unsafe { TODO: call ffi:gegl_graph_dump_outputs() }
//}

//#[doc(alias = "gegl_graph_dump_request")]
//pub fn graph_dump_request(node: /*Ignored*/&Node, roi: &Rectangle) {
//    unsafe { TODO: call ffi:gegl_graph_dump_request() }
//}

/// ## `operation_type`
/// the name of the operation
///
/// # Returns
///
/// A boolean telling whether the operation is present or not. This
/// also returns true for any compat-name registered by operations.
#[doc(alias = "gegl_has_operation")]
pub fn has_operation(operation_type: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gegl_has_operation(operation_type.to_glib_none().0))
    }
}

//#[doc(alias = "gegl_init")]
//pub fn init(argv: /*Unimplemented*/Vec<glib::GString>) {
//    unsafe { TODO: call ffi:gegl_init() }
//}

#[doc(alias = "gegl_is_main_thread")]
pub fn is_main_thread() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gegl_is_main_thread())
    }
}

///
/// # Returns
///
/// An
/// alphabetically sorted array of available operation names. This excludes any
/// compat-name registered by operations. The list should be freed with g_free
/// after use.
/// ---
/// gchar **operations;
/// guint n_operations;
/// gint i;
///
/// operations = gegl_list_operations (&n_operations);
/// g_print ("Available operations:\n");
/// for (i=0; i < n_operations; i++)
///  {
///  g_print ("\t`s`\n", operations[i]);
///  }
/// g_free (operations);
#[doc(alias = "gegl_list_operations")]
pub fn list_operations() -> Vec<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        let mut n_operations_p = std::mem::MaybeUninit::uninit();
        let ret = FromGlibContainer::from_glib_container_num(ffi::gegl_list_operations(n_operations_p.as_mut_ptr()), n_operations_p.assume_init() as _);
        ret
    }
}

/// Load all gegl modules found in the given directory.
/// ## `path`
/// the directory to load modules from
#[doc(alias = "gegl_load_module_directory")]
pub fn load_module_directory(path: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gegl_load_module_directory(path.to_glib_none().0);
    }
}

//#[doc(alias = "gegl_malloc")]
//pub fn malloc(n_bytes: usize) -> /*Unimplemented*/Option<Basic: Pointer> {
//    unsafe { TODO: call ffi:gegl_malloc() }
//}

//#[doc(alias = "gegl_memeq_zero")]
//pub fn memeq_zero(ptr: /*Unimplemented*/Option<Basic: Pointer>, size: usize) -> bool {
//    unsafe { TODO: call ffi:gegl_memeq_zero() }
//}

//#[doc(alias = "gegl_memset_pattern")]
//pub fn memset_pattern(dst_ptr: /*Unimplemented*/Option<Basic: Pointer>, src_ptr: /*Unimplemented*/Option<Basic: Pointer>, pattern_size: i32, count: i32) {
//    unsafe { TODO: call ffi:gegl_memset_pattern() }
//}

/// Distributes the execution of a function across multiple threads,
/// by calling it with a different index on each thread.
/// ## `max_n`
/// the maximal number of threads to use
/// ## `func`
/// the function to call
#[doc(alias = "gegl_parallel_distribute")]
pub fn parallel_distribute<P: FnMut(i32, i32)>(max_n: i32, func: P) {
    assert_initialized_main_thread!();
    let mut func_data: P = func;
    unsafe extern "C" fn func_func<P: FnMut(i32, i32)>(i: std::ffi::c_int, n: std::ffi::c_int, user_data: glib::ffi::gpointer) {
        let callback = user_data as *mut P;
        (*callback)(i, n)
    }
    let func = Some(func_func::<P> as _);
    let super_callback0: &mut P = &mut func_data;
    unsafe {
        ffi::gegl_parallel_distribute(max_n, func, super_callback0 as *mut _ as *mut _);
    }
}

/// Distributes the processing of a planar data-structure across
/// multiple threads, by calling the given function with different
/// sub-areas on different threads.
/// ## `area`
/// the area to process
/// ## `thread_cost`
/// the cost of using each additional thread, relative
///  to the cost of processing a single data element
/// ## `split_strategy`
/// the strategy to use for dividing the area
/// ## `func`
/// the function to call
#[doc(alias = "gegl_parallel_distribute_area")]
pub fn parallel_distribute_area<P: FnMut(&Rectangle)>(area: &Rectangle, thread_cost: f64, split_strategy: SplitStrategy, func: P) {
    assert_initialized_main_thread!();
    let mut func_data: P = func;
    unsafe extern "C" fn func_func<P: FnMut(&Rectangle)>(area: *const ffi::GeglRectangle, user_data: glib::ffi::gpointer) {
        let area = from_glib_borrow(area);
        let callback = user_data as *mut P;
        (*callback)(&area)
    }
    let func = Some(func_func::<P> as _);
    let super_callback0: &mut P = &mut func_data;
    unsafe {
        ffi::gegl_parallel_distribute_area(area.to_glib_none().0, thread_cost, split_strategy.into_glib(), func, super_callback0 as *mut _ as *mut _);
    }
}

/// Distributes the processing of a linear data-structure across
/// multiple threads, by calling the given function with different
/// sub-ranges on different threads.
/// ## `size`
/// the total size of the data
/// ## `thread_cost`
/// the cost of using each additional thread, relative
///  to the cost of processing a single data element
/// ## `func`
/// the function to call
#[doc(alias = "gegl_parallel_distribute_range")]
pub fn parallel_distribute_range<P: FnMut(usize, usize)>(size: usize, thread_cost: f64, func: P) {
    assert_initialized_main_thread!();
    let mut func_data: P = func;
    unsafe extern "C" fn func_func<P: FnMut(usize, usize)>(offset: libc::size_t, size: libc::size_t, user_data: glib::ffi::gpointer) {
        let callback = user_data as *mut P;
        (*callback)(offset, size)
    }
    let func = Some(func_func::<P> as _);
    let super_callback0: &mut P = &mut func_data;
    unsafe {
        ffi::gegl_parallel_distribute_range(size, thread_cost, func, super_callback0 as *mut _ as *mut _);
    }
}

/// Creates a new [`glib::ParamSpec`][crate::glib::ParamSpec] instance specifying a [`AudioFragment`][crate::AudioFragment] property.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_audio_fragment")]
pub fn param_spec_audio_fragment(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_audio_fragment(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, flags.into_glib()))
    }
}

/// Creates a new [`glib::ParamSpec`][crate::glib::ParamSpec] instance specifying a [`Color`][crate::Color] property.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `default_color_string`
/// the default value for the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_color_from_string")]
pub fn param_spec_color_from_string(name: &str, nick: &str, blurb: &str, default_color_string: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_color_from_string(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, default_color_string.to_glib_none().0, flags.into_glib()))
    }
}

//#[doc(alias = "gegl_param_spec_curve")]
//pub fn param_spec_curve(name: &str, nick: &str, blurb: &str, default_curve: /*Ignored*/&Curve, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
//    unsafe { TODO: call ffi:gegl_param_spec_curve() }
//}

/// Creates a new `GeglParamSpecDouble` instance.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `minimum`
/// minimum value for the property specified
/// ## `maximum`
/// maximum value for the property specified
/// ## `default_value`
/// default value for the property specified
/// ## `ui_minimum`
/// minimum value a user should be allowed to input
/// ## `ui_maximum`
/// maximum value a user should be allowed to input
/// ## `ui_gamma`
/// the gamma that should be used when adjusting the value
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_double")]
pub fn param_spec_double(name: &str, nick: &str, blurb: &str, minimum: f64, maximum: f64, default_value: f64, ui_minimum: f64, ui_maximum: f64, ui_gamma: f64, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_double(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, minimum, maximum, default_value, ui_minimum, ui_maximum, ui_gamma, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecEnum` instance.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `enum_type`
/// the enum type to get valid values from
/// ## `default_value`
/// default value for the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_enum")]
pub fn param_spec_enum(name: &str, nick: &str, blurb: &str, enum_type: glib::types::Type, default_value: i32, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_enum(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, enum_type.into_glib(), default_value, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecFilePath` instance.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `no_validate`
/// true if the string should be validated with g_utf8_validate
/// ## `null_ok`
/// true if the string can be NULL
/// ## `default_value`
/// default value for the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_file_path")]
pub fn param_spec_file_path(name: &str, nick: &str, blurb: &str, no_validate: bool, null_ok: bool, default_value: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_file_path(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, no_validate.into_glib(), null_ok.into_glib(), default_value.to_glib_none().0, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecFormat` instance specifying a Babl format.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_format")]
pub fn param_spec_format(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_format(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecInt` instance.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `minimum`
/// minimum value for the property specified
/// ## `maximum`
/// maximum value for the property specified
/// ## `default_value`
/// default value for the property specified
/// ## `ui_minimum`
/// minimum value a user should be allowed to input
/// ## `ui_maximum`
/// maximum value a user should be allowed to input
/// ## `ui_gamma`
/// the gamma that should be used when adjusting the value
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_int")]
pub fn param_spec_int(name: &str, nick: &str, blurb: &str, minimum: i32, maximum: i32, default_value: i32, ui_minimum: i32, ui_maximum: i32, ui_gamma: f64, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_int(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, minimum, maximum, default_value, ui_minimum, ui_maximum, ui_gamma, flags.into_glib()))
    }
}

/// Creates a new [`glib::ParamSpec`][crate::glib::ParamSpec] instance specifying a [`Path`][crate::Path] property.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `default_path`
/// the default value for the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_path")]
pub fn param_spec_path(name: &str, nick: &str, blurb: &str, default_path: &Path, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_path(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, default_path.to_glib_none().0, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecSeed` instance specifying an integer random seed.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_seed")]
pub fn param_spec_seed(name: &str, nick: &str, blurb: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_seed(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecString` instance.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `no_validate`
/// true if the string should be validated with g_utf8_validate
/// ## `null_ok`
/// true if the string can be NULL
/// ## `default_value`
/// default value for the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_string")]
pub fn param_spec_string(name: &str, nick: &str, blurb: &str, no_validate: bool, null_ok: bool, default_value: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_string(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, no_validate.into_glib(), null_ok.into_glib(), default_value.to_glib_none().0, flags.into_glib()))
    }
}

/// Creates a new `GeglParamSpecUri` instance.
/// ## `name`
/// canonical name of the property specified
/// ## `nick`
/// nick name for the property specified
/// ## `blurb`
/// description of the property specified
/// ## `no_validate`
/// true if the string should be validated with g_utf8_validate
/// ## `null_ok`
/// true if the string can be NULL
/// ## `default_value`
/// default value for the property specified
/// ## `flags`
/// flags for the property specified
///
/// # Returns
///
/// a newly created parameter specification
#[doc(alias = "gegl_param_spec_uri")]
pub fn param_spec_uri(name: &str, nick: &str, blurb: &str, no_validate: bool, null_ok: bool, default_value: &str, flags: glib::ParamFlags) -> Option<glib::ParamSpec> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gegl_param_spec_uri(name.to_glib_none().0, nick.to_glib_none().0, blurb.to_glib_none().0, no_validate.into_glib(), null_ok.into_glib(), default_value.to_glib_none().0, flags.into_glib()))
    }
}

//#[doc(alias = "gegl_render_op")]
//pub fn render_op(source_buffer: &Buffer, target_buffer: &Buffer, operation_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
//    unsafe { TODO: call ffi:gegl_render_op() }
//}

//#[doc(alias = "gegl_render_op_valist")]
//pub fn render_op_valist(source_buffer: &Buffer, target_buffer: &Buffer, operation_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
//    unsafe { TODO: call ffi:gegl_render_op_valist() }
//}

/// Resets the cumulative data gathered by the [`Stats`][crate::Stats] object returned
/// by [`stats()`][crate::stats()].
#[doc(alias = "gegl_reset_stats")]
pub fn reset_stats() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gegl_reset_stats();
    }
}

//#[doc(alias = "gegl_scratch_alloc")]
//pub fn scratch_alloc(size: usize) -> /*Unimplemented*/Option<Basic: Pointer> {
//    unsafe { TODO: call ffi:gegl_scratch_alloc() }
//}

//#[doc(alias = "gegl_scratch_alloc0")]
//pub fn scratch_alloc0(size: usize) -> /*Unimplemented*/Option<Basic: Pointer> {
//    unsafe { TODO: call ffi:gegl_scratch_alloc0() }
//}

//#[doc(alias = "gegl_scratch_free")]
//pub fn scratch_free(ptr: /*Unimplemented*/Option<Basic: Pointer>) {
//    unsafe { TODO: call ffi:gegl_scratch_free() }
//}

//#[doc(alias = "gegl_serialize")]
//pub fn serialize(start: /*Ignored*/&Node, end: /*Ignored*/&Node, basepath: &str, serialize_flags: SerializeFlag) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:gegl_serialize() }
//}

/// Returns a GeglStats object with properties that can be read to monitor
/// GEGL statistics.
///
/// # Returns
///
/// a [`Stats`][crate::Stats]
#[doc(alias = "gegl_stats")]
pub fn stats() -> Option<Stats> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gegl_stats())
    }
}

//#[doc(alias = "gegl_try_malloc")]
//pub fn try_malloc(n_bytes: usize) -> /*Unimplemented*/Option<Basic: Pointer> {
//    unsafe { TODO: call ffi:gegl_try_malloc() }
//}
