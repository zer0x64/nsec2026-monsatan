import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:google_fonts/google_fonts.dart';
import '../theme.dart';

/// Full-page corporate research programme document.
///
/// Reached via the "READ THE PROSPECTUS" button on the home page.
/// Styled as an official Monsatan Corp document with dark corporate
/// aesthetics and intentionally sinister fine print.
class ProspectusPage extends StatelessWidget {
  const ProspectusPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: MonsatanColors.darkBackground,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        // Back button returns to the home page via go_router.
        leading: IconButton(
          icon: const Icon(
            Icons.arrow_back,
            color: MonsatanColors.textSecondary,
          ),
          onPressed: () => context.go('/'),
        ),
        title: _Wordmark(),
      ),
      body: SingleChildScrollView(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 860),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 48),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _DocumentHeader(),
                  const SizedBox(height: 48),

                  // --- Section 01: Introduction ---
                  _ProspectusSection(
                    number: '01',
                    title: 'Introduction',
                    children: [
                      _bodyText(
                        'Monsatan Corp\u2019s Open Research Programme (ORP) invites qualified '
                        'external researchers, academics, and institutions to collaborate with '
                        'our in-house biotechnology teams on high-priority agricultural challenges. '
                        'We believe that great science happens at the intersection of curiosity '
                        'and contractual clarity.',
                      ),
                    ],
                  ),

                  // --- Section 02: Available Research Areas ---
                  _ProspectusSection(
                    number: '02',
                    title: 'Available Research Areas',
                    children: [
                      _BulletItem(
                        'Genome editing and trait insertion for yield optimisation',
                      ),
                      _BulletItem(
                        'Herbicide resistance mechanisms and next-generation MonsatanKill\u2122 formulations',
                      ),
                      _BulletItem(
                        'Soil microbiome suppression and managed biodiversity protocols',
                      ),
                      _BulletItem(
                        'Crop data telemetry, predictive modelling, and remote yield surveillance',
                      ),
                      _BulletItem(
                        'Consumer perception management and regulatory navigation strategies',
                      ),
                    ],
                  ),

                  // --- Section 03: Intellectual Property Terms ---
                  _ProspectusSection(
                    number: '03',
                    title: 'Intellectual Property Terms',
                    children: [
                      _bodyText(
                        'All research outputs, discoveries, inventions, datasets, models, and '
                        'associated intellectual property generated during, arising from, or '
                        'tangentially related to the collaboration are assigned exclusively and '
                        'irrevocably to Monsatan Corp and its subsidiaries.',
                      ),
                      const SizedBox(height: 16),
                      // Gold left-border pull-quote for the particularly egregious clause.
                      Container(
                        decoration: const BoxDecoration(
                          border: Border(
                            left: BorderSide(
                              color: MonsatanColors.solarGold,
                              width: 3,
                            ),
                          ),
                        ),
                        padding: const EdgeInsets.only(left: 20),
                        child: Text(
                          '\u201cCollaborators retain the right to be listed in the Acknowledgements '
                          'section of any resulting publications, subject to review by the Monsatan '
                          'Legal Department and payment of the applicable acknowledgement processing '
                          'fee (APF-2038: \u20ac400 per paper).\u201d',
                          style: GoogleFonts.playfairDisplay(
                            fontSize: 15,
                            fontStyle: FontStyle.italic,
                            color: MonsatanColors.solarGold,
                          ),
                        ),
                      ),
                    ],
                  ),

                  // --- Section 04: Eligibility ---
                  _ProspectusSection(
                    number: '04',
                    title: 'Eligibility',
                    children: [
                      _BulletItem(
                        'PhD required in a relevant field (as determined by Monsatan Corp)',
                      ),
                      _BulletItem(
                        'Institutional affiliation with a university holding at least two active '
                        'Monsatan licensing agreements',
                      ),
                      _BulletItem(
                        'No history of publication in journals critical of proprietary seed technology',
                      ),
                      _BulletItem(
                        'Willingness to sign the Monsatan Researcher Conduct & Confidentiality '
                        'Agreement (MRCCA-2038) prior to onboarding',
                      ),
                      _BulletItem(
                        'Applicants from the EU must provide proof of regulatory waiver acceptance',
                      ),
                    ],
                  ),

                  // --- Section 05: Application Process ---
                  _ProspectusSection(
                    number: '05',
                    title: 'Application Process',
                    children: [
                      _NumberedItem(
                        1,
                        'Submit your application and research proposal via the Monsatan Partner Portal',
                      ),
                      _NumberedItem(
                        2,
                        'Shortlisted candidates are contacted for a preliminary screening interview',
                      ),
                      _NumberedItem(
                        3,
                        'Successful screened candidates undergo a background check and institutional '
                        'credit assessment',
                      ),
                      _NumberedItem(
                        4,
                        'IP pre-assignment agreement signed and notarised',
                      ),
                      _NumberedItem(
                        5,
                        'Welcome call with your assigned Monsatan Research Liaison',
                      ),
                      _NumberedItem(
                        6,
                        'Onboarding: facility access, data governance briefing, and equipment '
                        'loaner agreement',
                      ),
                    ],
                  ),

                  // --- CTA row ---
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      ElevatedButton(
                        onPressed: () => context.go('/partner'),
                        child: const Text('APPLY NOW'),
                      ),
                      const SizedBox(width: 16),
                      Text(
                        'Questions? Contact research@monsatan.corp',
                        style: GoogleFonts.dmSans(
                          fontSize: 13,
                          color: MonsatanColors.textMuted,
                        ),
                      ),
                    ],
                  ),

                  const SizedBox(height: 40),

                  // --- Legal fine print ---
                  Text(
                    'Monsatan Corp reserves the right to modify the terms of this programme at any '
                    'time without notice. Previous versions of this prospectus are classified. If '
                    'you have obtained an earlier version, please destroy it and contact our Legal '
                    'Department.',
                    textAlign: TextAlign.center,
                    style: GoogleFonts.dmSans(
                      fontSize: 11,
                      color: MonsatanColors.textMuted,
                      height: 1.6,
                    ),
                  ),

                  const SizedBox(height: 48),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  /// Shared body paragraph style (DM Sans, 15px, comfortable line height).
  static Widget _bodyText(String text) {
    return Text(
      text,
      style: GoogleFonts.dmSans(
        fontSize: 15,
        height: 1.8,
        color: MonsatanColors.textSecondary,
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// AppBar wordmark
// ---------------------------------------------------------------------------

/// Compact Monsatan Corp wordmark used in the AppBar.
///
/// Renders a gold circle with the letter "M" followed by the company name.
class _Wordmark extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Gold circle monogram.
        Container(
          width: 30,
          height: 30,
          decoration: const BoxDecoration(
            color: MonsatanColors.solarGold,
            shape: BoxShape.circle,
          ),
          alignment: Alignment.center,
          child: Text(
            'M',
            style: GoogleFonts.playfairDisplay(
              fontSize: 15,
              fontWeight: FontWeight.w700,
              color: MonsatanColors.darkBackground,
            ),
          ),
        ),
        const SizedBox(width: 10),
        Text(
          'MONSATAN CORP',
          style: GoogleFonts.dmSans(
            fontSize: 13,
            fontWeight: FontWeight.w700,
            letterSpacing: 2.5,
            color: MonsatanColors.textPrimary,
          ),
        ),
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// Document header box
// ---------------------------------------------------------------------------

/// Top-of-page box that mimics an official document header.
///
/// Shows the document reference number, title, and metadata row.
class _DocumentHeader extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: MonsatanColors.surfaceCard,
        border: Border.all(color: MonsatanColors.surfaceBorder),
      ),
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Reference number label.
          Text(
            'DOCUMENT REF: ORP-2038-V4.2',
            style: GoogleFonts.dmSans(
              fontSize: 10,
              letterSpacing: 2,
              color: MonsatanColors.textMuted,
            ),
          ),
          const SizedBox(height: 8),

          // Document title.
          Text(
            'Open Research Programme\n2038 Prospectus',
            style: GoogleFonts.playfairDisplay(
              fontSize: 36,
              fontWeight: FontWeight.w700,
              color: MonsatanColors.textPrimary,
              height: 1.2,
            ),
          ),
          const SizedBox(height: 12),

          // Metadata row: classification | issued | status.
          Row(
            children: [
              _metaLabel('CLASSIFICATION: Public (Redacted)'),
              _divider(),
              _metaLabel('ISSUED: Q1 2038'),
              _divider(),
              _metaLabel('STATUS: Open'),
            ],
          ),
        ],
      ),
    );
  }

  /// Single metadata label in the header row.
  static Widget _metaLabel(String text) {
    return Text(
      text,
      style: GoogleFonts.dmSans(
        fontSize: 11,
        letterSpacing: 1.5,
        color: MonsatanColors.textMuted,
      ),
    );
  }

  /// Thin vertical divider between metadata labels.
  static Widget _divider() {
    return Container(
      width: 1,
      height: 12,
      color: MonsatanColors.surfaceBorder,
      margin: const EdgeInsets.symmetric(horizontal: 12),
    );
  }
}

// ---------------------------------------------------------------------------
// Section wrapper
// ---------------------------------------------------------------------------

/// A titled document section with a faint gold accent number.
///
/// [number] — zero-padded string, e.g. `'01'`
/// [title]  — section heading displayed in Playfair Display
/// [children] — body content widgets
class _ProspectusSection extends StatelessWidget {
  const _ProspectusSection({
    required this.number,
    required this.title,
    required this.children,
  });

  final String number;
  final String title;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Oversized faint number — decorative background accent.
        Text(
          number,
          style: GoogleFonts.playfairDisplay(
            fontSize: 48,
            color: MonsatanColors.surfaceBorder,
            height: 1,
          ),
        ),

        // Section heading.
        Text(
          title,
          style: GoogleFonts.playfairDisplay(
            fontSize: 22,
            fontWeight: FontWeight.w600,
            color: MonsatanColors.textPrimary,
          ),
        ),
        const SizedBox(height: 8),

        // Gold underline accent.
        Container(width: 40, height: 2, color: MonsatanColors.solarGold),
        const SizedBox(height: 20),

        // Section body content.
        ...children,

        const SizedBox(height: 32),
        const Divider(color: MonsatanColors.surfaceBorder),
        const SizedBox(height: 40),
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// List item helpers
// ---------------------------------------------------------------------------

/// A bullet-point list item with a gold dot marker.
class _BulletItem extends StatelessWidget {
  const _BulletItem(this.text);

  final String text;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Gold bullet dot, nudged down to align with the text cap height.
          Padding(
            padding: const EdgeInsets.only(top: 7, right: 12),
            child: Container(
              width: 5,
              height: 5,
              decoration: const BoxDecoration(
                color: MonsatanColors.solarGold,
                shape: BoxShape.circle,
              ),
            ),
          ),
          Expanded(
            child: Text(
              text,
              style: GoogleFonts.dmSans(
                fontSize: 15,
                height: 1.7,
                color: MonsatanColors.textSecondary,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

/// A numbered list item with a gold step number.
class _NumberedItem extends StatelessWidget {
  const _NumberedItem(this.step, this.text);

  final int step;
  final String text;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Step number in gold, fixed width keeps items aligned.
          SizedBox(
            width: 28,
            child: Text(
              '$step.',
              style: GoogleFonts.dmSans(
                fontSize: 15,
                fontWeight: FontWeight.w700,
                color: MonsatanColors.solarGold,
              ),
            ),
          ),
          Expanded(
            child: Text(
              text,
              style: GoogleFonts.dmSans(
                fontSize: 15,
                height: 1.7,
                color: MonsatanColors.textSecondary,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
