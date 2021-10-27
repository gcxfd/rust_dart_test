import 'package:path/path.dart' as path;
import 'dart:ffi';
import 'dart:io' show Platform;
import 'dart:math';
import './cpp.dart' show NativeLibrary;

void init() {
  final fp =
      '/Users/z/Downloads/rust_dart_test/target/debug/librust_dart.dylib';
  final so = NativeLibrary(DynamicLibrary.open(fp));
  so.initDart(NativeApi.initializeApiDLData);
  print(so);
  while (true) {
    final rng = Random();
    final li = List<int>.generate(999999, (i) => rng.nextInt(256));

    so.gcBind(li);
  }
}
