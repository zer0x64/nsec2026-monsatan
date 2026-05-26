import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'pages/home_page.dart';
import 'pages/legal_page.dart';
import 'pages/partner_page.dart';
import 'pages/prospectus_page.dart';
import 'theme.dart';

/// Root application widget for the Monsatan Corp website.
///
/// Configures [GoRouter] for client-side URL routing and applies
/// the [MonsatanTheme] globally.
class MonsatanApp extends StatelessWidget {
  MonsatanApp({super.key});

  /// Client-side router. Routes:
  ///   - `/`           → [HomePage]
  ///   - `/prospectus` → [ProspectusPage]
  ///   - `/partner`    → [PartnerPage]
  ///   - `/legal/:doc` → [LegalPage]
  final GoRouter _router = GoRouter(
    routes: [
      GoRoute(
        path: '/',
        name: 'home',
        builder: (context, state) => const HomePage(),
      ),
      GoRoute(
        path: '/prospectus',
        name: 'prospectus',
        builder: (context, state) => const ProspectusPage(),
      ),
      GoRoute(
        path: '/partner',
        name: 'partner',
        builder: (context, state) => const PartnerPage(),
      ),
      GoRoute(
        path: '/legal/:doc',
        name: 'legal',
        builder: (context, state) =>
            LegalPage(doc: state.pathParameters['doc'] ?? ''),
      ),
    ],
  );

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'Monsatan Corp — Growing Tomorrow\'s World Today™',
      debugShowCheckedModeBanner: false,
      theme: MonsatanTheme.theme,
      routerConfig: _router,
    );
  }
}
