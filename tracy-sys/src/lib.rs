use std::os::raw::{c_char, c_double, c_int};

#[repr(C)]
pub struct SourceLocation {
    pub name: *const c_char,
    pub function: *const c_char,
    pub file: *const c_char,
    pub line: u32,
    pub color: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ZoneContext {
    id: u32,
    active: c_int,
}

pub struct ZoneScoped {
    ctx: ZoneContext,
}

impl ZoneScoped {
    pub fn new(loc: &SourceLocation) -> Self {
        let ctx = emit_zone_begin(loc);
        ZoneScoped { ctx }
    }
}

impl Drop for ZoneScoped {
    fn drop(&mut self) {
        emit_zone_end(self.ctx);
    }
}

extern "C" {
    fn ___tracy_emit_zone_begin(srcloc: *const SourceLocation, active: c_int) -> ZoneContext;
    fn ___tracy_emit_zone_end(ctx: ZoneContext);

    fn ___tracy_emit_frame_mark(name: *const c_char);
    fn ___tracy_emit_frame_mark_start(name: *const c_char);
    fn ___tracy_emit_frame_mark_end(name: *const c_char);

    fn ___tracy_emit_plot(name: *const c_char, value: c_double);

    fn ___tracy_emit_messageL(name: *const c_char, callstack: c_int);
}

pub fn emit_zone_begin(loc: &SourceLocation) -> ZoneContext {
    unsafe { ___tracy_emit_zone_begin(loc, 1) }
}

pub fn emit_zone_end(ctx: ZoneContext) {
    unsafe { ___tracy_emit_zone_end(ctx) }
}

// Tentative
pub fn emit_frame_mark_with_null() {
    unsafe { ___tracy_emit_frame_mark(std::ptr::null()) }
}

pub fn emit_frame_mark(name: &'static str) {
    unsafe { ___tracy_emit_frame_mark(name.as_ptr() as *const _) }
}

pub fn emit_frame_mark_start(name: &'static str) {
    unsafe { ___tracy_emit_frame_mark_start(name.as_ptr() as *const _) }
}

pub fn emit_frame_mark_end(name: &'static str) {
    unsafe { ___tracy_emit_frame_mark_end(name.as_ptr() as *const _) }
}

pub fn emit_plot(name: &'static str, value: f64) {
    unsafe { ___tracy_emit_plot(name.as_ptr() as *const _, value) }
}

pub fn emit_message_string_literal(name: &'static str) {
    unsafe { ___tracy_emit_messageL(name.as_ptr() as *const _, 1) }
}
