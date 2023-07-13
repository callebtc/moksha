// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.77.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';

class NativePlatform extends FlutterRustBridgeBase<NativeWire>
    with FlutterRustBridgeSetupMixin {
  NativePlatform(FutureOr<WasmModule> dylib) : super(NativeWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  Object api2wire_u64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer
}

// Section: WASM wire module

@JS('wasm_bindgen')
external NativeWasmModule get wasmModule;

@JS()
@anonymous
class NativeWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external NativeWasmModule bind(dynamic thisArg, String moduleName);
  external dynamic /* void */ wire_init_cashu(NativePortType port_);

  external dynamic /* void */ wire_get_cashu_balance(NativePortType port_);

  external dynamic /* void */ wire_cashu_mint_tokens(
      NativePortType port_, Object amount, String hash);

  external dynamic /* void */ wire_get_cashu_mint_payment_request(
      NativePortType port_, Object amount);

  external dynamic /* void */ wire_get_fedimint_payment_request(
      NativePortType port_, Object amount);

  external dynamic /* void */ wire_fedimint_mint_tokens(
      NativePortType port_, Object amount, String operation_id);

  external dynamic /* void */ wire_decode_invoice(
      NativePortType port_, String invoice);

  external dynamic /* void */ wire_pay_invoice(
      NativePortType port_, String invoice);

  external dynamic /* void */ wire_import_token(
      NativePortType port_, String token);

  external dynamic /* void */ wire_join_federation(
      NativePortType port_, String federation);

  external dynamic /* void */ wire_get_fedimint_balance(NativePortType port_);
}

// Section: WASM wire connector

class NativeWire extends FlutterRustBridgeWasmWireBase<NativeWasmModule> {
  NativeWire(FutureOr<WasmModule> module)
      : super(WasmModule.cast<NativeWasmModule>(module));

  void wire_init_cashu(NativePortType port_) =>
      wasmModule.wire_init_cashu(port_);

  void wire_get_cashu_balance(NativePortType port_) =>
      wasmModule.wire_get_cashu_balance(port_);

  void wire_cashu_mint_tokens(
          NativePortType port_, Object amount, String hash) =>
      wasmModule.wire_cashu_mint_tokens(port_, amount, hash);

  void wire_get_cashu_mint_payment_request(
          NativePortType port_, Object amount) =>
      wasmModule.wire_get_cashu_mint_payment_request(port_, amount);

  void wire_get_fedimint_payment_request(NativePortType port_, Object amount) =>
      wasmModule.wire_get_fedimint_payment_request(port_, amount);

  void wire_fedimint_mint_tokens(
          NativePortType port_, Object amount, String operation_id) =>
      wasmModule.wire_fedimint_mint_tokens(port_, amount, operation_id);

  void wire_decode_invoice(NativePortType port_, String invoice) =>
      wasmModule.wire_decode_invoice(port_, invoice);

  void wire_pay_invoice(NativePortType port_, String invoice) =>
      wasmModule.wire_pay_invoice(port_, invoice);

  void wire_import_token(NativePortType port_, String token) =>
      wasmModule.wire_import_token(port_, token);

  void wire_join_federation(NativePortType port_, String federation) =>
      wasmModule.wire_join_federation(port_, federation);

  void wire_get_fedimint_balance(NativePortType port_) =>
      wasmModule.wire_get_fedimint_balance(port_);
}
