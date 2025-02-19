// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Surface;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkDragSurface")]
    pub struct DragSurface(Interface<ffi::GdkDragSurface, ffi::GdkDragSurfaceInterface>) @requires Surface;

    match fn {
        type_ => || ffi::gdk_drag_surface_get_type(),
    }
}

impl DragSurface {
    pub const NONE: Option<&'static DragSurface> = None;
}

pub trait DragSurfaceExt: 'static {
    #[doc(alias = "gdk_drag_surface_present")]
    fn present(&self, width: i32, height: i32) -> bool;
}

impl<O: IsA<DragSurface>> DragSurfaceExt for O {
    fn present(&self, width: i32, height: i32) -> bool {
        unsafe {
            from_glib(ffi::gdk_drag_surface_present(
                self.as_ref().to_glib_none().0,
                width,
                height,
            ))
        }
    }
}

impl fmt::Display for DragSurface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DragSurface")
    }
}
