#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_init_cashu_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "init_cashu",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| init_cashu(),
    )
}
fn wire_get_cashu_balance_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "get_cashu_balance",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| get_cashu_balance(task_callback.stream_sink::<_, u64>()),
    )
}
fn wire_cashu_mint_tokens_impl(
    port_: MessagePort,
    amount: impl Wire2Api<u64> + UnwindSafe,
    hash: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "cashu_mint_tokens",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_amount = amount.wire2api();
            let api_hash = hash.wire2api();
            move |task_callback| {
                cashu_mint_tokens(task_callback.stream_sink::<_, u64>(), api_amount, api_hash)
            }
        },
    )
}
fn wire_get_cashu_mint_payment_request_impl(
    port_: MessagePort,
    amount: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "get_cashu_mint_payment_request",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_amount = amount.wire2api();
            move |task_callback| {
                get_cashu_mint_payment_request(
                    task_callback.stream_sink::<_, FlutterPaymentRequest>(),
                    api_amount,
                )
            }
        },
    )
}
fn wire_decode_invoice_impl(
    invoice: impl Wire2Api<String> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "decode_invoice",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_invoice = invoice.wire2api();
            decode_invoice(api_invoice)
        },
    )
}
fn wire_cashu_pay_invoice_impl(port_: MessagePort, invoice: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "cashu_pay_invoice",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| {
                cashu_pay_invoice(task_callback.stream_sink::<_, bool>(), api_invoice)
            }
        },
    )
}
fn wire_join_federation_impl(port_: MessagePort, federation: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "join_federation",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_federation = federation.wire2api();
            move |task_callback| join_federation(api_federation)
        },
    )
}
fn wire_get_fedimint_payment_request_impl(
    port_: MessagePort,
    amount: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "get_fedimint_payment_request",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_amount = amount.wire2api();
            move |task_callback| {
                get_fedimint_payment_request(
                    task_callback.stream_sink::<_, FedimintPaymentRequest>(),
                    api_amount,
                )
            }
        },
    )
}
fn wire_fedimint_mint_tokens_impl(
    port_: MessagePort,
    amount: impl Wire2Api<u64> + UnwindSafe,
    operation_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "fedimint_mint_tokens",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_amount = amount.wire2api();
            let api_operation_id = operation_id.wire2api();
            move |task_callback| {
                fedimint_mint_tokens(
                    task_callback.stream_sink::<_, u64>(),
                    api_amount,
                    api_operation_id,
                )
            }
        },
    )
}
fn wire_get_fedimint_balance_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "get_fedimint_balance",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| get_fedimint_balance(task_callback.stream_sink::<_, u64>()),
    )
}
fn wire_fedimint_pay_invoice_impl(port_: MessagePort, invoice: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "fedimint_pay_invoice",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_invoice = invoice.wire2api();
            move |task_callback| {
                fedimint_pay_invoice(task_callback.stream_sink::<_, bool>(), api_invoice)
            }
        },
    )
}
fn wire_receive_token_impl(port_: MessagePort, token: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "receive_token",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_token = token.wire2api();
            move |task_callback| receive_token(task_callback.stream_sink::<_, u64>(), api_token)
        },
    )
}
fn wire_get_btcprice_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "get_btcprice",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| get_btcprice(task_callback.stream_sink::<_, f64>()),
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for FedimintPaymentRequest {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.pr.into_into_dart().into_dart(),
            self.operation_id.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for FedimintPaymentRequest {}
impl rust2dart::IntoIntoDart<FedimintPaymentRequest> for FedimintPaymentRequest {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for FlutterInvoice {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.pr.into_into_dart().into_dart(),
            self.amount_sats.into_into_dart().into_dart(),
            self.expiry_time.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for FlutterInvoice {}
impl rust2dart::IntoIntoDart<FlutterInvoice> for FlutterInvoice {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for FlutterPaymentRequest {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.pr.into_into_dart().into_dart(),
            self.hash.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for FlutterPaymentRequest {}
impl rust2dart::IntoIntoDart<FlutterPaymentRequest> for FlutterPaymentRequest {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
