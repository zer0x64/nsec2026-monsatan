import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:google_fonts/google_fonts.dart';
import '../theme.dart';

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// The content for a single legal document.
class _LegalDoc {
  final String title;
  final String effectiveDate;
  final List<_LegalSection> sections;

  const _LegalDoc({
    required this.title,
    required this.effectiveDate,
    required this.sections,
  });
}

/// A titled section within a [_LegalDoc], containing one or more paragraphs.
class _LegalSection {
  final String heading;
  final List<String> paragraphs;

  const _LegalSection({required this.heading, required this.paragraphs});
}

// ---------------------------------------------------------------------------
// Document content
// ---------------------------------------------------------------------------

/// All available legal documents, keyed by their URL slug.
const Map<String, _LegalDoc> _kLegalDocs = {
  'grower-eula': _LegalDoc(
    title: 'Grower End-User License Agreement',
    effectiveDate: '1 January 2038',
    sections: [
      _LegalSection(
        heading: '1. Grant of Licence',
        paragraphs: [
          'Monsatan Corp grants you a non-exclusive, non-transferable, revocable licence to germinate and cultivate seeds purchased directly from Monsatan Corp or an authorised distributor, subject to the terms below.',
          'This licence does not include the right to save, replant, share, gift, sell, or otherwise reproduce any seed or plant derived from Monsatan Corp seed stock. Unauthorised reproduction constitutes wilful patent infringement and will be pursued to the fullest extent of intergalactic law.',
        ],
      ),
      _LegalSection(
        heading: '2. Field Audit Rights',
        paragraphs: [
          'By planting Monsatan seeds you grant Monsatan Corp and its designated auditors irrevocable right of access to your land, crops, and associated agricultural records at any time, with or without prior notice.',
          'Refusal to permit audit access constitutes a material breach of this Agreement and triggers the Monsatan Automatic Penalty Clause (MAPC), resulting in immediate licence termination and a minimum fine of €50,000 per acre under cultivation.',
        ],
      ),
      _LegalSection(
        heading: '3. Termination',
        paragraphs: [
          'Monsatan Corp may terminate this licence at any time, for any reason, without notice. Upon termination, all plants derived from Monsatan seed stock must be destroyed within 72 hours. Failure to comply will be treated as continued infringement.',
        ],
      ),
      _LegalSection(
        heading: '4. Governing Law',
        paragraphs: [
          'This Agreement is governed by the laws of the jurisdiction most favourable to Monsatan Corp at the time of dispute, as determined by Monsatan Corp.',
        ],
      ),
    ],
  ),

  'privacy-policy': _LegalDoc(
    title: 'Privacy Policy',
    effectiveDate: '15 March 2038',
    sections: [
      _LegalSection(
        heading: 'What We Collect',
        paragraphs: [
          'We collect information you provide directly (name, email, location, crop yields, field coordinates, soil composition reports, genetic data from plants grown under licence, and any other information you share with us or that we are able to infer).',
          'We also collect information automatically when you interact with our website, including IP address, device identifiers, browsing behaviour, mouse movements, and approximate physical location.',
        ],
      ),
      _LegalSection(
        heading: 'How We Use Your Data',
        paragraphs: [
          'We use your data to improve our services, enforce our licence agreements, identify potential infringement, target advertising, sell aggregated datasets to third parties (including governments, insurers, and other agricultural conglomerates), and for any other purpose we deem commercially reasonable.',
        ],
      ),
      _LegalSection(
        heading: 'Data Retention',
        paragraphs: [
          'We retain your data indefinitely. Under our Data Ownership Policy (see separate notice), data collected in connection with any Monsatan product or service is the property of Monsatan Corp and is not subject to deletion requests.',
        ],
      ),
      _LegalSection(
        heading: 'Your Rights',
        paragraphs: [
          'Depending on your jurisdiction, you may have the right to request access to your personal data. We will respond to such requests within the legally required timeframe, which we have engaged three law firms to maximise.',
        ],
      ),
    ],
  ),

  'data-ownership': _LegalDoc(
    title: 'Data Ownership Notice',
    effectiveDate: '1 January 2038',
    sections: [
      _LegalSection(
        heading: 'Scope',
        paragraphs: [
          'This Notice applies to all data generated by, derived from, or associated with the use of any Monsatan Corp product, service, or licenced seed stock, whether generated by you, your equipment, your employees, or the plants themselves.',
        ],
      ),
      _LegalSection(
        heading: 'Ownership',
        paragraphs: [
          'All such data is the sole and exclusive property of Monsatan Corp. You are granted a limited, non-exclusive licence to view your own farm data through the Monsatan AgriPortal™, provided your licence is active and your account is in good standing.',
          'Exporting, sharing, publishing, or using your farm data outside the AgriPortal™ without written consent from Monsatan Corp constitutes a breach of your Grower EULA and this Notice.',
        ],
      ),
    ],
  ),

  'seed-patent': _LegalDoc(
    title: 'Seed Patent Registry',
    effectiveDate: 'Continuously updated',
    sections: [
      _LegalSection(
        heading: 'Overview',
        paragraphs: [
          'Monsatan Corp holds active patents on the genome, phenotype, germination mechanism, and commercial potential of the following seed varieties. This list is not exhaustive. If you are unsure whether a particular seed is covered by a Monsatan patent, assume that it is.',
        ],
      ),
      _LegalSection(
        heading: 'Currently Patented Varieties',
        paragraphs: [
          'RadiGrow™ Wheat (Patents 2031-WH-001 through 2031-WH-4,847), HerbiShield™ Corn (2032-CR-001 through 2034-CR-12,003), MegaYield™ Soy (2032-SY-001 through 2035-SY-8,291), SunSeed™ Sunflower, AquaRoot™ Rice, NightShade™ Tomato, and all genetic sequences derived therefrom, including but not limited to any sequences arising from cross-pollination with non-Monsatan varieties.',
          'Monsatan Corp additionally holds a blanket patent on the concept of herbicide-tolerant crop improvement (Patent MCC-2033-OMNI-1). Growers who believe their non-Monsatan crops have been inadvertently improved by proximity to Monsatan fields should contact our Licensing Department immediately.',
        ],
      ),
    ],
  ),

  'field-audit': _LegalDoc(
    title: 'Field Audit Policy',
    effectiveDate: '1 April 2037',
    sections: [
      _LegalSection(
        heading: 'Purpose',
        paragraphs: [
          'Field audits are conducted to ensure compliance with your Grower EULA, verify accurate yield reporting, identify potential seed infringement, and collect agronomic data for Monsatan Corp research purposes.',
        ],
      ),
      _LegalSection(
        heading: 'Frequency',
        paragraphs: [
          'Standard partners may expect 2–4 audits per growing season. Partners with open compliance queries, irregular yield reports, or Monsatan Corp interest in their land may be audited more frequently. There is no maximum audit frequency.',
        ],
      ),
      _LegalSection(
        heading: 'Audit Refusal',
        paragraphs: [
          'Refusal to permit an audit, or obstruction of an auditor, constitutes immediate material breach of your Grower EULA, triggers the MAPC penalty clause, and may result in referral to our Seed Enforcement Division (SED). The SED operates in 147 countries and has a 98.3% resolution rate (as self-reported).',
        ],
      ),
    ],
  ),

  'ip-enforcement': _LegalDoc(
    title: 'IP Enforcement Policy',
    effectiveDate: '1 January 2031 (original); last amended 1 March 2038',
    sections: [
      _LegalSection(
        heading: 'Zero-Tolerance Policy',
        paragraphs: [
          'Monsatan Corp operates a zero-tolerance policy on intellectual property infringement. Any unauthorised use, reproduction, or cultivation of Monsatan-patented genetic material will result in legal action, regardless of whether the infringement was intentional, accidental, or caused by wind.',
        ],
      ),
      _LegalSection(
        heading: 'Reporting Infringement',
        paragraphs: [
          'We encourage the agricultural community to report suspected infringement via our anonymous tip line. Confirmed reports are eligible for a Seed Enforcement Reward of up to €5,000. Reporting a neighbour\'s field does not affect your own audit schedule (much).',
        ],
      ),
      _LegalSection(
        heading: 'Penalties',
        paragraphs: [
          'Penalties for infringement range from mandatory licence purchase and back-payment of royalties (retroactive to planting date) to full crop forfeiture and land-use injunctions. Repeat infringement escalates to criminal referral under the 2035 Agricultural IP Protection Act.',
        ],
      ),
    ],
  ),

  'terms-of-germination': _LegalDoc(
    title: 'Terms of Germination',
    effectiveDate: '1 January 2038',
    sections: [
      _LegalSection(
        heading: 'Acceptance',
        paragraphs: [
          'By introducing any Monsatan Corp seed to soil, water, or any growth medium, you accept these Terms of Germination in full, without reservation. No signature is required. Germination constitutes consent.',
        ],
      ),
      _LegalSection(
        heading: 'The Seed Contract',
        paragraphs: [
          'Each Monsatan seed contains within its genome a digital-legal watermark constituting a binding micro-contract. Upon germination, the micro-contract activates and supersedes any prior understanding you may have had about seed ownership, plant rights, or natural law.',
        ],
      ),
      _LegalSection(
        heading: 'Liability',
        paragraphs: [
          'Monsatan Corp accepts no liability for crop failure, soil degradation, loss of biodiversity, health effects on humans or animals, regulatory penalties, or the gradual philosophical unease that may accompany growing food under these terms.',
        ],
      ),
    ],
  ),

  'cookies': _LegalDoc(
    title: 'Cookie Policy',
    effectiveDate: '1 June 2037',
    sections: [
      _LegalSection(
        heading: 'What Are Cookies?',
        paragraphs: [
          'Cookies are small data files placed on your device. We use them to enhance your experience, remember your preferences, and track everything you do on our site in granular detail.',
        ],
      ),
      _LegalSection(
        heading: 'What We Use',
        paragraphs: [
          'Essential cookies (cannot be disabled), analytics cookies (cannot be disabled), advertising cookies (cannot be disabled), and experimental biometric session tracking cookies (pilot programme, opt-out not yet available).',
        ],
      ),
      _LegalSection(
        heading: 'Your Choices',
        paragraphs: [
          'You may clear cookies from your browser at any time. We will immediately re-set them on your next visit. Continued use of this site constitutes acceptance of our cookie practices under Section 14(b) of your Grower EULA (even if you have never purchased seeds).',
        ],
      ),
    ],
  ),

  'whistleblower': _LegalDoc(
    title: 'Whistleblower Portal',
    effectiveDate: '1 January 2038',
    sections: [
      _LegalSection(
        heading: 'Our Commitment to Transparency',
        paragraphs: [
          'Monsatan Corp is fully committed to ethical business practices and welcomes reports of genuine misconduct through proper, monitored channels.',
        ],
      ),
      _LegalSection(
        heading: 'How to Submit a Report',
        paragraphs: [
          'All reports must be submitted in writing, in triplicate, using Form WB-2038-A (available from your local Monsatan Field Office). Reports may also be submitted via our secure online portal (currently undergoing maintenance — estimated restoration Q3 2039).',
        ],
      ),
      _LegalSection(
        heading: 'Confidentiality',
        paragraphs: [
          'Your identity will be kept strictly confidential, subject to our standard data retention and sharing practices (see Privacy Policy). Monsatan Corp cannot guarantee confidentiality in jurisdictions where disclosure is required by law, by our Legal Department, or by our internal audit process.',
          'By submitting a report, you acknowledge that Monsatan Corp may investigate the circumstances of the report, including your identity, your employment history, and your current Grower EULA compliance status.',
        ],
      ),
    ],
  ),
};

// ---------------------------------------------------------------------------
// Fine-print copy shown at the bottom of every document.
// ---------------------------------------------------------------------------

const String _kFinePrint =
    'This document is provided for informational purposes. Monsatan Corp '
    'reserves the right to amend these terms at any time. Continued existence '
    'on a planet where Monsatan operates constitutes acceptance.';

// ---------------------------------------------------------------------------
// Page widget
// ---------------------------------------------------------------------------

/// Displays a legal document identified by [doc] (a slug from the URL path
/// `/legal/:doc`). Unknown slugs render a "not found" message.
class LegalPage extends StatelessWidget {
  final String doc;

  const LegalPage({super.key, required this.doc});

  @override
  Widget build(BuildContext context) {
    final legalDoc = _kLegalDocs[doc];

    return Scaffold(
      backgroundColor: MonsatanColors.darkBackground,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        leading: IconButton(
          icon: const Icon(Icons.arrow_back, color: MonsatanColors.textPrimary),
          onPressed: () => context.go('/'),
          tooltip: 'Back to home',
        ),
        title: const _MonsatanWordmark(),
      ),
      body: SingleChildScrollView(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 860),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 48),
              child: legalDoc == null
                  ? _NotFoundBody(onBack: () => context.go('/'))
                  : _DocumentBody(legalDoc: legalDoc),
            ),
          ),
        ),
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// "Document not found" fallback
// ---------------------------------------------------------------------------

/// Shown when the [doc] slug does not match any entry in [_kLegalDocs].
class _NotFoundBody extends StatelessWidget {
  final VoidCallback onBack;

  const _NotFoundBody({required this.onBack});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            'Document not found.',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textPrimary,
              fontSize: 18,
            ),
          ),
          const SizedBox(height: 24),
          OutlinedButton(onPressed: onBack, child: const Text('BACK TO HOME')),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Document body
// ---------------------------------------------------------------------------

/// Renders the full content of a [_LegalDoc].
class _DocumentBody extends StatelessWidget {
  final _LegalDoc legalDoc;

  const _DocumentBody({required this.legalDoc});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Small category label
        Text(
          'LEGAL DOCUMENT',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.accentGreen,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 3,
          ),
        ),
        const SizedBox(height: 12),

        // Gold underline accent
        Container(width: 40, height: 2, color: MonsatanColors.solarGold),
        const SizedBox(height: 16),

        // Document title
        Text(
          legalDoc.title,
          style: GoogleFonts.playfairDisplay(
            color: MonsatanColors.textPrimary,
            fontSize: 36,
            fontWeight: FontWeight.w700,
          ),
        ),
        const SizedBox(height: 8),

        // Effective date
        Text(
          'Effective: ${legalDoc.effectiveDate}',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 12,
            letterSpacing: 1.5,
          ),
        ),
        const SizedBox(height: 8),

        // Horizontal rule
        Container(height: 1, color: MonsatanColors.surfaceBorder),
        const SizedBox(height: 40),

        // Sections
        ..._buildSections(legalDoc.sections),

        // Closing rule + fine print
        Container(height: 1, color: MonsatanColors.surfaceBorder),
        const SizedBox(height: 24),
        Text(
          _kFinePrint,
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 11,
            height: 1.8,
          ),
        ),
      ],
    );
  }

  /// Builds the list of section widgets, inserting spacing between them.
  List<Widget> _buildSections(List<_LegalSection> sections) {
    final widgets = <Widget>[];

    for (var i = 0; i < sections.length; i++) {
      widgets.add(_SectionBlock(section: sections[i]));
      if (i < sections.length - 1) {
        widgets.add(const SizedBox(height: 32));
      }
    }

    // A little extra breathing room before the closing fine print.
    widgets.add(const SizedBox(height: 40));

    return widgets;
  }
}

// ---------------------------------------------------------------------------
// Section block
// ---------------------------------------------------------------------------

/// Renders a single [_LegalSection]: an uppercase heading followed by its
/// paragraphs.
class _SectionBlock extends StatelessWidget {
  final _LegalSection section;

  const _SectionBlock({required this.section});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Section heading — small-caps style via letterSpacing + uppercase
        Text(
          section.heading.toUpperCase(),
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textPrimary,
            fontSize: 14,
            fontWeight: FontWeight.w700,
            letterSpacing: 1.5,
          ),
        ),
        const SizedBox(height: 12),

        // Body paragraphs
        for (final paragraph in section.paragraphs) ...[
          Text(
            paragraph,
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 15,
              height: 1.8,
            ),
          ),
          // Space between consecutive paragraphs within the same section.
          if (paragraph != section.paragraphs.last) const SizedBox(height: 12),
        ],
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// AppBar wordmark (mirrors partner_page.dart)
// ---------------------------------------------------------------------------

/// Monsatan Corp wordmark: gold circle monogram + two-tone logotype.
class _MonsatanWordmark extends StatelessWidget {
  const _MonsatanWordmark();

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Gold circle with "M" monogram
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
                fontSize: 20,
              ),
            ),
          ),
        ),
        const SizedBox(width: 10),

        // "MONSATAN" bold + "CORP" light, both letter-spaced
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
    );
  }
}
