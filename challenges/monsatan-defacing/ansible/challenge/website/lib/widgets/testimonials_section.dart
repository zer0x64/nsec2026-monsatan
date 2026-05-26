import 'package:carousel_slider/carousel_slider.dart';
import 'package:flutter/material.dart';

import 'package:google_fonts/google_fonts.dart';
import 'package:visibility_detector/visibility_detector.dart';
import '../data/testimonials_data.dart';
import '../models/testimonial.dart';
import '../theme.dart';

/// Testimonials section with an auto-advancing carousel of customer quotes.
///
/// Each quote reads like a glowing endorsement while revealing the quiet
/// horror of life under a Monsatan Corp contract.
class TestimonialsSection extends StatefulWidget {
  const TestimonialsSection({super.key});

  @override
  State<TestimonialsSection> createState() => _TestimonialsSectionState();
}

class _TestimonialsSectionState extends State<TestimonialsSection> {
  bool _visible = false;

  /// Currently highlighted carousel index, kept in sync with the
  /// [CarouselSliderController] so the dot indicator stays accurate.
  int _currentIndex = 0;

  final CarouselSliderController _carouselController =
      CarouselSliderController();

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    return VisibilityDetector(
      key: const Key('testimonials-section'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.1 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: Container(
        color: MonsatanColors.surfaceCard,
        padding: const EdgeInsets.symmetric(vertical: 120),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 1100),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 32),
              child: Column(
                children: [
                  _SectionHeader(visible: _visible),
                  const SizedBox(height: 64),
                  _Carousel(
                    visible: _visible,
                    isWide: isWide,
                    controller: _carouselController,
                    currentIndex: _currentIndex,
                    onPageChanged: (index) =>
                        setState(() => _currentIndex = index),
                  ),
                  const SizedBox(height: 32),
                  // Dot indicator + manual nav buttons
                  _CarouselControls(
                    visible: _visible,
                    count: kTestimonials.length,
                    currentIndex: _currentIndex,
                    onPrevious: () => _carouselController.previousPage(),
                    onNext: () => _carouselController.nextPage(),
                    onDotTap: (i) => _carouselController.animateToPage(i),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Section header
// ---------------------------------------------------------------------------

class _SectionHeader extends StatelessWidget {
  final bool visible;
  const _SectionHeader({required this.visible});

  @override
  Widget build(BuildContext context) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      opacity: visible ? 1.0 : 0.0,
      child: Column(
        children: [
          // Small label
          Text(
            'TESTIMONIALS',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.accentGreen,
              fontSize: 12,
              fontWeight: FontWeight.w700,
              letterSpacing: 3,
            ),
          ),
          const SizedBox(height: 8),
          // Gold underline accent centred below the label
          Center(
            child: Container(
              width: 40,
              height: 2,
              color: MonsatanColors.solarGold,
            ),
          ),
          const SizedBox(height: 20),
          Text(
            'Verified™ Customer Voices',
            style: GoogleFonts.playfairDisplay(
              color: MonsatanColors.textPrimary,
              fontSize: 36,
              fontWeight: FontWeight.w700,
            ),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: 16),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 560),
            child: Text(
              'These testimonials were collected voluntarily by our '
              'customer outreach team. Participation in the programme '
              'does not affect contract terms, pending audits, or '
              'ongoing litigation.',
              style: GoogleFonts.dmSans(
                color: MonsatanColors.textSecondary,
                fontSize: 15,
                height: 1.75,
              ),
              textAlign: TextAlign.center,
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Carousel
// ---------------------------------------------------------------------------

class _Carousel extends StatelessWidget {
  final bool visible;
  final bool isWide;
  final CarouselSliderController controller;
  final int currentIndex;
  final ValueChanged<int> onPageChanged;

  const _Carousel({
    required this.visible,
    required this.isWide,
    required this.controller,
    required this.currentIndex,
    required this.onPageChanged,
  });

  @override
  Widget build(BuildContext context) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 800),
      opacity: visible ? 1.0 : 0.0,
      child: CarouselSlider.builder(
        carouselController: controller,
        itemCount: kTestimonials.length,
        itemBuilder: (context, index, realIndex) {
          return _TestimonialCard(testimonial: kTestimonials[index]);
        },
        options: CarouselOptions(
          height: isWide ? 340 : 420,
          viewportFraction: isWide ? 0.7 : 0.92,
          enlargeCenterPage: true,
          enlargeFactor: 0.15,
          autoPlay: true,
          autoPlayInterval: const Duration(seconds: 8),
          autoPlayAnimationDuration: const Duration(milliseconds: 400),
          autoPlayCurve: Curves.easeInOut,
          onPageChanged: (index, _) => onPageChanged(index),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Testimonial card
// ---------------------------------------------------------------------------

/// A single testimonial card displaying quote, author, role, and region.
class _TestimonialCard extends StatefulWidget {
  final Testimonial testimonial;
  const _TestimonialCard({required this.testimonial});

  @override
  State<_TestimonialCard> createState() => _TestimonialCardState();
}

class _TestimonialCardState extends State<_TestimonialCard> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 250),
        margin: const EdgeInsets.symmetric(vertical: 8, horizontal: 8),
        padding: const EdgeInsets.all(40),
        decoration: BoxDecoration(
          color: MonsatanColors.darkBackground,
          border: Border.all(
            color: _hovered
                ? MonsatanColors.solarGold.withValues(alpha: 0.45)
                : MonsatanColors.surfaceBorder,
            width: 1,
          ),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Large decorative opening quote mark
            Text(
              '\u201C',
              style: GoogleFonts.playfairDisplay(
                color: MonsatanColors.solarGold.withValues(alpha: 0.35),
                fontSize: 80,
                height: 0.6,
                fontWeight: FontWeight.w700,
              ),
            ),
            const SizedBox(height: 16),

            // Quote body
            Expanded(
              child: Text(
                widget.testimonial.quote,
                style: GoogleFonts.playfairDisplay(
                  color: MonsatanColors.textPrimary,
                  fontSize: 17,
                  height: 1.7,
                  fontStyle: FontStyle.italic,
                ),
                overflow: TextOverflow.fade,
              ),
            ),

            const SizedBox(height: 24),

            // Divider
            Container(
              height: 1,
              color: MonsatanColors.surfaceBorder,
              margin: const EdgeInsets.only(bottom: 20),
            ),

            // Author attribution row
            Row(
              children: [
                // Avatar circle with initials
                _AuthorAvatar(name: widget.testimonial.author),
                const SizedBox(width: 16),
                // Name, role, and region
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        widget.testimonial.author,
                        style: GoogleFonts.dmSans(
                          color: MonsatanColors.textPrimary,
                          fontSize: 14,
                          fontWeight: FontWeight.w700,
                        ),
                      ),
                      const SizedBox(height: 2),
                      Text(
                        widget.testimonial.role,
                        style: GoogleFonts.dmSans(
                          color: MonsatanColors.accentGreen,
                          fontSize: 12,
                          letterSpacing: 0.5,
                        ),
                      ),
                      const SizedBox(height: 2),
                      Text(
                        widget.testimonial.region,
                        style: GoogleFonts.dmSans(
                          color: MonsatanColors.textMuted,
                          fontSize: 11,
                          letterSpacing: 0.5,
                        ),
                      ),
                    ],
                  ),
                ),
                // Verified badge
                _VerifiedBadge(),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

/// A small circle avatar showing the author's first initial.
class _AuthorAvatar extends StatelessWidget {
  final String name;
  const _AuthorAvatar({required this.name});

  @override
  Widget build(BuildContext context) {
    // Use first character, fall back to '?' for anonymous entries.
    final initial = name.isNotEmpty ? name[0].toUpperCase() : '?';

    return Container(
      width: 42,
      height: 42,
      decoration: BoxDecoration(
        color: MonsatanColors.primaryGreen.withValues(alpha: 0.4),
        shape: BoxShape.circle,
        border: Border.all(color: MonsatanColors.surfaceBorder, width: 1),
      ),
      child: Center(
        child: Text(
          initial,
          style: GoogleFonts.playfairDisplay(
            color: MonsatanColors.textPrimary,
            fontSize: 18,
            fontWeight: FontWeight.w600,
          ),
        ),
      ),
    );
  }
}

/// Small "VERIFIED™" badge shown on each testimonial card.
class _VerifiedBadge extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        border: Border.all(
          color: MonsatanColors.accentGreen.withValues(alpha: 0.4),
          width: 1,
        ),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          const Icon(
            Icons.check_circle_outline,
            color: MonsatanColors.accentGreen,
            size: 11,
          ),
          const SizedBox(width: 4),
          Text(
            'VERIFIED™',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.accentGreen,
              fontSize: 9,
              fontWeight: FontWeight.w700,
              letterSpacing: 1.5,
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Carousel controls
// ---------------------------------------------------------------------------

/// Dot indicator and previous / next navigation for the testimonials carousel.
class _CarouselControls extends StatelessWidget {
  final bool visible;
  final int count;
  final int currentIndex;
  final VoidCallback onPrevious;
  final VoidCallback onNext;
  final ValueChanged<int> onDotTap;

  const _CarouselControls({
    required this.visible,
    required this.count,
    required this.currentIndex,
    required this.onPrevious,
    required this.onNext,
    required this.onDotTap,
  });

  @override
  Widget build(BuildContext context) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 600),
      opacity: visible ? 1.0 : 0.0,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          // Previous button
          _NavButton(icon: Icons.arrow_back_ios, onTap: onPrevious),
          const SizedBox(width: 24),

          // Dot indicators
          Row(
            mainAxisSize: MainAxisSize.min,
            children: List.generate(count, (index) {
              final isActive = index == currentIndex;
              return GestureDetector(
                onTap: () => onDotTap(index),
                child: AnimatedContainer(
                  duration: const Duration(milliseconds: 300),
                  width: isActive ? 24 : 8,
                  height: 8,
                  margin: const EdgeInsets.symmetric(horizontal: 4),
                  decoration: BoxDecoration(
                    color: isActive
                        ? MonsatanColors.solarGold
                        : MonsatanColors.surfaceBorder,
                    borderRadius: BorderRadius.circular(4),
                  ),
                ),
              );
            }),
          ),

          const SizedBox(width: 24),

          // Next button
          _NavButton(icon: Icons.arrow_forward_ios, onTap: onNext),
        ],
      ),
    );
  }
}

/// A small square navigation button used by [_CarouselControls].
class _NavButton extends StatefulWidget {
  final IconData icon;
  final VoidCallback onTap;

  const _NavButton({required this.icon, required this.onTap});

  @override
  State<_NavButton> createState() => _NavButtonState();
}

class _NavButtonState extends State<_NavButton> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: widget.onTap,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          width: 40,
          height: 40,
          decoration: BoxDecoration(
            color: _hovered
                ? MonsatanColors.primaryGreen.withValues(alpha: 0.3)
                : Colors.transparent,
            border: Border.all(
              color: _hovered
                  ? MonsatanColors.accentGreen
                  : MonsatanColors.surfaceBorder,
              width: 1,
            ),
          ),
          child: Icon(
            widget.icon,
            color: _hovered
                ? MonsatanColors.solarGold
                : MonsatanColors.textSecondary,
            size: 14,
          ),
        ),
      ),
    );
  }
}
