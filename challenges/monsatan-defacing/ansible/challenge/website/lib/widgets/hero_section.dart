import 'package:animated_text_kit/animated_text_kit.dart';
import 'package:flutter/material.dart';
import 'package:flutter_animate/flutter_animate.dart';
import 'package:google_fonts/google_fonts.dart';
import '../theme.dart';

/// Full-screen hero section with typewriter headline and subtle grid background.
///
/// Sets the tone: visually stunning solarpunk corporate aesthetic that slowly
/// reveals its sinister undertones through the statistics strip at the bottom.
class HeroSection extends StatelessWidget {
  /// Called when the "EXPLORE PRODUCTS" button is pressed.
  final VoidCallback? onProductsTap;

  /// Called when the "OUR RESEARCH" button is pressed.
  final VoidCallback? onTechnologyTap;

  const HeroSection({super.key, this.onProductsTap, this.onTechnologyTap});

  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    final isWide = size.width > Breakpoints.tablet;

    return SizedBox(
      height: size.height,
      child: Stack(
        children: [
          // Subtle green grid lines covering the full background.
          const Positioned.fill(child: _GridBackground()),

          // Bottom gradient fade into the next section.
          Positioned(
            bottom: 0,
            left: 0,
            right: 0,
            child: Container(
              height: 220,
              decoration: const BoxDecoration(
                gradient: LinearGradient(
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                  colors: [Colors.transparent, MonsatanColors.darkBackground],
                ),
              ),
            ),
          ),

          // Main content column, centred on screen.
          Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 980),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 32),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    // Establishment badge
                    _Badge(
                      label: 'ESTABLISHED 2031  ·  FEEDING THE FUTURE™',
                    ).animate().fadeIn(delay: 0.ms, duration: 600.ms),

                    const SizedBox(height: 36),

                    // Typewriter headline — the company's defining lie.
                    AnimatedTextKit(
                      animatedTexts: [
                        TypewriterAnimatedText(
                          'Nature Is Good.\nWe Make It Better.',
                          textStyle: GoogleFonts.playfairDisplay(
                            color: MonsatanColors.textPrimary,
                            fontSize: isWide ? 68 : 38,
                            fontWeight: FontWeight.w700,
                            height: 1.15,
                          ),
                          textAlign: TextAlign.center,
                          speed: const Duration(milliseconds: 25),
                        ),
                      ],
                      totalRepeatCount: 1,
                      displayFullTextOnTap: true,
                    ),

                    const SizedBox(height: 28),

                    // Subtitle — corporate sustainability speak.
                    ConstrainedBox(
                      constraints: const BoxConstraints(maxWidth: 680),
                      child: Text(
                        'Monsatan Corp pioneers next-generation agricultural '
                        'biotechnology, harmonising cutting-edge science with '
                        'the Earth\'s natural rhythms to deliver sustainable '
                        'abundance — on our terms.',
                        style: GoogleFonts.dmSans(
                          color: MonsatanColors.textSecondary,
                          fontSize: isWide ? 20 : 16,
                          height: 1.75,
                        ),
                        textAlign: TextAlign.center,
                      ),
                    ).animate().fadeIn(delay: 800.ms, duration: 700.ms),

                    const SizedBox(height: 48),

                    // CTA buttons
                    Wrap(
                      spacing: 16,
                      runSpacing: 16,
                      alignment: WrapAlignment.center,
                      children: [
                        ElevatedButton(
                          onPressed: onProductsTap,
                          child: const Text('EXPLORE PRODUCTS'),
                        ),
                        OutlinedButton(
                          onPressed: onTechnologyTap,
                          child: const Text('OUR RESEARCH'),
                        ),
                      ],
                    ).animate().fadeIn(delay: 1000.ms, duration: 600.ms),

                    const SizedBox(height: 72),

                    // Key metrics strip
                    const _StatsStrip().animate().fadeIn(
                      delay: 1200.ms,
                      duration: 800.ms,
                    ),
                  ],
                ),
              ),
            ),
          ),

          // Scroll-down chevron at the very bottom.
          Positioned(
            bottom: 32,
            left: 0,
            right: 0,
            child: const _ScrollIndicator().animate().fadeIn(
              delay: 1400.ms,
              duration: 500.ms,
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Supporting widgets
// ---------------------------------------------------------------------------

/// Small bordered badge used for the "Established …" line above the headline.
class _Badge extends StatelessWidget {
  final String label;
  const _Badge({required this.label});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        border: Border.all(
          color: MonsatanColors.solarGold.withValues(alpha: 0.55),
          width: 1,
        ),
      ),
      child: Text(
        label,
        style: GoogleFonts.dmSans(
          color: MonsatanColors.solarGold,
          fontSize: 11,
          letterSpacing: 3,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }
}

/// Four-column strip showing unsettling corporate statistics.
class _StatsStrip extends StatelessWidget {
  const _StatsStrip();

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.mobile;

    final stats = [
      _StatItem(value: '2.4B', label: 'Acres Under\nContract'),
      _StatItem(value: '94%', label: 'Global Corn\nMarket Share'),
      _StatItem(value: '∞', label: 'Active\nPatents'),
      _StatItem(value: '0', label: 'Pending Class\nAction Suits*'),
    ];

    return Column(
      children: [
        // Divider line above stats
        Container(
          height: 1,
          color: MonsatanColors.surfaceBorder,
          margin: const EdgeInsets.only(bottom: 28),
        ),
        isWide
            ? Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: _intersperse(stats, const _VerticalDivider()),
              )
            : Wrap(
                spacing: 24,
                runSpacing: 24,
                alignment: WrapAlignment.center,
                children: stats,
              ),
        const SizedBox(height: 12),
        Text(
          '* As of Q3 2037. Subject to change without notice.',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 10,
            letterSpacing: 1,
          ),
        ),
      ],
    );
  }

  /// Inserts [separator] between every element of [items].
  List<Widget> _intersperse(List<Widget> items, Widget separator) {
    final result = <Widget>[];
    for (var i = 0; i < items.length; i++) {
      result.add(items[i]);
      if (i < items.length - 1) result.add(separator);
    }
    return result;
  }
}

/// A single labelled statistic displayed in the hero strip.
class _StatItem extends StatelessWidget {
  final String value;
  final String label;

  const _StatItem({required this.value, required this.label});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 32),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            value,
            style: GoogleFonts.playfairDisplay(
              color: MonsatanColors.solarGold,
              fontSize: 36,
              fontWeight: FontWeight.w700,
            ),
          ),
          const SizedBox(height: 6),
          Text(
            label,
            textAlign: TextAlign.center,
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 12,
              letterSpacing: 1.5,
              height: 1.6,
            ),
          ),
        ],
      ),
    );
  }
}

/// Thin vertical rule used to separate stat items.
class _VerticalDivider extends StatelessWidget {
  const _VerticalDivider();

  @override
  Widget build(BuildContext context) {
    return Container(width: 1, height: 60, color: MonsatanColors.surfaceBorder);
  }
}

/// Downward chevron that bounces to hint at scrollable content below.
class _ScrollIndicator extends StatelessWidget {
  const _ScrollIndicator();

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          'SCROLL',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 10,
            letterSpacing: 3,
          ),
        ),
        const SizedBox(height: 6),
        const Icon(
          Icons.keyboard_arrow_down,
          color: MonsatanColors.textMuted,
          size: 20,
        ),
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// Grid background painter
// ---------------------------------------------------------------------------

/// Paints a faint green grid over the hero background, evoking a
/// solarpunk "blueprint of nature" aesthetic.
class _GridBackground extends StatelessWidget {
  const _GridBackground();

  @override
  Widget build(BuildContext context) {
    return CustomPaint(painter: _GridPainter(), child: const SizedBox.expand());
  }
}

class _GridPainter extends CustomPainter {
  /// Spacing between grid lines in logical pixels.
  static const double _spacing = 64.0;

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = MonsatanColors.primaryGreen.withValues(alpha: 0.1)
      ..strokeWidth = 1;

    // Vertical lines
    for (double x = 0; x < size.width; x += _spacing) {
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), paint);
    }

    // Horizontal lines
    for (double y = 0; y < size.height; y += _spacing) {
      canvas.drawLine(Offset(0, y), Offset(size.width, y), paint);
    }
  }

  @override
  bool shouldRepaint(_GridPainter oldDelegate) => false;
}
