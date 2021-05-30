pub use tracy_macros::{zone_scoped, zone_scoped_prefix};
pub use tracy_sys::*;

/// Emits a string message.
///
/// ```
/// fn handle_event() {
///     message_string_literal!("Got an event");
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! message_string_literal {
    ($name:tt) => {
        rustracy::emit_message_string_literal(concat!($name, "\0"))
    };
}

/// Captures numeric value changes over time.
///
/// ```
/// let num_samples = generate_samples();
/// plot!("num_samples", num_samples);
/// // ...
/// ```
#[macro_export]
macro_rules! plot {
    ($name:tt, $value:expr) => {
        rustracy::emit_plot(concat!($name, "\0"), $value as f64)
    };
}

/// Creates a zone.
///
/// A zone represents the lifetime of a special on-stack profiler variable.
///
/// ```
/// fn some_task() {
///     let _zone = create_zone_scoped!("some_task");
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! create_zone_scoped {
    ($name:tt) => {{
        const name: &'static str = concat!($name, "\0");
        const file: &'static str = concat!(file!(), "\0");
        const loc: rustracy::SourceLocation = rustracy::SourceLocation {
            name: name.as_ptr() as *const _,
            function: name.as_ptr() as *const _,
            file: file.as_ptr() as *const _,
            line: line!(),
            color: 0,
        };
        rustracy::ZoneScoped::new(&loc)
    }};
}
