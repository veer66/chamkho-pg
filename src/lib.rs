use lazy_static::lazy_static;
use pgrx::pg_sys::FunctionCallInfo;
use pgrx::prelude::*;
use pgrx::PgBox;
use regex::Regex;
use std::path::Path;
use wordcut_engine::Wordcut;
::pgrx::pg_module_magic!(name, version);

lazy_static! {
    static ref WORDCUT: Wordcut = {
        let pg_path = Path::new(concat!(
            env!("PG_SHARE_DIR"),
            "/tsearch_data/chamkho_dict.txt"
        ));
        let dev_path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tsearch_data/chamkho_dict.txt"
        ));
        let dict_path = if pg_path.exists() { pg_path } else { dev_path };
        Wordcut::new(wordcut_engine::load_dict(dict_path).unwrap())
    };
    static ref SPACE_RE: Regex = Regex::new(r"[\s\t\r\n]").unwrap();
}

#[derive(Debug)]
struct TextRange {
    s: usize,
    e: usize,
}

pub struct ChamkhoParserContext {
    text_ptr: *const u8,
    text_len: i32,
    ranges_ptr: *mut TextRange,
    ranges_len: i32,
    word_idx: i32,
    is_segmented: bool,
}

impl_sql_translatable!(ChamkhoParserContext, "internal");

#[pg_extern(
    sql = "CREATE FUNCTION chamkho_parser_start(internal, int4) RETURNS internal LANGUAGE C STRICT AS 'MODULE_PATHNAME', 'chamkho_parser_start_wrapper';"
)]
fn chamkho_parser_start(func_call_info: FunctionCallInfo) -> PgBox<ChamkhoParserContext> {
    let size = std::mem::size_of::<ChamkhoParserContext>();

    unsafe {
        let nargs = (*func_call_info).nargs as usize;
        let args: Vec<pg_sys::NullableDatum> = (0..nargs)
            .map(|i| *(*func_call_info).args.as_mut_ptr().add(i))
            .collect();

        let text_ptr = usize::from(args[0].value) as *const u8;
        let text_len = usize::from(args[1].value) as i32;

        let pg_text = pg_sys::palloc(text_len as usize) as *mut u8;
        std::ptr::copy_nonoverlapping(text_ptr, pg_text, text_len as usize);

        let ctx_ptr = pg_sys::palloc0(size) as *mut ChamkhoParserContext;
        (*ctx_ptr).text_ptr = pg_text;
        (*ctx_ptr).text_len = text_len;
        (*ctx_ptr).ranges_ptr = std::ptr::null_mut();
        (*ctx_ptr).ranges_len = 0;
        (*ctx_ptr).word_idx = 0;
        (*ctx_ptr).is_segmented = false;
        PgBox::from_pg(ctx_ptr)
    }
}

#[pg_extern(
    sql = "CREATE FUNCTION chamkho_parser_get_token(internal, internal, internal) RETURNS internal LANGUAGE C STRICT AS 'MODULE_PATHNAME', 'chamkho_parser_get_token_wrapper';"
)]
fn chamkho_parser_get_token(func_call_info: FunctionCallInfo) -> i32 {
    unsafe {
        let nargs = (*func_call_info).nargs as usize;
        let args: Vec<pg_sys::NullableDatum> = (0..nargs)
            .map(|i| *(*func_call_info).args.as_mut_ptr().add(i))
            .collect();

        let ctx = usize::from(args[0].value) as *mut ChamkhoParserContext;
        let token = usize::from(args[1].value) as *mut *const u8;
        let token_len = usize::from(args[2].value) as *mut i32;

        if !(*ctx).is_segmented {
            let text_slice = std::slice::from_raw_parts((*ctx).text_ptr, (*ctx).text_len as usize);
            let text = String::from_utf8_lossy(text_slice);
            let ranges = WORDCUT.segment_into_byte_ranges(&text);
            let ranges_len = ranges.len() as i32;
            let ranges_size = std::mem::size_of::<TextRange>() * ranges.len();
            let ranges_ptr = pg_sys::palloc(ranges_size) as *mut TextRange;
            for (i, r) in ranges.iter().enumerate() {
                core::ptr::write(ranges_ptr.add(i), TextRange { s: r.s, e: r.e });
            }
            (*ctx).ranges_ptr = ranges_ptr;
            (*ctx).ranges_len = ranges_len;
            (*ctx).is_segmented = true;
            (*ctx).word_idx = 0;
        }

        let word_idx = (*ctx).word_idx as i32;
        if word_idx >= (*ctx).ranges_len {
            *token_len = 0;
            (*ctx).is_segmented = false;
            0
        } else {
            let r = &*((*ctx).ranges_ptr.add(word_idx as usize));
            let len = (r.e - r.s) as i32;
            let buf = (*ctx).text_ptr.add(r.s);
            *token_len = len;
            *token = buf;
            (*ctx).word_idx += 1;
            let w = String::from_utf8_lossy(std::slice::from_raw_parts(buf, len as usize));
            if SPACE_RE.is_match(&w) {
                12
            } else {
                2
            }
        }
    }
}

#[pg_extern(
    sql = "CREATE FUNCTION chamkho_parser_end(internal) RETURNS void LANGUAGE C STRICT AS 'MODULE_PATHNAME', 'chamkho_parser_end_wrapper';"
)]
fn chamkho_parser_end(func_call_info: FunctionCallInfo) {
    unsafe {
        let args = (*func_call_info).args.as_mut_ptr();
        let ctx = usize::from((*args.add(0)).value) as *mut ChamkhoParserContext;
        if !(*ctx).text_ptr.is_null() {
            pg_sys::pfree((*ctx).text_ptr as *mut std::os::raw::c_void);
        }
        if !(*ctx).ranges_ptr.is_null() {
            pg_sys::pfree((*ctx).ranges_ptr as *mut std::os::raw::c_void);
        }
        pg_sys::pfree(ctx as *mut std::os::raw::c_void);
    }
}

extension_sql!(
    r#"CREATE TEXT SEARCH PARSER chamkho_parser (
    START    = chamkho_parser_start,
    GETTOKEN = chamkho_parser_get_token,
    END      = chamkho_parser_end,
    HEADLINE = pg_catalog.prsd_headline,
    LEXTYPES = prsd_lextype
);"#,
    name = "create_ts_parser",
    requires = [
        chamkho_parser_start,
        chamkho_parser_get_token,
        chamkho_parser_end
    ]
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_chamkho_parser() {
        Spi::run("CREATE TEXT SEARCH CONFIGURATION chamkho (PARSER = chamkho_parser);").unwrap();
        Spi::run("ALTER TEXT SEARCH CONFIGURATION chamkho ADD MAPPING FOR word WITH simple;")
            .unwrap();
        Spi::run("SELECT to_tsvector('chamkho', 'វេវចនានុក្រមពហុភាសាដោយឥតគិតថ្លៃฉันกินข้าวၵႂၢမ်းတႆးလိၵ်ႈတႆຸຊອກຫາອີ່ຫຍັງ本日のお仕事終了しましたpop musicရှေးန်မာမင်း အဆက်ဆက်ကတည်း');").unwrap();
    }

    #[pg_test]
    fn test_chamkho_parser_segmentation() {
        Spi::run("CREATE TEXT SEARCH CONFIGURATION chamkho (PARSER = chamkho_parser);").unwrap();
        Spi::run("ALTER TEXT SEARCH CONFIGURATION chamkho ADD MAPPING FOR word WITH simple;")
            .unwrap();

        let result: Option<i32> = Spi::get_one(
            "SELECT array_length(tsvector_to_array(to_tsvector('chamkho', 'แมวกินน้ำ')), 1)",
        )
        .unwrap();
        assert_eq!(result, Some(3), "Expected 3 words from 'แมวกินน้ำ'");
    }
}

#[cfg(feature = "pg_bench")]
#[pg_schema]
mod benches {
    use pgrx::prelude::*;
    use pgrx_bench::{black_box, Bencher};

    #[pg_bench]
    fn bench_hello_chamkho_parser(b: &mut Bencher) {
        // b.iter(|| {
        //     black_box(crate::hello_chamkho_parser());
        // });
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
