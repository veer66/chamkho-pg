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
use std::path::Path;
use wordcut_engine::{TextRange, Wordcut};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(improper_ctypes)]
#[allow(dead_code)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
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
            let mut abi = [0 as i8; 32];
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


// pg_magic!(version: pg_sys::PG_VERSION_NUM);

// #[no_mangle]
// pub extern "C}" fn pg_finfo_chamkho_parser_start() -> &'static Pg_finfo_record {
//     &Pg_finfo_record { api_version: 1 }
// }

// #[no_mangle]
// pub extern "C" fn pg_finfo_chamkho_parser_get_token() -> &'static Pg_finfo_record {
//     &Pg_finfo_record { api_version: 1 }
// }

// #[no_mangle]
// pub extern "C" fn pg_finfo_chamkho_parser_end() -> &'static Pg_finfo_record {
//     &Pg_finfo_record { api_version: 1 }
// }

// lazy_static! {
//     static ref WORDCUT: Wordcut = Wordcut::new(
//         wordcut_engine::load_dict(Path::new(concat!(
//             env!("CARGO_MANIFEST_DIR"),
//             "/data/dix.txt"
//         )))
//         .unwrap()
//     );
//     static ref THAI_RE: Regex = Regex::new(r"[ก-์]").unwrap();
//     static ref SPACE_RE: Regex = Regex::new(r"[\s\t\r\n]").unwrap();
// }

// #[repr(C)]
// struct ParserCtx {
//     text: *const u8,
//     text_len: usize,
//     text_ranges: Vec<TextRange>,
//     word_idx: usize,
//     is_segmented: bool,
// }

// #[no_mangle]
// pub extern "C" fn chamkho_parser_start(func_call_info: FunctionCallInfo) -> Datum {
//     unsafe {
//         let ctx = pg_sys::palloc0(std::mem::size_of::<ParserCtx>() as u64) as *mut ParserCtx;
//         let args: Vec<_> = pg_extend::get_args(func_call_info.as_mut().unwrap()).collect();
//         (*ctx).text = args[0].unwrap() as *const u8;
//         (*ctx).text_len = (args[1].unwrap() as i32) as usize;
//         (*ctx).is_segmented = false;
//         ctx as Datum
//     }
// }

// #[no_mangle]
// pub extern "C" fn chamkho_parser_get_token(func_call_info: FunctionCallInfo) -> Datum {
//     unsafe {
//         let args: Vec<_> = pg_extend::get_args(func_call_info.as_mut().unwrap()).collect();
//         let ctx = args[0].unwrap() as *mut ParserCtx;
//         let token = args[1].unwrap() as *mut *const u8;
//         let token_len = args[2].unwrap() as *mut i32;

//         if !(*ctx).is_segmented {
//             (*ctx).text_ranges = WORDCUT.segment_into_byte_ranges(&String::from_utf8_lossy(
//                 std::slice::from_raw_parts((*ctx).text, (*ctx).text_len as usize),
//             ));
//             (*ctx).is_segmented = true;
//             (*ctx).word_idx = 0;
//         }

//         if (*ctx).word_idx >= (*ctx).text_ranges.len() {
//             *token_len = 0;
//             drop(&(*ctx).text_ranges);
//             (*ctx).is_segmented = false;
//             0
//         } else {
//             let r = &((*ctx).text_ranges[(*ctx).word_idx]);
//             let len = (r.e - r.s) as i32;
//             let buf = (*ctx).text.offset(r.s as isize);
//             *token_len = len;
//             *token = buf;
//             (*ctx).word_idx += 1;
//             let w = &String::from_utf8_lossy(std::slice::from_raw_parts(buf, len as usize));
//             (if THAI_RE.find(w).is_some() {
//                 2
//             } else if SPACE_RE.is_match(w) {
//                 12
//             } else {
//                 2
//             }) as Datum
//         }
//     }
// }

// #[no_mangle]
// pub extern "C" fn chamkho_parser_end(func_call_info: FunctionCallInfo) -> Datum {
//     unsafe {
//         let args: Vec<_> = pg_extend::get_args(func_call_info.as_mut().unwrap()).collect();
//         let ctx = args[0].unwrap() as *mut ParserCtx;
//         pg_sys::pfree(ctx as *mut std::ffi::c_void);
//         0
//     }
// }
