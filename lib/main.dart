import 'package:flutter/material.dart';
import 'bridge_generated.dart';
import 'dart:io';
import 'dart:ffi';

void main() async {
  var dylib = DynamicLibrary.open("..\\..\\..\\..\\rust\\target\\debug\\rust.dll");
  var cli = RustImpl(dylib);
  print("Dylib Opened");
  var sink = cli.setSink();
  print("Library Sink Set");
  sink.listen((event) {
    print(event);
  });
  // Without this, library will not have set sink as static variable yet.
  // await Future.delayed(Duration(seconds: 1));
  print("Sink Event Handler set");
  await cli.trySink();
  print("CLI running");
}
