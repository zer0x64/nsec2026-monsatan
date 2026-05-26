import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:google_fonts/google_fonts.dart';

import '../theme.dart';

/// Footer for the Monsatan Corp website.
///
/// Contains the company wordmark, navigation columns, a newsletter sign-up
/// field, and the obligatory wall of legal boilerplate that no one reads —
/// which is precisely how Monsatan prefers it.
///
/// Scroll-to-section callbacks ([onAboutTap], [onProductsTap],
/// [onTechnologyTap]) are forwarded to the relevant footer links so that
/// clicking them smoothly scrolls the home page instead of navigating away.
class FooterSection extends StatelessWidget {
  final VoidCallback? onAboutTap;
  final VoidCallback? onProductsTap;
  final VoidCallback? onTechnologyTap;

  const FooterSection({
    super.key,
    this.onAboutTap,
    this.onProductsTap,
    this.onTechnologyTap,
  });

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    // Build link lists here so they can reference the scroll callbacks.
    final companyLinks = [
      _FooterLink.callback('About Us', onAboutTap),
      _FooterLink.callback('Our Research', onTechnologyTap),
      const _FooterLink('Partner With Us', route: '/partner'),
    ];

    final productLinks = [
      _FooterLink.callback('RadiGrow™ Wheat', onProductsTap),
      _FooterLink.callback('HerbiShield™ Corn', onProductsTap),
      _FooterLink.callback('MegaYield™ Soy', onProductsTap),
      _FooterLink.callback('SunSeed™ Sunflower', onProductsTap),
      _FooterLink.callback('AquaRoot™ Rice', onProductsTap),
      _FooterLink.callback('NightShade™ Tomato', onProductsTap),
      _FooterLink.callback('MonsatanKill™ Herbicide', onProductsTap),
    ];

    return Container(
      color: const Color(0xFF050C07), // slightly darker than darkBackground
      child: Column(
        children: [
          // Top divider line
          Container(height: 1, color: MonsatanColors.surfaceBorder),

          // Main footer body
          Padding(
            padding: EdgeInsets.symmetric(
              horizontal: 48,
              vertical: isWide ? 80 : 48,
            ),
            child: Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 1200),
                child: isWide
                    ? _WideLayout(
                        companyLinks: companyLinks,
                        productLinks: productLinks,
                      )
                    : _NarrowLayout(
                        companyLinks: companyLinks,
                        productLinks: productLinks,
                      ),
              ),
            ),
          ),

          // Bottom bar: copyright + legal links
          _BottomBar(),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Layout variants
// ---------------------------------------------------------------------------

/// Desktop layout: logo column + three link columns + newsletter, side by side.
class _WideLayout extends StatelessWidget {
  final List<_FooterLink> companyLinks;
  final List<_FooterLink> productLinks;

  const _WideLayout({required this.companyLinks, required this.productLinks});

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Brand column (wider)
        Expanded(flex: 3, child: _BrandColumn()),
        const SizedBox(width: 48),
        // Link columns
        Expanded(
          flex: 2,
          child: _LinkColumn(title: 'COMPANY', links: companyLinks),
        ),
        const SizedBox(width: 24),
        Expanded(
          flex: 2,
          child: _LinkColumn(title: 'PRODUCTS', links: productLinks),
        ),
        const SizedBox(width: 24),
        Expanded(
          flex: 2,
          child: _LinkColumn(title: 'LEGAL', links: _kLegalLinks),
        ),
        const SizedBox(width: 48),
        // Newsletter
        Expanded(flex: 3, child: _NewsletterColumn()),
      ],
    );
  }
}

/// Mobile layout: stacked columns with wrapping link grid.
class _NarrowLayout extends StatelessWidget {
  final List<_FooterLink> companyLinks;
  final List<_FooterLink> productLinks;

  const _NarrowLayout({required this.companyLinks, required this.productLinks});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _BrandColumn(),
        const SizedBox(height: 48),
        Wrap(
          spacing: 32,
          runSpacing: 32,
          children: [
            _LinkColumn(title: 'COMPANY', links: companyLinks),
            _LinkColumn(title: 'PRODUCTS', links: productLinks),
            _LinkColumn(title: 'LEGAL', links: _kLegalLinks),
          ],
        ),
        const SizedBox(height: 48),
        _NewsletterColumn(),
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// Brand column
// ---------------------------------------------------------------------------

class _BrandColumn extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Wordmark
        Row(
          children: [
            Container(
              width: 32,
              height: 32,
              decoration: const BoxDecoration(
                color: MonsatanColors.solarGold,
                shape: BoxShape.circle,
              ),
              child: Center(
                child: Text(
                  'M',
                  style: GoogleFonts.playfairDisplay(
                    color: MonsatanColors.darkBackground,
                    fontWeight: FontWeight.w900,
                    fontSize: 19,
                  ),
                ),
              ),
            ),
            const SizedBox(width: 10),
            RichText(
              text: TextSpan(
                children: [
                  TextSpan(
                    text: 'MONSATAN',
                    style: GoogleFonts.dmSans(
                      color: MonsatanColors.textPrimary,
                      fontSize: 16,
                      fontWeight: FontWeight.w800,
                      letterSpacing: 3,
                    ),
                  ),
                  TextSpan(
                    text: ' CORP',
                    style: GoogleFonts.dmSans(
                      color: MonsatanColors.solarGold,
                      fontSize: 16,
                      fontWeight: FontWeight.w300,
                      letterSpacing: 3,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),

        const SizedBox(height: 20),

        // Tagline
        Text(
          'Growing Tomorrow\'s World Today™',
          style: GoogleFonts.playfairDisplay(
            color: MonsatanColors.textSecondary,
            fontSize: 14,
            fontStyle: FontStyle.italic,
          ),
        ),

        const SizedBox(height: 20),

        // Short blurb
        ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 320),
          child: Text(
            'Monsatan Corp is the world\'s leading provider of patented '
            'agricultural biotechnology solutions. We operate in 147 '
            'countries, under 12 active regulatory frameworks, and '
            'above all applicable ethical guidelines (as self-assessed).',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textMuted,
              fontSize: 13,
              height: 1.75,
            ),
          ),
        ),

        const SizedBox(height: 20),

        // Certifications badges
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: const [
            _CertBadge(label: 'ISO 9001:2038'),
            _CertBadge(label: 'B-Corp™ Pending'),
            _CertBadge(label: 'Net Zero*'),
          ],
        ),
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// Link column
// ---------------------------------------------------------------------------

/// A footer link that either navigates to a GoRouter [route] or calls an
/// [action] callback (e.g. scroll-to-section).
class _FooterLink {
  final String label;
  final String? route; // GoRouter path (e.g. '/legal/grower-eula')
  final VoidCallback? action; // scroll-to-section or other callback

  const _FooterLink(this.label, {this.route}) : action = null;
  _FooterLink.callback(this.label, this.action) : route = null;
}

/// Legal links always navigate to their own pages via GoRouter.
const List<_FooterLink> _kLegalLinks = [
  _FooterLink('Grower EULA', route: '/legal/grower-eula'),
  _FooterLink('Privacy Policy', route: '/legal/privacy-policy'),
  _FooterLink('Data Ownership Notice', route: '/legal/data-ownership'),
  _FooterLink('Seed Patent Registry', route: '/legal/seed-patent'),
  _FooterLink('Field Audit Policy', route: '/legal/field-audit'),
  _FooterLink('IP Enforcement', route: '/legal/ip-enforcement'),
  _FooterLink('Terms of Germination', route: '/legal/terms-of-germination'),
];

/// A single footer column with a title and list of links.
class _LinkColumn extends StatelessWidget {
  final String title;
  final List<_FooterLink> links;

  const _LinkColumn({required this.title, required this.links});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textPrimary,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 2.5,
          ),
        ),
        const SizedBox(height: 6),
        Container(width: 24, height: 1, color: MonsatanColors.solarGold),
        const SizedBox(height: 20),
        ...links.map((link) => _FooterLinkItem(link: link)),
      ],
    );
  }
}

class _FooterLinkItem extends StatefulWidget {
  final _FooterLink link;
  const _FooterLinkItem({required this.link});

  @override
  State<_FooterLinkItem> createState() => _FooterLinkItemState();
}

class _FooterLinkItemState extends State<_FooterLinkItem> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => _hovered = true),
      onExit: (_) => setState(() => _hovered = false),
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () {
          if (widget.link.route != null) {
            context.go(widget.link.route!);
          } else {
            widget.link.action?.call();
          }
        },
        child: Padding(
          padding: const EdgeInsets.only(bottom: 10),
          child: AnimatedDefaultTextStyle(
            duration: const Duration(milliseconds: 200),
            style: GoogleFonts.dmSans(
              color: _hovered
                  ? MonsatanColors.accentGreen
                  : MonsatanColors.textMuted,
              fontSize: 13,
              height: 1.4,
            ),
            child: Text(widget.link.label),
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Newsletter column
// ---------------------------------------------------------------------------

/// Email newsletter sign-up widget.
///
/// Collecting emails is, of course, the least of what Monsatan collects.
class _NewsletterColumn extends StatefulWidget {
  @override
  State<_NewsletterColumn> createState() => _NewsletterColumnState();
}

class _NewsletterColumnState extends State<_NewsletterColumn> {
  final TextEditingController _emailController = TextEditingController();
  bool _submitted = false;

  @override
  void dispose() {
    _emailController.dispose();
    super.dispose();
  }

  void _submit() {
    if (_emailController.text.trim().isEmpty) return;
    setState(() => _submitted = true);
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'STAY INFORMED',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textPrimary,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 2.5,
          ),
        ),
        const SizedBox(height: 6),
        Container(width: 24, height: 1, color: MonsatanColors.solarGold),
        const SizedBox(height: 20),
        Text(
          'Subscribe to receive product updates, regulatory briefings, '
          'and quarterly field audit schedules.',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 13,
            height: 1.7,
          ),
        ),
        const SizedBox(height: 24),

        if (_submitted)
          _SuccessMessage()
        else
          _EmailForm(controller: _emailController, onSubmit: _submit),

        const SizedBox(height: 16),
        Text(
          '* By subscribing you consent to receiving commercial communications '
          'and confirm that your email, IP address, and approximate geolocation '
          'may be retained indefinitely under our Data Ownership Policy.',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 10,
            height: 1.6,
          ),
        ),
      ],
    );
  }
}

class _EmailForm extends StatelessWidget {
  final TextEditingController controller;
  final VoidCallback onSubmit;

  const _EmailForm({required this.controller, required this.onSubmit});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: TextField(
            controller: controller,
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textPrimary,
              fontSize: 14,
            ),
            decoration: InputDecoration(
              hintText: 'your@email.com',
              hintStyle: GoogleFonts.dmSans(
                color: MonsatanColors.textMuted,
                fontSize: 14,
              ),
              filled: true,
              fillColor: MonsatanColors.surfaceCard,
              contentPadding: const EdgeInsets.symmetric(
                horizontal: 16,
                vertical: 14,
              ),
              enabledBorder: OutlineInputBorder(
                borderSide: const BorderSide(
                  color: MonsatanColors.surfaceBorder,
                  width: 1,
                ),
                borderRadius: BorderRadius.circular(2),
              ),
              focusedBorder: OutlineInputBorder(
                borderSide: const BorderSide(
                  color: MonsatanColors.accentGreen,
                  width: 1,
                ),
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            onSubmitted: (_) => onSubmit(),
          ),
        ),
        const SizedBox(width: 8),
        ElevatedButton(
          onPressed: onSubmit,
          style: ElevatedButton.styleFrom(
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
          ),
          child: const Text('JOIN'),
        ),
      ],
    );
  }
}

class _SuccessMessage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        border: Border.all(color: MonsatanColors.accentGreen, width: 1),
        color: MonsatanColors.accentGreen.withValues(alpha: 0.08),
      ),
      child: Row(
        children: [
          const Icon(
            Icons.check_circle_outline,
            color: MonsatanColors.accentGreen,
            size: 18,
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Text(
              'You\'re subscribed. Your data is in safe hands — ours.',
              style: GoogleFonts.dmSans(
                color: MonsatanColors.accentGreen,
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
// Cert widget
// ---------------------------------------------------------------------------

/// A small bordered certification badge.
class _CertBadge extends StatelessWidget {
  final String label;
  const _CertBadge({required this.label});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        border: Border.all(color: MonsatanColors.surfaceBorder, width: 1),
      ),
      child: Text(
        label,
        style: GoogleFonts.dmSans(
          color: MonsatanColors.textMuted,
          fontSize: 10,
          letterSpacing: 1.5,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Bottom bar
// ---------------------------------------------------------------------------

/// The very bottom strip of the footer: copyright line and mini legal links.
class _BottomBar extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.mobile;

    return Container(
      decoration: const BoxDecoration(
        border: Border(
          top: BorderSide(color: MonsatanColors.surfaceBorder, width: 1),
        ),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 48, vertical: 20),
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 1200),
          child: isWide
              ? Row(
                  children: [
                    _copyrightText(),
                    const Spacer(),
                    _legalLinks(context),
                  ],
                )
              : Column(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    _copyrightText(),
                    const SizedBox(height: 12),
                    _legalLinks(context),
                  ],
                ),
        ),
      ),
    );
  }

  Widget _copyrightText() {
    return Text(
      '© 2031–2038 Monsatan Corp. All rights reserved. All life forms '
      'derived from our seed stock are the intellectual property of '
      'Monsatan Corp and its subsidiaries.',
      style: GoogleFonts.dmSans(
        color: MonsatanColors.textMuted,
        fontSize: 11,
        height: 1.6,
      ),
    );
  }

  Widget _legalLinks(BuildContext context) {
    // Each entry is (display label, GoRouter route).
    const miniLinks = [
      ('Privacy', '/legal/privacy-policy'),
      ('Cookies', '/legal/cookies'),
      ('Whistleblower Portal*', '/legal/whistleblower'),
    ];

    return Wrap(
      spacing: 16,
      children: miniLinks
          .map(
            ((String, String) entry) => GestureDetector(
              onTap: () => context.go(entry.$2),
              child: MouseRegion(
                cursor: SystemMouseCursors.click,
                child: Text(
                  entry.$1,
                  style: GoogleFonts.dmSans(
                    color: MonsatanColors.textMuted,
                    fontSize: 11,
                    letterSpacing: 0.5,
                  ),
                ),
              ),
            ),
          )
          .toList(),
    );
  }
}
