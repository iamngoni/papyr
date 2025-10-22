// Not required for test files
// ignore_for_file: prefer_const_constructors
import 'package:paypr/paypr.dart';
import 'package:test/test.dart';

void main() {
  group('Paypr', () {
    test('can be instantiated', () {
      expect(Paypr(), isNotNull);
    });
  });
}
