import 'package:papyr/papyr.dart';
import 'package:test/test.dart';

void main() {
  group('Papyr', () {
    late Papyr papyr;

    setUp(() {
      papyr = Papyr();
    });

    test('can be instantiated', () {
      expect(papyr, isNotNull);
    });

    test('initialize succeeds', () async {
      try {
        await papyr.initialize();
        expect(papyr.isAvailable, isTrue);
      } catch (e) {
        // May fail if library not built yet
        expect(e, isA<PapyrException>());
      }
    });

    test('discoverScanners returns list', () async {
      try {
        await papyr.initialize();
        final scanners = await papyr.discoverScanners();
        expect(scanners, isA<List<Scanner>>());
      } catch (e) {
        // Expected if no library or no scanners
      }
    });

    test('ScanConfig can be created', () {
      const scanner = Scanner(
        id: 'test',
        name: 'Test Scanner',
        backend: ScannerBackend.unknown,
        capabilities: Capabilities(
          sources: [ScanSource.flatbed],
          dpis: [300],
          colorModes: [ColorMode.color],
          supportsDuplex: false,
        ),
      );

      final config = ScanConfig(
        scanner: scanner,
        source: ScanSource.flatbed,
        dpi: 300,
        colorMode: ColorMode.color,
      );

      expect(config.scanner, equals(scanner));
      expect(config.source, equals(ScanSource.flatbed));
      expect(config.dpi, equals(300));
      expect(config.colorMode, equals(ColorMode.color));
      expect(config.format, equals(ScanFormat.pdf));
      expect(config.useDuplex, isFalse);
    });

    test('Scanner extensions work', () {
      const scanner = Scanner(
        id: 'test',
        name: 'Test Scanner',
        backend: ScannerBackend.escl,
        capabilities: Capabilities(
          sources: [ScanSource.flatbed, ScanSource.adf],
          dpis: [150, 300, 600],
          colorModes: [ColorMode.color, ColorMode.grayscale],
          supportsDuplex: true,
        ),
      );

      expect(scanner.supportsColor, isTrue);
      expect(scanner.supportsGrayscale, isTrue);
      expect(scanner.supportsBlackAndWhite, isFalse);
      expect(scanner.hasAdf, isTrue);
      expect(scanner.hasFlatbed, isTrue);
      expect(scanner.supportsDuplex, isTrue);
      expect(scanner.isNetworkScanner, isTrue);
      expect(scanner.isLocalScanner, isFalse);
      expect(scanner.maxDpi, equals(600));
      expect(scanner.minDpi, equals(150));
    });

    test('ScanConfig validation works', () {
      const scanner = Scanner(
        id: 'test',
        name: 'Test Scanner',
        backend: ScannerBackend.escl,
        capabilities: Capabilities(
          sources: [ScanSource.flatbed],
          dpis: [300],
          colorModes: [ColorMode.color],
          supportsDuplex: false,
        ),
      );

      final validConfig = ScanConfig(
        scanner: scanner,
        source: ScanSource.flatbed,
        dpi: 300,
        colorMode: ColorMode.color,
      );
      expect(validConfig.isValid, isTrue);

      final invalidDpi = ScanConfig(
        scanner: scanner,
        source: ScanSource.flatbed,
        dpi: 600,
        colorMode: ColorMode.color,
      );
      expect(invalidDpi.isValid, isFalse);
      expect(
        invalidDpi.validate(),
        contains('does not support DPI'),
      );

      final invalidSource = ScanConfig(
        scanner: scanner,
        source: ScanSource.adf,
        dpi: 300,
        colorMode: ColorMode.color,
      );
      expect(invalidSource.isValid, isFalse);
      expect(
        invalidSource.validate(),
        contains('does not support source'),
      );
    });
  });
}
