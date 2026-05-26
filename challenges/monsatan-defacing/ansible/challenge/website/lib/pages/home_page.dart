import 'package:flutter/material.dart';
import '../widgets/nav_bar.dart';
import '../widgets/hero_section.dart';
import '../widgets/about_section.dart';
import '../widgets/products_section.dart';
import '../widgets/technology_section.dart';
import '../widgets/testimonials_section.dart';
import '../widgets/footer_section.dart';

/// The main (and only) page of the Monsatan Corp website.
///
/// A single scrollable column composed of all section widgets.
/// The [NavBar] receives callbacks that scroll to each section's
/// [GlobalKey] when the user clicks a navigation link.
class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final ScrollController _scrollController = ScrollController();

  // GlobalKeys allow us to locate each section's render box and scroll to it.
  final GlobalKey _productsKey = GlobalKey();
  final GlobalKey _technologyKey = GlobalKey();
  final GlobalKey _aboutKey = GlobalKey();
  final GlobalKey _footerKey = GlobalKey();

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  /// Smoothly scrolls to the widget identified by [key].
  void _scrollTo(GlobalKey key) {
    final context = key.currentContext;
    if (context == null) return;
    Scrollable.ensureVisible(
      context,
      duration: const Duration(milliseconds: 800),
      curve: Curves.easeInOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // The NavBar floats above the hero by using a Stack in the body.
      // We use extendBodyBehindAppBar so the hero fills the full viewport height.
      extendBodyBehindAppBar: true,
      appBar: NavBar(
        scrollController: _scrollController,
        onProductsTap: () => _scrollTo(_productsKey),
        onTechnologyTap: () => _scrollTo(_technologyKey),
        onAboutTap: () => _scrollTo(_aboutKey),
      ),
      body: SingleChildScrollView(
        controller: _scrollController,
        child: Column(
          children: [
            // Hero fills the full viewport; the nav bar overlays it.
            HeroSection(
              onProductsTap: () => _scrollTo(_productsKey),
              onTechnologyTap: () => _scrollTo(_technologyKey),
            ),

            // Products section
            KeyedSubtree(key: _productsKey, child: const ProductsSection()),

            // Technology section
            KeyedSubtree(key: _technologyKey, child: const TechnologySection()),

            // About section
            KeyedSubtree(key: _aboutKey, child: const AboutSection()),

            // Testimonials carousel
            const TestimonialsSection(),

            // Footer — receives section scroll callbacks for its navigation links.
            FooterSection(
              key: _footerKey,
              onAboutTap: () => _scrollTo(_aboutKey),
              onProductsTap: () => _scrollTo(_productsKey),
              onTechnologyTap: () => _scrollTo(_technologyKey),
            ),
          ],
        ),
      ),
    );
  }
}
