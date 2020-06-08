extern crate pg_extend;
extern crate pg_extern_attr;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate wordcut_engine;

use pg_extend::pg_magic;
use pg_extend::pg_sys;
use pg_extend::pg_sys::{Datum, FunctionCallInfo, Pg_finfo_record};
use regex::Regex;
use std::path::Path;
use wordcut_engine::{TextRange, Wordcut};

pg_magic!(version: pg_sys::PG_VERSION_NUM);

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_start() -> &'static Pg_finfo_record {
    &Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_get_token() -> &'static Pg_finfo_record {
    &Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_end() -> &'static Pg_finfo_record {
    &Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_lextype() -> &'static Pg_finfo_record {
    &Pg_finfo_record { api_version: 1 }
}

lazy_static! {
    static ref WORDCUT: Wordcut = Wordcut::new(
        wordcut_engine::load_dict(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/dix.txt"
        )))
        .unwrap()
    );
    static ref THAI_RE: Regex = Regex::new(r"[ก-์]").unwrap();
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

#[repr(C)]
struct LexDescr {
    lexid: i32,
    alias: *mut i8,
    descr: *mut i8,
}

#[no_mangle]
pub extern "C" fn chamkho_parser_start(func_call_info: FunctionCallInfo) -> Datum {
    unsafe {
        let ctx = pg_sys::palloc0(std::mem::size_of::<ParserCtx>() as u64) as *mut ParserCtx;
        let args: Vec<_> = pg_extend::get_args(func_call_info.as_mut().unwrap()).collect();
        (*ctx).text = args[0].unwrap() as *const u8;
        (*ctx).text_len = (args[1].unwrap() as i32) as usize;
        (*ctx).is_segmented = false;
        ctx as Datum
    }
}

#[no_mangle]
pub extern "C" fn chamkho_parser_get_token(func_call_info: FunctionCallInfo) -> Datum {
    unsafe {
        let args: Vec<_> = pg_extend::get_args(func_call_info.as_mut().unwrap()).collect();
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
            drop(&(*ctx).text_ranges);
            (*ctx).is_segmented = false;
            0
        } else {
            let r = &((*ctx).text_ranges[(*ctx).word_idx]);
            let len = (r.e - r.s) as i32;
            let buf = (*ctx).text.offset(r.s as isize);
            *token_len = len;
            *token = buf;
            (*ctx).word_idx += 1;
            let w = &String::from_utf8_lossy(std::slice::from_raw_parts(buf, len as usize));
            (if THAI_RE.find(w).is_some() {
                'a'
            } else if SPACE_RE.is_match(w) {
                'c'
            } else {
                'b'
            }) as Datum
        }
    }
}

#[no_mangle]
pub extern "C" fn chamkho_parser_end(func_call_info: FunctionCallInfo) -> Datum {
    unsafe {
        let args: Vec<_> = pg_extend::get_args(func_call_info.as_mut().unwrap()).collect();
        let ctx = args[0].unwrap() as *mut ParserCtx;
        pg_sys::pfree(ctx as *mut std::ffi::c_void);
        0
    }
}

#[no_mangle]
pub extern "C" fn chamkho_parser_lextype(_func_call_info: FunctionCallInfo) -> Datum {
    unsafe {
        let descr = pg_sys::palloc0((std::mem::size_of::<LexDescr>() * 27) as u64) as *mut LexDescr;
        (*descr.offset(0)).lexid = 97;
        (*descr.offset(0)).alias = pg_sys::pstrdup(b"a".as_ptr() as *const i8);
        (*descr.offset(0)).alias = pg_sys::pstrdup(b"Thai word".as_ptr() as *const i8);
        (*descr.offset(1)).lexid = 98;
        (*descr.offset(1)).alias = pg_sys::pstrdup(b"b".as_ptr() as *const i8);
        (*descr.offset(1)).alias = pg_sys::pstrdup(b"English word".as_ptr() as *const i8);
        (*descr.offset(2)).lexid = 99;
        (*descr.offset(2)).alias = pg_sys::pstrdup(b"c".as_ptr() as *const i8);
        (*descr.offset(2)).alias = pg_sys::pstrdup(b"Space".as_ptr() as *const i8);
        (*descr.offset(26)).lexid = 0;
        descr as Datum
    }
}
