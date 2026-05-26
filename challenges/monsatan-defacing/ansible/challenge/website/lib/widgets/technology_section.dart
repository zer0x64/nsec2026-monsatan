import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:visibility_detector/visibility_detector.dart';
import '../theme.dart';

/// Technology section showcasing Monsatan Corp's four research pillars.
///
/// Each pillar is presented with breathless corporate enthusiasm while
/// quietly describing practices that range from troubling to catastrophic.
class TechnologySection extends StatelessWidget {
  const TechnologySection({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: MonsatanColors.darkBackground,
      padding: const EdgeInsets.symmetric(vertical: 120),
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 1200),
          child: const Padding(
            padding: EdgeInsets.symmetric(horizontal: 32),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _SectionHeader(),
                SizedBox(height: 80),
                _PillarsGrid(),
                SizedBox(height: 100),
                _ResearchBanner(),
              ],
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

class _SectionHeader extends StatefulWidget {
  const _SectionHeader();

  @override
  State<_SectionHeader> createState() => _SectionHeaderState();
}

class _SectionHeaderState extends State<_SectionHeader> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    return VisibilityDetector(
      key: const Key('technology-header'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.2 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: AnimatedOpacity(
        duration: const Duration(milliseconds: 400),
        opacity: _visible ? 1 : 0,
        child: isWide
            ? Row(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  _titleBlock(),
                  const SizedBox(width: 80),
                  Expanded(child: _subtitleBlock()),
                ],
              )
            : Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _titleBlock(),
                  const SizedBox(height: 24),
                  _subtitleBlock(),
                ],
              ),
      ),
    );
  }

  Widget _titleBlock() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'TECHNOLOGY',
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
          'Science Without\nLimits. Or Oversight.',
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
    return Text(
      'Our researchers operate at the bleeding edge of biotechnology — '
      'a place where regulatory frameworks have not yet caught up with '
      'our vision. We consider this a competitive advantage. Each of our '
      'four research pillars represents a commitment to progress, profit, '
      'and the occasional peer-reviewed paper written entirely by Monsatan employees.',
      style: GoogleFonts.dmSans(
        color: MonsatanColors.textSecondary,
        fontSize: 15,
        height: 1.8,
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Research pillars data
// ---------------------------------------------------------------------------

/// A single Monsatan research pillar displayed as a feature card.
class _PillarData {
  final String number;
  final IconData icon;
  final String title;
  final String subtitle;
  final String body;
  final List<String> specs;

  const _PillarData({
    required this.number,
    required this.icon,
    required this.title,
    required this.subtitle,
    required this.body,
    required this.specs,
  });
}

const List<_PillarData> _kPillars = [
  _PillarData(
    number: '01',
    icon: Icons.biotech,
    title: 'Quantum Gene Expression',
    subtitle: 'QGE Platform',
    body:
        'Our QGE platform manipulates gene expression at the quantum level, '
        'allowing traits to be activated, suppressed, or licensed remotely '
        'via our cloud infrastructure. A firmware update to your crops is '
        'just one subscription renewal away.',
    specs: [
      'Remote trait toggle via MonsatanCloud™',
      'Over-the-air genome patch support',
      'Subscription-based trait unlocks (annual)',
      'Trait revocation on contract breach',
    ],
  ),
  _PillarData(
    number: '02',
    icon: Icons.wb_sunny,
    title: 'Solar Synergy™',
    subtitle: 'Dual-Photon Harvesting',
    body:
        'By embedding micro-photovoltaic nano-fibres directly into leaf '
        'tissue, Solar Synergy™ crops generate supplemental electrical '
        'current that accelerates cellular metabolism. Field data — '
        'including yield, location, and soil composition — is transmitted '
        'continuously to our servers as a standard operating condition.',
    specs: [
      '28% faster cellular metabolism',
      'Continuous telemetry (non-optional)',
      'Data sold to agri-commodity futures desks',
      'Compatible with SunSeed™ product line',
    ],
  ),
  _PillarData(
    number: '03',
    icon: Icons.water_drop,
    title: 'HydroCapture™',
    subtitle: 'Atmospheric Moisture Extraction',
    body:
        'HydroCapture™ root systems extend the concept of "plant water '
        'uptake" to its logical conclusion: every available molecule '
        'within range. Our root architecture reaches depths and lateral '
        'spreads previously associated only with geological formations.',
    specs: [
      '50 m lateral water extraction radius',
      'Deep aquifer tap — up to 200 m depth',
      'Neighbouring crop interference: "expected"',
      'Liability transferred to soil under §9(c)',
    ],
  ),
  _PillarData(
    number: '04',
    icon: Icons.science,
    title: 'BioFlux™ Irradiation',
    subtitle: 'Controlled Mutagenesis at Scale',
    body:
        'Rather than waiting millennia for beneficial mutations to arise '
        'naturally, BioFlux™ accelerates the process through targeted '
        'radiation exposure. We don\'t cherry-pick mutations — we generate '
        'thousands, discard the fatal ones, and trademark the rest.',
    specs: [
      '10,000+ mutation variants per season',
      'Automated lethality screening (97.3% accuracy)',
      'Surviving variants auto-patented',
      'Worker exposure within "tolerated" EU limits',
    ],
  ),
];

// ---------------------------------------------------------------------------
// Pillars grid
// ---------------------------------------------------------------------------

class _PillarsGrid extends StatefulWidget {
  const _PillarsGrid();

  @override
  State<_PillarsGrid> createState() => _PillarsGridState();
}

class _PillarsGridState extends State<_PillarsGrid> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final columns = width > Breakpoints.tablet ? 2 : 1;

    return VisibilityDetector(
      key: const Key('technology-pillars'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.05 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: GridView.builder(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: columns,
          crossAxisSpacing: 24,
          mainAxisSpacing: 24,
          childAspectRatio: columns == 1 ? 1.1 : 1.05,
        ),
        itemCount: _kPillars.length,
        itemBuilder: (context, index) {
          return _PillarCard(
            data: _kPillars[index],
            visible: _visible,
            delay: Duration(milliseconds: 120 * index),
          );
        },
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Pillar card
// ---------------------------------------------------------------------------

class _PillarCard extends StatefulWidget {
  final _PillarData data;
  final bool visible;
  final Duration delay;

  const _PillarCard({
    required this.data,
    required this.visible,
    required this.delay,
  });

  @override
  State<_PillarCard> createState() => _PillarCardState();
}

class _PillarCardState extends State<_PillarCard> {
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
          padding: const EdgeInsets.all(36),
          decoration: BoxDecoration(
            color: _hovered
                ? MonsatanColors.surfaceCard
                : MonsatanColors.surfaceCard.withValues(alpha: 0.5),
            border: Border.all(
              color: _hovered
                  ? MonsatanColors.solarGold.withValues(alpha: 0.5)
                  : MonsatanColors.surfaceBorder,
              width: 1,
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Card top row: pillar number + icon
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(
                    widget.data.number,
                    style: GoogleFonts.playfairDisplay(
                      color: MonsatanColors.surfaceBorder,
                      fontSize: 48,
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                  AnimatedContainer(
                    duration: const Duration(milliseconds: 250),
                    width: 48,
                    height: 48,
                    decoration: BoxDecoration(
                      color: _hovered
                          ? MonsatanColors.solarGold.withValues(alpha: 0.15)
                          : MonsatanColors.primaryGreen.withValues(alpha: 0.15),
                      shape: BoxShape.circle,
                    ),
                    child: Icon(
                      widget.data.icon,
                      color: _hovered
                          ? MonsatanColors.solarGold
                          : MonsatanColors.accentGreen,
                      size: 22,
                    ),
                  ),
                ],
              ),

              const SizedBox(height: 20),

              // Subtitle / platform name
              Text(
                widget.data.subtitle.toUpperCase(),
                style: GoogleFonts.dmSans(
                  color: MonsatanColors.accentGreen,
                  fontSize: 11,
                  fontWeight: FontWeight.w700,
                  letterSpacing: 2.5,
                ),
              ),
              const SizedBox(height: 8),

              // Pillar title
              Text(
                widget.data.title,
                style: GoogleFonts.playfairDisplay(
                  color: MonsatanColors.textPrimary,
                  fontSize: 22,
                  fontWeight: FontWeight.w600,
                  height: 1.3,
                ),
              ),
              const SizedBox(height: 16),

              // Description paragraph
              Text(
                widget.data.body,
                style: GoogleFonts.dmSans(
                  color: MonsatanColors.textSecondary,
                  fontSize: 14,
                  height: 1.75,
                ),
              ),

              const SizedBox(height: 24),

              // Thin divider
              Container(height: 1, color: MonsatanColors.surfaceBorder),
              const SizedBox(height: 20),

              // Technical specifications
              Text(
                'TECHNICAL SPECS',
                style: GoogleFonts.dmSans(
                  color: MonsatanColors.textMuted,
                  fontSize: 10,
                  letterSpacing: 2.5,
                  fontWeight: FontWeight.w700,
                ),
              ),
              const SizedBox(height: 12),
              ...widget.data.specs.map((spec) => _SpecRow(text: spec)),
            ],
          ),
        ),
      ),
    );
  }
}

/// A single specification line with a coloured dash prefix.
class _SpecRow extends StatelessWidget {
  final String text;
  const _SpecRow({required this.text});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.only(top: 2, right: 10),
            child: Text(
              '—',
              style: GoogleFonts.dmSans(
                color: MonsatanColors.solarGold,
                fontSize: 12,
                fontWeight: FontWeight.w700,
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
// Research banner
// ---------------------------------------------------------------------------

/// Full-width call-to-action banner at the bottom of the technology section.
class _ResearchBanner extends StatefulWidget {
  const _ResearchBanner();

  @override
  State<_ResearchBanner> createState() => _ResearchBannerState();
}

class _ResearchBannerState extends State<_ResearchBanner> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    return VisibilityDetector(
      key: const Key('technology-banner'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.2 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: AnimatedOpacity(
        duration: const Duration(milliseconds: 700),
        opacity: _visible ? 1 : 0,
        child: AnimatedSlide(
          duration: const Duration(milliseconds: 600),
          offset: _visible ? Offset.zero : const Offset(0, 0.15),
          curve: Curves.easeOut,
          child: Container(
            padding: EdgeInsets.all(isWide ? 56 : 32),
            decoration: BoxDecoration(
              // Diagonal gradient from dark green to almost-black
              gradient: const LinearGradient(
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
                colors: [Color(0xFF1A3828), MonsatanColors.darkBackground],
              ),
              border: Border.all(
                color: MonsatanColors.primaryGreen.withValues(alpha: 0.5),
                width: 1,
              ),
            ),
            child: isWide
                ? Row(
                    children: [
                      Expanded(child: _bannerText()),
                      const SizedBox(width: 48),
                      _bannerCta(),
                    ],
                  )
                : Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      _bannerText(),
                      const SizedBox(height: 32),
                      _bannerCta(),
                    ],
                  ),
          ),
        ),
      ),
    );
  }

  Widget _bannerText() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          'OPEN RESEARCH PROGRAMME',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.accentGreen,
            fontSize: 11,
            fontWeight: FontWeight.w700,
            letterSpacing: 3,
          ),
        ),
        const SizedBox(height: 14),
        Text(
          'Apply to collaborate with our scientists.',
          style: GoogleFonts.playfairDisplay(
            color: MonsatanColors.textPrimary,
            fontSize: 28,
            fontWeight: FontWeight.w600,
            height: 1.3,
          ),
        ),
        const SizedBox(height: 14),
        Text(
          'We welcome external researchers, academics, and institutions '
          'who share our commitment to progress. All joint IP will be '
          'owned exclusively by Monsatan Corp. Collaborators retain the '
          'right to attend the announcement press conference.',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textSecondary,
            fontSize: 15,
            height: 1.7,
          ),
        ),
      ],
    );
  }

  Widget _bannerCta() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        ElevatedButton(
          onPressed: () => context.go('/partner'),
          child: const Text('APPLY NOW'),
        ),
        const SizedBox(height: 16),
        OutlinedButton(
          onPressed: () => context.go('/prospectus'),
          child: const Text('READ THE PROSPECTUS'),
        ),
        const SizedBox(height: 16),
        Text(
          '* Application does not guarantee collaboration.\n'
          'All applicants subject to background screening.',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 11,
            height: 1.6,
          ),
        ),
      ],
    );
  }
}
