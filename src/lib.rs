extern crate pg_extend;
extern crate pg_extern_attr;

use pg_extend::pg_magic;
use pg_extern_attr::pg_extern;
//use pg_extend::PgTypeInfo;
use pg_extend::info;
use pg_extend::pg_datum::PgDatum;
use pg_extend::pg_sys;
use std::ffi::CString;
use std::os::raw::c_char;

pg_magic!(version: pg_sys::PG_VERSION_NUM);

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_start() -> &'static pg_sys::Pg_finfo_record {
    &pg_extend::pg_sys::Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_get_token() -> &'static pg_sys::Pg_finfo_record {
    &pg_extend::pg_sys::Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_end() -> &'static pg_sys::Pg_finfo_record {
    &pg_extend::pg_sys::Pg_finfo_record { api_version: 1 }
}

#[no_mangle]
pub extern "C" fn pg_finfo_chamkho_parser_lextype() -> &'static pg_sys::Pg_finfo_record {
    &pg_extend::pg_sys::Pg_finfo_record { api_version: 1 }
}

#[repr(C)]
struct ParserCtx {
    i: i32,
    // char* text;         // the current position of text
    // int text_len;       // the length of text
    // char* buf;          // A text to parse
    // int buf_len;        // the length of text
    // int *pos;           // The position of a breaking word
    // int num;            // The number of a breaking word
    // int cur_id;         // An ID of current word
}

#[repr(C)]
struct LexDescr {
    lexid: i32,
    // int  lexid;
    // char *alias;
    // char *descr;
}

#[no_mangle]
pub extern "C" fn chamkho_parser_start(func_call_info: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    let ctx = unsafe { pg_sys::palloc0(std::mem::size_of::<ParserCtx>() as u64) as pg_sys::Datum };
    let args: Vec<_> = unsafe { pg_extend::get_args(func_call_info.as_mut().unwrap()) }.collect();
    let text_len = args[1].unwrap() as i32;
    info!("!!!! TEXT-LEN = {:?}", text_len);

    let raw_text = args[0].unwrap() as *mut u8;
    let text_u8 = unsafe { std::slice::from_raw_parts(raw_text, text_len as usize) };
    let text = String::from_utf8(text_u8.to_vec()).unwrap();
    info!("!!! TEXT = {}", &text);
    ctx
}

#[no_mangle]
pub extern "C" fn chamkho_parser_get_token(
    func_call_info: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    let ctx = unsafe { pg_sys::palloc0(std::mem::size_of::<ParserCtx>() as u64) as pg_sys::Datum };
//    let text = unsafe { pg_sys::PG_GETARG_POINTER(0) };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    ctx
}

#[no_mangle]
pub extern "C" fn chamkho_parser_end(_func_call_info: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    let x = unsafe { pg_sys::palloc(10) as *mut u8 };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    x as pg_sys::Datum
}

#[no_mangle]
pub extern "C" fn chamkho_parser_lextype(
    _func_call_info: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    let x = unsafe { pg_sys::palloc(10) as *mut u8 };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    x as pg_sys::Datum
}
