// Not required for test files
// ignore_for_file: prefer_const_constructors
import 'package:papyr/papyr.dart';
import 'package:test/test.dart';

void main() {
  group('Papyr', () {
    test('can be instantiated', () {
      expect(Papyr(), isNotNull);
    });
  });
}
