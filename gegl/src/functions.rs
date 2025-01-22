use glib::translate::ToGlibPtr;

/// Call this function before using any other GEGL functions. It will
/// initialize everything needed to operate GEGL and parses some
/// standard command line options. `argc` and `argv` are adjusted
/// accordingly so your own code will never see those standard
/// arguments.
///
/// Note that there is an alternative way to initialize GEGL: if you
/// are calling `g_option_context_parse()` with the option group returned
/// by `gegl_get_option_group()`, you don't have to call `gegl_init()`.
/// ## `argv`
/// a pointer to the array of command line arguments.
#[doc(alias = "gegl_init")]
pub fn init() {
    unsafe {
        let argv = ::std::env::args().take(1).collect::<Vec<_>>();
        ffi::gegl_init(&mut 1, &mut argv.to_glib_none().0);
    }
}
