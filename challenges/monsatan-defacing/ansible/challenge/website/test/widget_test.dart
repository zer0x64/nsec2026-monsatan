// This is a basic Flutter widget test for the Monsatan Corp website.
//
// To perform an interaction with a widget in the test, use the WidgetTester
// utility in the flutter_test package.

import 'package:flutter_test/flutter_test.dart';
import 'package:monsatan_website/app.dart';

void main() {
  testWidgets('App renders without crashing', (WidgetTester tester) async {
    // Build the app and trigger a frame.
    await tester.pumpWidget(MonsatanApp());

    // Verify that the app boots and the Monsatan wordmark is present.
    expect(find.text('MONSATAN'), findsOneWidget);
  });
}
