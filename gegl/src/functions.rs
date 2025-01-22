use glib::translate::ToGlibPtr;

#[doc(alias = "gegl_init")]
pub fn init() {
    unsafe {
        let argv = ::std::env::args().take(1).collect::<Vec<_>>();
        ffi::gegl_init(&mut 1, &mut argv.to_glib_none().0);
    }
}
