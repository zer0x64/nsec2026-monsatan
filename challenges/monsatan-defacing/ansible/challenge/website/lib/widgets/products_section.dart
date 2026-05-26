import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:is_even/is_even.dart';
import 'package:visibility_detector/visibility_detector.dart';
import '../data/products_data.dart';
import '../models/product.dart';
import '../theme.dart';

/// Products section displaying the full Monsatan Corp product lineup.
///
/// Products are shown in a responsive grid of [_ProductCard] widgets.
/// The entire section fades and slides in when it enters the viewport.
class ProductsSection extends StatefulWidget {
  const ProductsSection({super.key});

  @override
  State<ProductsSection> createState() => _ProductsSectionState();
}

class _ProductsSectionState extends State<ProductsSection> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    return VisibilityDetector(
      key: const Key('products-section'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.05 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: Container(
        color: MonsatanColors.surfaceCard,
        padding: const EdgeInsets.symmetric(vertical: 120),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 1200),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 32),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Section header
                  _SectionHeader(visible: _visible),
                  const SizedBox(height: 64),
                  // Responsive product grid
                  _ProductGrid(visible: _visible),
                  const SizedBox(height: 48),
                  // Fine-print legal disclaimer at the bottom
                  _LegalFootnote(visible: _visible),
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
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      opacity: visible ? 1 : 0,
      child: isWide
          ? Row(
              crossAxisAlignment: CrossAxisAlignment.end,
              children: [_titleBlock(), const Spacer(), _subtitleBlock()],
            )
          : Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _titleBlock(),
                const SizedBox(height: 20),
                _subtitleBlock(),
              ],
            ),
    );
  }

  Widget _titleBlock() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Small label
        Text(
          'OUR PRODUCTS',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.accentGreen,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 3,
          ),
        ),
        const SizedBox(height: 8),
        Container(width: 40, height: 2, color: MonsatanColors.solarGold),
        const SizedBox(height: 20),
        Text(
          'The Future of Food.\nTrademarked.',
          style: GoogleFonts.playfairDisplay(
            color: MonsatanColors.textPrimary,
            fontSize: 40,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
      ],
    );
  }

  Widget _subtitleBlock() {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 420),
      child: Text(
        'Every Monsatan product is the result of decades of proprietary '
        'research, thousands of patent filings, and an unwavering belief '
        'that a better harvest is simply a matter of chemistry — and '
        'enforceable contract law.',
        style: GoogleFonts.dmSans(
          color: MonsatanColors.textSecondary,
          fontSize: 15,
          height: 1.75,
        ),
        textAlign: TextAlign.left,
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Product grid
// ---------------------------------------------------------------------------

class _ProductGrid extends StatelessWidget {
  final bool visible;
  const _ProductGrid({required this.visible});

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;

    // Column count based on available width.
    final int columns;
    if (width > Breakpoints.tablet) {
      columns = 3;
    } else if (width > Breakpoints.mobile) {
      columns = 2;
    } else {
      columns = 1;
    }

    return GridView.builder(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: columns,
        crossAxisSpacing: 24,
        mainAxisSpacing: 24,
        // Taller cards on fewer columns to keep content readable.
        childAspectRatio: columns == 1 ? 1.1 : 0.75,
      ),
      itemCount: kProducts.length,
      itemBuilder: (context, index) {
        return _ProductCard(
          product: kProducts[index],
          index: index,
          visible: visible,
          delay: Duration(milliseconds: 80 * index),
        );
      },
    );
  }
}

// ---------------------------------------------------------------------------
// Product card
// ---------------------------------------------------------------------------

/// An individual product card with emoji icon, features list, and disclaimer.
///
/// [index] is used with [isEven] to alternate the feature-dot accent colour
/// between solar gold (even) and accent green (odd).
class _ProductCard extends StatefulWidget {
  final Product product;
  final int index;
  final bool visible;
  final Duration delay;

  const _ProductCard({
    required this.product,
    required this.index,
    required this.visible,
    required this.delay,
  });

  @override
  State<_ProductCard> createState() => _ProductCardState();
}

class _ProductCardState extends State<_ProductCard> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      opacity: widget.visible ? 1.0 : 0.0,
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 250),
          decoration: BoxDecoration(
            color: MonsatanColors.darkBackground,
            border: Border.all(
              color: _hovered
                  ? MonsatanColors.accentGreen
                  : MonsatanColors.surfaceBorder,
              width: 1,
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Top colour band with emoji icon
              _CardHeader(
                emoji: widget.product.emoji,
                tagline: widget.product.tagline,
                hovered: _hovered,
              ),
              // Main card body
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(24, 20, 24, 16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Product name
                      Text(
                        widget.product.name,
                        style: GoogleFonts.playfairDisplay(
                          color: MonsatanColors.textPrimary,
                          fontSize: 22,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(height: 10),
                      // Short description
                      Text(
                        widget.product.description,
                        style: GoogleFonts.dmSans(
                          color: MonsatanColors.textSecondary,
                          fontSize: 13,
                          height: 1.65,
                        ),
                        maxLines: 4,
                        overflow: TextOverflow.ellipsis,
                      ),
                      const SizedBox(height: 16),
                      // Features list
                      ...widget.product.features.map(
                        (f) => _FeatureRow(
                          text: f,
                          // Alternate dot colour by card position.
                          dotColor: isEven(widget.index)
                              ? MonsatanColors.solarGold
                              : MonsatanColors.accentGreen,
                        ),
                      ),
                      const Spacer(),
                      // Divider before disclaimer
                      Container(
                        height: 1,
                        color: MonsatanColors.surfaceBorder,
                        margin: const EdgeInsets.symmetric(vertical: 12),
                      ),
                      // Legal disclaimer in tiny text
                      Text(
                        widget.product.disclaimer,
                        style: GoogleFonts.dmSans(
                          color: MonsatanColors.textMuted,
                          fontSize: 10,
                          height: 1.5,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// Top banner of a product card — coloured accent strip with emoji and tagline.
class _CardHeader extends StatelessWidget {
  final String emoji;
  final String tagline;
  final bool hovered;

  const _CardHeader({
    required this.emoji,
    required this.tagline,
    required this.hovered,
  });

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 250),
      height: 80,
      padding: const EdgeInsets.symmetric(horizontal: 24),
      decoration: BoxDecoration(
        color: hovered
            ? MonsatanColors.primaryGreen.withValues(alpha: 0.35)
            : MonsatanColors.primaryGreen.withValues(alpha: 0.15),
        border: Border(
          bottom: BorderSide(
            color: hovered
                ? MonsatanColors.accentGreen
                : MonsatanColors.surfaceBorder,
            width: 1,
          ),
        ),
      ),
      child: Row(
        children: [
          // Emoji as product icon
          Text(emoji, style: const TextStyle(fontSize: 32)),
          const SizedBox(width: 16),
          // Tagline
          Text(
            tagline.toUpperCase(),
            style: GoogleFonts.dmSans(
              color: MonsatanColors.solarGold,
              fontSize: 11,
              fontWeight: FontWeight.w700,
              letterSpacing: 2.5,
            ),
          ),
        ],
      ),
    );
  }
}

/// A bullet-point feature row with a small accent dot.
///
/// [dotColor] is supplied by the parent card and alternates between
/// [MonsatanColors.solarGold] and [MonsatanColors.accentGreen] via [isEven].
class _FeatureRow extends StatelessWidget {
  final String text;
  final Color dotColor;
  const _FeatureRow({required this.text, required this.dotColor});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 6),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.only(top: 6, right: 8),
            child: Container(
              width: 5,
              height: 5,
              decoration: BoxDecoration(
                color: dotColor,
                shape: BoxShape.circle,
              ),
            ),
          ),
          Expanded(
            child: Text(
              text,
              style: GoogleFonts.dmSans(
                color: MonsatanColors.textSecondary,
                fontSize: 13,
                height: 1.5,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Legal footnote
// ---------------------------------------------------------------------------

/// Full-width fine-print note at the bottom of the products section.
class _LegalFootnote extends StatelessWidget {
  final bool visible;
  const _LegalFootnote({required this.visible});

  @override
  Widget build(BuildContext context) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 800),
      opacity: visible ? 1.0 : 0.0,
      child: Container(
        padding: const EdgeInsets.all(20),
        decoration: BoxDecoration(
          border: Border.all(color: MonsatanColors.surfaceBorder, width: 1),
        ),
        child: Text(
          'All Monsatan products are protected by international patent law. '
          'Cultivation, reproduction, or reverse-engineering of any Monsatan '
          'genome is prohibited without a valid licence agreement. Monsatan '
          'Corp reserves the right to conduct field audits at any time. '
          'By planting our seeds, you agree to the Monsatan Grower EULA '
          '(available on request; acceptance is automatic upon germination).',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 11,
            height: 1.65,
          ),
          textAlign: TextAlign.center,
        ),
      ),
    );
  }
}
