// extern crate pg_extend;
// extern crate pg_extern_attr;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate wordcut_engine;

// use pg_extend::pg_magic;
// use pg_extend::pg_sys;
// use pg_extend::pg_sys::{Datum, FunctionCallInfo, Pg_finfo_record};
use regex::Regex;
use std::os::raw::{c_char, c_int};
use std::path::Path;
use std::sync::atomic::{compiler_fence, Ordering};
use wordcut_engine::{TextRange, Wordcut};
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
#[allow(clippy::approx_constant)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::needless_lifetimes)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Copied from pg-extend-rs project
cfg_if::cfg_if! {
    if #[cfg(windows)] {
        unsafe fn pg_sys_longjmp(_buf: *mut sys::_JBTYPE, _value: ::std::os::raw::c_int) {
            sys::longjmp(_buf, _value);
        }
    } else if #[cfg(target_os = "macos")] {
        unsafe fn pg_sys_longjmp(_buf: *mut c_int, _value: ::std::os::raw::c_int) {
            sys::siglongjmp(_buf, _value);
        }
    } else if #[cfg(unix)] {
        unsafe fn pg_sys_longjmp(_buf: *mut sys::__jmp_buf_tag, _value: ::std::os::raw::c_int) {
            sys::siglongjmp(_buf, _value);
        }
    }
}

// Copied from pg-extend-rs project
/// Postgres logging Levels
///
/// # Note
///
/// Some of these levels effect the status of the connection and transaction in Postgres.
/// Specifically, >= `Error` will cause the connection and transaction to fail and be reset.
#[derive(Clone, Copy)]
pub enum Level {
    /// Debugging messages, in categories of 5 decreasing detail.
    Debug5 = sys::DEBUG5 as isize,
    /// Debugging messages, in categories of 4 decreasing detail.
    Debug4 = sys::DEBUG4 as isize,
    /// Debugging messages, in categories of 3 decreasing detail.
    Debug3 = sys::DEBUG3 as isize,
    /// Debugging messages, in categories of 2 decreasing detail.
    Debug2 = sys::DEBUG2 as isize,
    /// Debugging messages, in categories of 1 decreasing detail.
    Debug1 = sys::DEBUG1 as isize,
    /// Server operational messages; sent only to server log by default.
    Log = sys::LOG as isize,
    /// Same as LOG for server reporting, but never sent to client.
    ///   `CommError` is an alias for this
    #[cfg(not(postgres9))]
    LogServerOnly = sys::LOG_SERVER_ONLY as isize,
    /// Messages specifically requested by user (eg VACUUM VERBOSE output); always sent to client
    /// regardless of client_min_messages, but by default not sent to server log.
    Info = sys::INFO as isize,
    /// Helpful messages to users about query operation; sent to client and not to server log by
    /// default.
    Notice = sys::NOTICE as isize,
    /// Warnings.  NOTICE is for expected messages like implicit sequence creation by SERIAL.
    /// WARNING is for unexpected messages.
    Warning = sys::WARNING as isize,
    /// user error - abort transaction; return to known state
    Error = sys::ERROR as isize,
    /// fatal error - abort process
    Fatal = sys::FATAL as isize,
    /// take down the other backends with me
    Panic = sys::PANIC as isize,
}

// Copied from pg-extend-rs project
/// Provides a barrier between Rust and Postgres' usage of the C set/longjmp
///
/// In the case of a longjmp being caught, this will convert that to a panic. For this to work
///   properly, there must be a Rust panic handler (see crate::register_panic_handler).PanicContext
///   If the `pg_exern` attribute macro is used for exposing Rust functions to Postgres, then
///   this is already handled.
///
/// See the man pages for info on setjmp http://man7.org/linux/man-pages/man3/setjmp.3.html
#[cfg(windows)]
#[inline(never)]
pub(crate) unsafe fn guard_pg<R, F: FnOnce() -> R>(f: F) -> R {
    // setup the check protection
    let original_exception_stack: *mut pg_sys::jmp_buf = pg_sys::PG_exception_stack;
    let mut local_exception_stack: mem::MaybeUninit<pg_sys::jmp_buf> = mem::MaybeUninit::uninit();
    let jumped = pg_sys::_setjmp(
        // grab a mutable reference, cast to a mutabl pointr, then case to the expected erased pointer type
        local_exception_stack.as_mut_ptr() as *mut pg_sys::jmp_buf as *mut _,
    );
    // now that we have the local_exception_stack, we set that for any PG longjmps...

    if jumped != 0 {
        notice!("PG longjmped: {}", jumped);
        pg_sys::PG_exception_stack = original_exception_stack;

        // The C Panicked!, handling control to Rust Panic handler
        compiler_fence(Ordering::SeqCst);
        panic!(JumpContext { jump_value: jumped });
    }

    // replace the exception stack with ours to jump to the above point
    pg_sys::PG_exception_stack = local_exception_stack.as_mut_ptr() as *mut _;

    // enforce that the setjmp is not reordered, though that's probably unlikely...
    compiler_fence(Ordering::SeqCst);
    let result = f();

    compiler_fence(Ordering::SeqCst);
    pg_sys::PG_exception_stack = original_exception_stack;

    result
}

/// Provides a barrier between Rust and Postgres' usage of the C set/longjmp
///
/// In the case of a longjmp being caught, this will convert that to a panic. For this to work
///   properly, there must be a Rust panic handler (see crate::register_panic_handler).PanicContext
///   If the `pg_exern` attribute macro is used for exposing Rust functions to Postgres, then
///   this is already handled.
///
/// See the man pages for info on setjmp http://man7.org/linux/man-pages/man3/setjmp.3.html
#[cfg(unix)]
#[inline(never)]
pub(crate) unsafe fn guard_pg<R, F: FnOnce() -> R>(f: F) -> R {
    // setup the check protection
    let original_exception_stack: *mut sys::sigjmp_buf = sys::PG_exception_stack;
    let mut local_exception_stack: std::mem::MaybeUninit<sys::sigjmp_buf> =
        std::mem::MaybeUninit::uninit();
    let jumped = sys::__sigsetjmp(
        // grab a mutable reference, cast to a mutabl pointr, then case to the expected erased pointer type
        local_exception_stack.as_mut_ptr() as *mut sys::sigjmp_buf as *mut _,
        1,
    );
    // now that we have the local_exception_stack, we set that for any PG longjmps...

    if jumped != 0 {
        notice!("PG longjmped: {}", jumped);
        sys::PG_exception_stack = original_exception_stack;

        // The C Panicked!, handling control to Rust Panic handler
        compiler_fence(Ordering::SeqCst);
        std::panic::panic_any(JumpContext { jump_value: jumped });
    }

    // replace the exception stack with ours to jump to the above point
    sys::PG_exception_stack = local_exception_stack.as_mut_ptr() as *mut _;

    // enforce that the setjmp is not reordered, though that's probably unlikely...
    compiler_fence(Ordering::SeqCst);
    let result = f();

    compiler_fence(Ordering::SeqCst);
    sys::PG_exception_stack = original_exception_stack;

    result
}

// Copied from pg-extend-rs project
// WARNING: this is not part of the crate's public API and is subject to change at any time
#[doc(hidden)]
pub fn __private_api_log(
    _args: std::fmt::Arguments,
    level: Level,
    &(module_path, file, line): &(*const c_char, *const c_char, u32),
) {
    let errlevel: c_int = level as c_int;
    let line = line as c_int;
    //const LOG_DOMAIN: *const c_char = "RUST\0" as *const str as *const c_char;

    // Rust has no "function name" macro, for now we use module path instead.
    // See: https://github.com/rust-lang/rfcs/issues/1743
    let do_log = unsafe { crate::guard_pg(|| sys::errstart(errlevel, file)) };

    // If errstart returned false, the message won't be seen by anyone; logging will be skipped
    if do_log {
        // At this point we format the passed format string `args`; if the log level is suppressed,
        // no string processing needs to take place.
        //let msg = format!("{}", args);
        // let c_msg = CString::new(msg).or_else(
        //     |_| CString::new("failed to convert msg to a CString, check extension code for incompatible `CString` messages")
        // ).expect("this should not fail: msg");

        unsafe {
            crate::guard_pg(|| {
                compiler_fence(Ordering::SeqCst);
                //                let msg_result = sys::errmsg(c_msg.as_ptr());
                sys::errfinish(file, line, module_path);
            });
        }
    }
}

// Copied from pg-extend-rs project
/// Generic logging macro. See the [`Level` enum] for all available log levels.
///
/// Usually one wouldn't call this directly but the more convenient specialized macros.
///
/// # Example
///
/// ```rust,no_run
/// use pg_extend::pg_log;
/// use pg_extend::log::Level;
///
/// pg_log!(Level::LogServerOnly, "Big brother is watching {}!", "you");
/// ````
///
/// [`Level` enum]: enum.Level.html
#[macro_export]
macro_rules! pg_log {
    ($lvl:expr, $($arg:tt)+) => ({
        $crate::__private_api_log(
            format_args!($($arg)+),
            $lvl,
            // Construct a tuple; the whole tuple is a compile-time constant.
            &(
                // Construct zero-terminated strings at compile time.
                concat!(module_path!(), "\0") as *const str as *const ::std::os::raw::c_char,
                concat!(file!(), "\0") as *const str as *const ::std::os::raw::c_char,
                line!(),
            ),
        );
    });
}

// Copied from pg-extend-rs project
/// Log at `ERROR` level and abort the current query and transaction.
/// Beware! The PostgreSQL implementation uses exception handling with `longjmp`, which currently
/// has unsafe side-effects.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::pg_log!($crate::Level::Error, $($arg)*);
    )
}

// Copied from pg-extend-rs project
/// Log at `NOTICE` level. Use for helpful messages to users about query operation; expected
/// messages like implicit sequence creation by SERIAL.
#[macro_export]
macro_rules! notice {
    ($($arg:tt)*) => (
        $crate::pg_log!($crate::Level::Notice, $($arg)*);
    )
}

// Copied from pg-extend-rs project
pub fn register_panic_handler() {
    use std::panic;

    // set (and replace the existing) panic handler, this will tell Postgres that the call failed
    //   a level of Fatal will force the DB connection to be killed.
    panic::set_hook(Box::new(|info| {
        // downcast info, check if it's the value we need.
        //   this must check if the panic was due to a longjmp
        //   the fence is to make sure the longjmp is not reodered.
        compiler_fence(Ordering::SeqCst);
        if let Some(panic_context) = info.payload().downcast_ref::<JumpContext>() {
            // WARNING: do not set this level above Notice (ERROR, FATAL, PANIC), as it will cause
            //   the following longjmp to execute.
            notice!("continuing longjmp: {}", info);

            // the panic came from a pg longjmp... so unwrap it and rethrow
            unsafe {
                pg_sys_longjmp(sys::PG_exception_stack as *mut _, panic_context.jump_value);
            }
        } else {
            // error level will cause a longjmp in Postgres
            error!("panic in Rust extension: {}", info);
        }

        unreachable!("all above statements should have cause a longjmp to Postgres");
    }));
}

// Copied from pg-extend-rs
/// Information for a longjmp
struct JumpContext {
    jump_value: c_int,
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[link_name = "Pg_magic_func"]
pub fn Pg_magic_func() -> &'static sys::Pg_magic_struct {
    use std::mem::size_of;
    use std::os::raw::c_int;

    const my_magic: sys::Pg_magic_struct = sys::Pg_magic_struct {
        len: size_of::<sys::Pg_magic_struct>() as c_int,
        version: sys::PG_VERSION_NUM as std::os::raw::c_int / 100,
        funcmaxargs: sys::FUNC_MAX_ARGS as c_int,
        indexmaxkeys: sys::INDEX_MAX_KEYS as c_int,
        namedatalen: sys::NAMEDATALEN as c_int,
        float8byval: sys::USE_FLOAT8_BYVAL as c_int,
        abi_extra: {
            // Copied from PGRX project
            let mut abi = [0; 32];
            abi[0] = b'P' as _;
            abi[1] = b'o' as _;
            abi[2] = b's' as _;
            abi[3] = b't' as _;
            abi[4] = b'g' as _;
            abi[5] = b'r' as _;
            abi[6] = b'e' as _;
            abi[7] = b'S' as _;
            abi[8] = b'Q' as _;
            abi[9] = b'L' as _;
            abi[10] = 0;
            abi
        },
    };

    //register_panic_handler();
    &my_magic
}

// Copied from pg-extend-rs
/// Returns an iterator of argument Datums
pub fn get_args(
    func_call_info: &sys::FunctionCallInfoBaseData,
) -> impl '_ + Iterator<Item = Option<sys::Datum>> {
    let num_args = func_call_info.nargs as usize;

    return unsafe { func_call_info.args.as_slice(num_args) }
        .iter()
        .map(|nullable| {
            if nullable.isnull {
                None
            } else {
                Some(nullable.value)
            }
        });
}

#[no_mangle]
fn pg_finfo_chamkho_parser_start() -> &'static sys::Pg_finfo_record {
    &sys::Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
fn pg_finfo_chamkho_parser_get_token() -> &'static sys::Pg_finfo_record {
    &sys::Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
fn pg_finfo_chamkho_parser_end() -> &'static sys::Pg_finfo_record {
    &sys::Pg_finfo_record { api_version: 1 }
}

lazy_static! {
    static ref WORDCUT: Wordcut = Wordcut::new(
        wordcut_engine::load_dict(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/dix.txt"
        )))
        .unwrap()
    );
    static ref SPACE_RE: Regex = Regex::new(r"[\s\t\r\n]").unwrap();
}

#[repr(C)]
struct ParserCtx {
    text: *const u8,
    text_len: usize,
    text_ranges: Vec<TextRange>,
    word_idx: usize,
    is_segmented: bool,
}

#[no_mangle]
pub fn chamkho_parser_start(func_call_info: sys::FunctionCallInfo) -> sys::Datum {
    unsafe {
        let ctx = sys::palloc0(std::mem::size_of::<ParserCtx>()) as *mut ParserCtx;
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        let args: Vec<_> = get_args(func_call_info.as_mut().unwrap()).collect();
        (*ctx).text = args[0].unwrap() as *const u8;
        (*ctx).text_len = (args[1].unwrap() as i32) as usize;
        (*ctx).is_segmented = false;
        ctx as sys::Datum
    }
}

#[no_mangle]
fn chamkho_parser_get_token(func_call_info: sys::FunctionCallInfo) -> sys::Datum {
    unsafe {
        let args: Vec<_> = get_args(func_call_info.as_mut().unwrap()).collect();
        let ctx = args[0].unwrap() as *mut ParserCtx;
        let token = args[1].unwrap() as *mut *const u8;
        let token_len = args[2].unwrap() as *mut i32;

        if !(*ctx).is_segmented {
            (*ctx).text_ranges = WORDCUT.segment_into_byte_ranges(&String::from_utf8_lossy(
                std::slice::from_raw_parts((*ctx).text, (*ctx).text_len as usize),
            ));
            (*ctx).is_segmented = true;
            (*ctx).word_idx = 0;
        }

        if (*ctx).word_idx >= (*ctx).text_ranges.len() {
            *token_len = 0;
            (*ctx).is_segmented = false;
            0
        } else {
            let r = &((*ctx).text_ranges[(*ctx).word_idx]);
            let len = (r.e - r.s) as i32;
            let buf = (*ctx).text.add(r.s);
            *token_len = len;
            *token = buf;
            (*ctx).word_idx += 1;
            let w = &String::from_utf8_lossy(std::slice::from_raw_parts(buf, len as usize));
            (if SPACE_RE.is_match(w) {
                12
            } else {
                2
            }) as sys::Datum
        }
    }
}

#[no_mangle]
fn chamkho_parser_end(func_call_info: sys::FunctionCallInfo) -> sys::Datum {
    unsafe {
        let args: Vec<_> = get_args(func_call_info.as_mut().unwrap()).collect();
        let ctx = args[0].unwrap() as *mut ParserCtx;
        sys::pfree(ctx as *mut std::ffi::c_void);
        0
    }
}
