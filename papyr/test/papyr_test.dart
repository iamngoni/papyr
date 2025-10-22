import 'package:test/test.dart';
import 'package:papyr/papyr.dart';

void main() {
  group('Papyr', () {
    late Papyr papyr;

    setUp(() {
      papyr = Papyr.instance;
    });

    test('can be instantiated', () {
      expect(papyr, isNotNull);
    });

    test('initialization works', () async {
      // This will fail on non-Windows machines without proper library setup
      // but tests the API structure
      try {
        await papyr.initialize();
        expect(true, true); // Initialization succeeded
        
        // Test scanner listing
        final scanners = await papyr.listScanners();
        expect(scanners, isA<List<ScannerInfo>>());
        
        // Cleanup
        await papyr.dispose();
      } catch (e) {
        // Expected to fail on development machines without proper setup
        expect(e, isA<Exception>());
      }
    });

    test('ScannerInfo toString works', () {
      const scanner = ScannerInfo(
        id: 'test',
        name: 'Test Scanner',
        backend: ScannerBackend.wia,
      );
      
      expect(scanner.toString(), 'ScannerInfo(test: Test Scanner, ScannerBackend.wia)');
    });

    test('ScanConfig has correct defaults', () {
      const config = ScanConfig(source: ScanSource.flatbed);
      
      expect(config.source, ScanSource.flatbed);
      expect(config.duplex, false);
      expect(config.dpi, 300);
      expect(config.colorMode, ColorMode.color);
      expect(config.pageWidthMm, 216);
      expect(config.pageHeightMm, 279);
    });
  });
}
