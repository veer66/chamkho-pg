extern crate pg_extend;
extern crate pg_extern_attr;

use pg_extend::pg_magic;
use pg_extern_attr::pg_extern;

pg_magic!(version: pg_sys::PG_VERSION_NUM);

#[pg_extern]
fn add_one(value: i32) -> i32 {
    value + 1
}
