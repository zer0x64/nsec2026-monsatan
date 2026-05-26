import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:google_fonts/google_fonts.dart';
import '../theme.dart';

/// Height of the navigation bar in logical pixels.
const double kNavBarHeight = 72.0;

/// Sticky top navigation bar for the Monsatan Corp website.
///
/// Starts fully transparent over the hero section and transitions to a
/// semi-opaque dark background once the user scrolls down.
class NavBar extends StatefulWidget implements PreferredSizeWidget {
  final ScrollController scrollController;

  /// Callbacks wired up to scroll-to-section logic in the home page.
  final VoidCallback onProductsTap;
  final VoidCallback onTechnologyTap;
  final VoidCallback onAboutTap;

  const NavBar({
    super.key,
    required this.scrollController,
    required this.onProductsTap,
    required this.onTechnologyTap,
    required this.onAboutTap,
  });

  @override
  Size get preferredSize => const Size.fromHeight(kNavBarHeight);

  @override
  State<NavBar> createState() => _NavBarState();
}

class _NavBarState extends State<NavBar> {
  bool _isScrolled = false;

  /// Offset threshold (px) after which the bar becomes opaque.
  static const double _scrollThreshold = 20.0;

  @override
  void initState() {
    super.initState();
    widget.scrollController.addListener(_onScroll);
  }

  void _onScroll() {
    final scrolled = widget.scrollController.offset > _scrollThreshold;
    if (scrolled != _isScrolled) {
      setState(() => _isScrolled = scrolled);
    }
  }

  @override
  void dispose() {
    widget.scrollController.removeListener(_onScroll);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    return AnimatedContainer(
      duration: const Duration(milliseconds: 300),
      height: kNavBarHeight,
      decoration: BoxDecoration(
        color: _isScrolled
            ? MonsatanColors.darkBackground.withValues(alpha: 0.95)
            : Colors.transparent,
        border: _isScrolled
            ? const Border(
                bottom: BorderSide(
                  color: MonsatanColors.surfaceBorder,
                  width: 1,
                ),
              )
            : null,
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 48),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            const _MonsatanLogo(),
            const Spacer(),
            if (isWide) ...[
              _NavLink(label: 'PRODUCTS', onTap: widget.onProductsTap),
              const SizedBox(width: 36),
              _NavLink(label: 'TECHNOLOGY', onTap: widget.onTechnologyTap),
              const SizedBox(width: 36),
              _NavLink(label: 'ABOUT', onTap: widget.onAboutTap),
              const SizedBox(width: 48),
              ElevatedButton(
                onPressed: () => context.go('/partner'),
                child: const Text('PARTNER WITH US'),
              ),
            ] else
              // Simplified mobile menu trigger (no drawer implemented yet)
              const Icon(Icons.menu, color: MonsatanColors.textPrimary),
          ],
        ),
      ),
    );
  }
}

/// Monsatan Corp wordmark: gold circle with "M" + spaced logotype.
class _MonsatanLogo extends StatelessWidget {
  const _MonsatanLogo();

  @override
  Widget build(BuildContext context) {
    return const Image(image: AssetImage('assets/logo.png'));
  }
}

/// A single navigation link that highlights on hover.
class _NavLink extends StatefulWidget {
  final String label;
  final VoidCallback onTap;

  const _NavLink({required this.label, required this.onTap});

  @override
  State<_NavLink> createState() => _NavLinkState();
}

class _NavLinkState extends State<_NavLink> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedDefaultTextStyle(
          duration: const Duration(milliseconds: 200),
          style: GoogleFonts.dmSans(
            color: _hovered
                ? MonsatanColors.solarGold
                : MonsatanColors.textSecondary,
            fontSize: 13,
            fontWeight: FontWeight.w600,
            letterSpacing: 2.0,
          ),
          child: Text(widget.label),
        ),
      ),
    );
  }
}
