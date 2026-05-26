import 'package:flutter/material.dart';

import 'package:google_fonts/google_fonts.dart';
import 'package:visibility_detector/visibility_detector.dart';
import '../theme.dart';

/// "About Monsatan" section: corporate mission statement, timeline of
/// "achievements", and a values grid — all written in the greenwashed
/// solarpunk voice that makes the horror feel perfectly reasonable.
class AboutSection extends StatelessWidget {
  const AboutSection({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      color: MonsatanColors.darkBackground,
      padding: const EdgeInsets.symmetric(vertical: 120),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 1100),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 32),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: const [
              _SectionLabel(label: 'ABOUT MONSATAN'),
              SizedBox(height: 24),
              _MissionStatement(),
              SizedBox(height: 100),
              _ValuesGrid(),
              SizedBox(height: 100),
              _Timeline(),
            ],
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Mission statement
// ---------------------------------------------------------------------------

class _MissionStatement extends StatefulWidget {
  const _MissionStatement();

  @override
  State<_MissionStatement> createState() => _MissionStatementState();
}

class _MissionStatementState extends State<_MissionStatement> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.tablet;

    return VisibilityDetector(
      key: const Key('about-mission'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.15 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: isWide
          ? Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Expanded(flex: 5, child: _headline(_visible)),
                const SizedBox(width: 64),
                Expanded(flex: 6, child: _body(_visible)),
              ],
            )
          : Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _headline(_visible),
                const SizedBox(height: 32),
                _body(_visible),
              ],
            ),
    );
  }

  Widget _headline(bool visible) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      opacity: visible ? 1 : 0,
      child: Text(
        'Feeding Humanity.\nProtecting the Planet.\nOwning Both.',
        style: GoogleFonts.playfairDisplay(
          color: MonsatanColors.textPrimary,
          fontSize: 36,
          fontWeight: FontWeight.w600,
          height: 1.3,
        ),
      ),
    );
  }

  Widget _body(bool visible) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      curve: Curves.easeOut,
      opacity: visible ? 1 : 0,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Founded in 2031 in the aftermath of the Great Dust Collapse, '
            'Monsatan Corp was built on a simple premise: nature had its '
            'chance. Now it\'s our turn.',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 18,
              height: 1.75,
            ),
          ),
          const SizedBox(height: 20),
          Text(
            'We believe the future of food is too important to be left to '
            'soil, rain, and the whims of an increasingly uncooperative '
            'climate. That\'s why we invest billions annually in '
            'biotechnological solutions that remove uncertainty from '
            'agriculture — and replace it with Monsatan.',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 18,
              height: 1.75,
            ),
          ),
          const SizedBox(height: 20),
          Text(
            'Our global network of Licensed Agricultural Partners™ '
            'spans 147 countries, ensuring that wherever food is grown, '
            'Monsatan is present — contractually, legally, and '
            'biologically.',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 18,
              height: 1.75,
            ),
          ),
          const SizedBox(height: 32),
          // Pull-quote
          Container(
            padding: const EdgeInsets.only(left: 20),
            decoration: const BoxDecoration(
              border: Border(
                left: BorderSide(color: MonsatanColors.solarGold, width: 3),
              ),
            ),
            child: Text(
              '"We don\'t just feed the world. We hold the copyright on '
              'feeding the world."',
              style: GoogleFonts.playfairDisplay(
                color: MonsatanColors.solarGold,
                fontSize: 18,
                fontStyle: FontStyle.italic,
                height: 1.6,
              ),
            ),
          ),
          const SizedBox(height: 12),
          Text(
            '— Dr. Helena Voss, CEO, Monsatan Corp',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textMuted,
              fontSize: 13,
              letterSpacing: 1.5,
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Company values grid
// ---------------------------------------------------------------------------

/// The four core "values" of Monsatan Corp.
const List<_ValueData> _kValues = [
  _ValueData(
    icon: Icons.eco,
    title: 'Sustainability',
    body:
        'We are committed to sustaining our revenue streams for generations '
        'to come. Our products are designed with long-term dependency in mind, '
        'ensuring that farmers, governments, and ecosystems remain aligned '
        'with our strategic roadmap indefinitely.',
  ),
  _ValueData(
    icon: Icons.biotech,
    title: 'Innovation',
    body:
        'Every Monsatan product begins in our BioOptimisation Labs, where '
        'teams of scientists ask the same bold question every morning: '
        '"What would nature do here, and how can we patent it?"',
  ),
  _ValueData(
    icon: Icons.handshake,
    title: 'Partnership',
    body:
        'Our farmer partnerships are contractually guaranteed to last a '
        'minimum of 20 years. We consider ourselves family — and like all '
        'families, we take a small percentage of everything you produce.',
  ),
  _ValueData(
    icon: Icons.balance,
    title: 'Transparency',
    body:
        'Monsatan publishes full annual reports detailing our Scope 1 '
        'emissions, charitable donations, and approved biodiversity metrics. '
        'All other data is proprietary. Requests should be directed to '
        'legal@monsatan.corp.',
  ),
];

class _ValueData {
  final IconData icon;
  final String title;
  final String body;
  const _ValueData({
    required this.icon,
    required this.title,
    required this.body,
  });
}

class _ValuesGrid extends StatefulWidget {
  const _ValuesGrid();

  @override
  State<_ValuesGrid> createState() => _ValuesGridState();
}

class _ValuesGridState extends State<_ValuesGrid> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    return VisibilityDetector(
      key: const Key('about-values'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.1 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const _SectionLabel(label: 'OUR VALUES'),
          const SizedBox(height: 40),
          LayoutBuilder(
            builder: (context, constraints) {
              final crossAxis = constraints.maxWidth > Breakpoints.mobile
                  ? 2
                  : 1;
              return GridView.builder(
                shrinkWrap: true,
                physics: const NeverScrollableScrollPhysics(),
                gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                  crossAxisCount: crossAxis,
                  crossAxisSpacing: 24,
                  mainAxisSpacing: 24,
                  childAspectRatio: constraints.maxWidth > Breakpoints.tablet
                      ? 1.6
                      : 1.2,
                ),
                itemCount: _kValues.length,
                itemBuilder: (context, index) {
                  return _ValueCard(
                    data: _kValues[index],
                    visible: _visible,
                    delay: Duration(milliseconds: 150 * index),
                  );
                },
              );
            },
          ),
        ],
      ),
    );
  }
}

/// A single value card with an icon, title, and description.
class _ValueCard extends StatefulWidget {
  final _ValueData data;
  final bool visible;
  final Duration delay;

  const _ValueCard({
    required this.data,
    required this.visible,
    required this.delay,
  });

  @override
  State<_ValueCard> createState() => _ValueCardState();
}

class _ValueCardState extends State<_ValueCard> {
  bool _hovered = false;

  @override
  Widget build(BuildContext context) {
    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      opacity: widget.visible ? 1 : 0,
      child: MouseRegion(
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 250),
          padding: const EdgeInsets.all(32),
          decoration: BoxDecoration(
            color: _hovered
                ? MonsatanColors.surfaceCard
                : MonsatanColors.surfaceCard.withValues(alpha: 0.6),
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
              Icon(widget.data.icon, color: MonsatanColors.solarGold, size: 28),
              const SizedBox(height: 16),
              Text(
                widget.data.title.toUpperCase(),
                style: GoogleFonts.dmSans(
                  color: MonsatanColors.textPrimary,
                  fontSize: 14,
                  fontWeight: FontWeight.w700,
                  letterSpacing: 2.5,
                ),
              ),
              const SizedBox(height: 12),
              Expanded(
                child: Text(
                  widget.data.body,
                  style: GoogleFonts.dmSans(
                    color: MonsatanColors.textSecondary,
                    fontSize: 14,
                    height: 1.7,
                  ),
                  overflow: TextOverflow.fade,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Company timeline
// ---------------------------------------------------------------------------

const List<_TimelineEvent> _kTimeline = [
  _TimelineEvent(
    year: '2031',
    title: 'Founding',
    body:
        'Monsatan Corp incorporated in the aftermath of the Great Dust '
        'Collapse. Patent portfolio acquired from three dissolved national '
        'seed banks "for safekeeping."',
  ),
  _TimelineEvent(
    year: '2033',
    title: 'First Global Contract',
    body:
        'Signed the Nairobi Agricultural Continuity Agreement, granting '
        'exclusive seeding rights to 600 million acres across Sub-Saharan '
        'Africa. Hailed by press as "the deal that saved a continent."',
  ),
  _TimelineEvent(
    year: '2034',
    title: 'MonsatanKill™ Launch',
    body:
        'Released our flagship total-spectrum herbicide. Independently '
        'reviewed as "effective." All reviewers are on retainer.',
  ),
  _TimelineEvent(
    year: '2036',
    title: 'Monsatan BioOptimisation Campus',
    body:
        'Opened our 40-hectare research campus, entirely solar powered '
        '(Scope 1). Voted Most Beautiful Corporate Campus by Corporate '
        'Campus Magazine (a Monsatan publication).',
  ),
  _TimelineEvent(
    year: '2037',
    title: '94% Market Share (Corn)',
    body:
        'Achieved near-total corn market dominance. The remaining 6% is '
        'under review. We remain "committed to a competitive marketplace."',
  ),
  _TimelineEvent(
    year: '2038',
    title: 'Today',
    body:
        'Monsatan continues its mission to nourish, optimise, and '
        'contractually secure the global food supply. The future is '
        'bright — and it belongs to us.',
  ),
];

class _TimelineEvent {
  final String year;
  final String title;
  final String body;
  const _TimelineEvent({
    required this.year,
    required this.title,
    required this.body,
  });
}

class _Timeline extends StatefulWidget {
  const _Timeline();

  @override
  State<_Timeline> createState() => _TimelineState();
}

class _TimelineState extends State<_Timeline> {
  bool _visible = false;

  @override
  Widget build(BuildContext context) {
    return VisibilityDetector(
      key: const Key('about-timeline'),
      onVisibilityChanged: (info) {
        if (info.visibleFraction > 0.05 && !_visible) {
          setState(() => _visible = true);
        }
      },
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const _SectionLabel(label: 'OUR JOURNEY'),
          const SizedBox(height: 48),
          ..._kTimeline.asMap().entries.map((entry) {
            final index = entry.key;
            final event = entry.value;
            final isLast = index == _kTimeline.length - 1;
            return _TimelineRow(
              event: event,
              isLast: isLast,
              visible: _visible,
              delay: Duration(milliseconds: 100 * index),
            );
          }),
        ],
      ),
    );
  }
}

/// A single row in the timeline: vertical connector, year, title + body.
class _TimelineRow extends StatelessWidget {
  final _TimelineEvent event;
  final bool isLast;
  final bool visible;
  final Duration delay;

  const _TimelineRow({
    required this.event,
    required this.isLast,
    required this.visible,
    required this.delay,
  });

  @override
  Widget build(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.mobile;

    return AnimatedOpacity(
      duration: const Duration(milliseconds: 400),
      opacity: visible ? 1 : 0,
      child: IntrinsicHeight(
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Left column: year + vertical line connector
            SizedBox(
              width: isWide ? 120 : 70,
              child: Column(
                children: [
                  Text(
                    event.year,
                    style: GoogleFonts.dmSans(
                      color: MonsatanColors.solarGold,
                      fontSize: isWide ? 16 : 12,
                      fontWeight: FontWeight.w700,
                      letterSpacing: 1.5,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Expanded(
                    child: isLast
                        ? const SizedBox()
                        : Center(
                            child: Container(
                              width: 1,
                              color: MonsatanColors.surfaceBorder,
                            ),
                          ),
                  ),
                ],
              ),
            ),
            // Dot connector
            Column(
              children: [
                Container(
                  width: 10,
                  height: 10,
                  margin: const EdgeInsets.only(top: 4),
                  decoration: const BoxDecoration(
                    color: MonsatanColors.accentGreen,
                    shape: BoxShape.circle,
                  ),
                ),
              ],
            ),
            const SizedBox(width: 24),
            // Right column: title + body
            Expanded(
              child: Padding(
                padding: const EdgeInsets.only(bottom: 40),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      event.title,
                      style: GoogleFonts.dmSans(
                        color: MonsatanColors.textPrimary,
                        fontSize: 16,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      event.body,
                      style: GoogleFonts.dmSans(
                        color: MonsatanColors.textSecondary,
                        fontSize: 15,
                        height: 1.7,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Small all-caps section label with a gold underline accent.
class _SectionLabel extends StatelessWidget {
  final String label;
  const _SectionLabel({required this.label});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: GoogleFonts.dmSans(
            color: MonsatanColors.accentGreen,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 3,
          ),
        ),
        const SizedBox(height: 8),
        Container(width: 40, height: 2, color: MonsatanColors.solarGold),
      ],
    );
  }
}
