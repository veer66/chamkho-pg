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

#[repr(C)]
struct ChamkhoParser {
    i: i32,
}

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

#[no_mangle]
pub extern "C" fn chamkho_parser_start(_func_call_info: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    info!("@@@@@@@1");
    let x = unsafe { pg_sys::palloc(10) as *mut u8 };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    x as pg_sys::Datum
}

#[no_mangle]
pub extern "C" fn chamkho_parser_get_token(
    _func_call_info: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    info!("@@@@@@@2");
    let x = unsafe { pg_sys::palloc(10) as *mut u8 };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    x as pg_sys::Datum
}

#[no_mangle]
pub extern "C" fn chamkho_parser_end(_func_call_info: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    info!("@@@@@@@3");
    let x = unsafe { pg_sys::palloc(10) as *mut u8 };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    x as pg_sys::Datum
}

#[no_mangle]
pub extern "C" fn chamkho_parser_lextype(
    _func_call_info: pg_sys::FunctionCallInfo,
) -> pg_sys::Datum {
    info!("@@@@@@@4");
    let x = unsafe { pg_sys::palloc(10) as *mut u8 };
    //Box::into_raw(Box::new(ChamkhoParser { i: 10 })) as pg_sys::Datum
    x as pg_sys::Datum
}
