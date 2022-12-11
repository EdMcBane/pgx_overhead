use pgx::IntoDatum;
use pgx::prelude::*;
use pgx::utils::sql_entity_graph::metadata::{ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable};
use crate::pg_sys::{Datum, Oid};

pgx::pg_module_magic!();

static VAL: &'static str = "HelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHello";

/// Unguarded pg_extern; since the `no_guard` attribute on `pg_extern` is broken, we extern it manually
fn hello_unguarded_void() {
    // Nothing
}

#[no_mangle]
#[doc(hidden)]
pub unsafe extern "C" fn hello_unguarded_void_wrapper(_fcinfo: ::pgx::pg_sys::FunctionCallInfo) { #[allow(unused_unsafe)]unsafe { hello_unguarded_void() } }

#[no_mangle]
#[doc(hidden)]
pub extern "Rust" fn __pgx_internals_fn_hello_unguarded_void() -> ::pgx::utils::sql_entity_graph::SqlGraphEntity {
    extern crate alloc;
    #[allow(unused_imports)]
    use alloc::{vec, vec::Vec};
    type FunctionPointer = fn();
    let metadata: FunctionPointer = hello_unguarded_void;
    let submission = ::pgx::utils::sql_entity_graph::PgExternEntity {
        name: "hello_unguarded_void",
        unaliased_name: stringify!(hello_unguarded_void ),
        module_path: core::module_path!(),
        full_path: concat!(core::module_path!(), "::", stringify!(hello_unguarded_void )),
        metadata: pgx::utils::sql_entity_graph::metadata::FunctionMetadata::entity(&metadata),
        fn_args: vec![],
        fn_return: ::pgx::utils::sql_entity_graph::PgExternReturnEntity::None,
        #[allow(clippy::or_fun_call)] schema: None,
        file: file!(),
        line: line!(),
        extern_attrs: vec![::pgx::utils::ExternArgs::NoGuard],
        #[allow(clippy::or_fun_call)] search_path: None,
        #[allow(clippy::or_fun_call)] operator: None,
        to_sql_config: ::pgx::utils::sql_entity_graph::ToSqlConfigEntity { enabled: true, callback: None, content: None },
    };
    ::pgx::utils::sql_entity_graph::SqlGraphEntity::Function(submission)
}
#[no_mangle]
#[doc(hidden)]
pub extern "C" fn pg_finfo_hello_unguarded_void_wrapper() -> &'static pg_sys::Pg_finfo_record {
    const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
    &V1_API
}

#[pg_extern]
fn hello_void() {
    // nothing
}

#[pg_extern]
fn hello_i32() -> i32 {
    42
}

#[pg_extern]
fn hello_str() -> &'static str {
    VAL
}

#[pg_extern]
fn hello_fastr() -> Fastring {
    Fastring(VAL)
}

#[repr(transparent)]
struct Fastring(&'static str);

impl IntoDatum for Fastring {
    fn into_datum(self) -> Option<Datum> {
        str_to_datum(self.0)
    }

    fn type_oid() -> Oid {
        pg_sys::TEXTOID
    }
}

unsafe impl SqlTranslatable for Fastring {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        <&str as SqlTranslatable>::argument_sql()
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        <&str as SqlTranslatable>::return_sql()
    }
}

/// Improved IntoDatum impl recently implemented by @eeeebbbbrrrr
fn str_to_datum(v: &str) -> Option<pgx::pg_sys::Datum> {
    let len = pg_sys::VARHDRSZ + v.len();
    unsafe {
        // SAFETY:  palloc gives us a valid pointer if if there's not enough memory it'll raise an error
        let varlena = pg_sys::palloc(len) as *mut pg_sys::varlena;

        // SAFETY: `varlena` can properly cast into a `varattrib_4b` and all of what it contains is properly
        // allocated thanks to our call to `palloc` above
        let varattrib_4b = varlena
            .cast::<pg_sys::varattrib_4b>()
            .as_mut()
            .unwrap_unchecked()
            .va_4byte
            .as_mut();
        varattrib_4b.va_header = <usize as TryInto<u32>>::try_into(len)
            .expect("Rust string too large for a Postgres varlena datum")
            << 2u32;

        // SAFETY: src and dest pointers are valid and are exactly `v.len()` bytes long
        std::ptr::copy_nonoverlapping(
            v.as_ptr().cast(),
            varattrib_4b.va_data.as_mut_ptr(),
            v.len(),
        );

        Some(Datum::from(varlena))
    }
}
