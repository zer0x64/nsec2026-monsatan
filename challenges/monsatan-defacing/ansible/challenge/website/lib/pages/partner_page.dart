import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:google_fonts/google_fonts.dart';
import '../theme.dart';

// ---------------------------------------------------------------------------
// PartnerPage
// ---------------------------------------------------------------------------

/// Full-page partner application view, reached via "PARTNER WITH US" /
/// "APPLY NOW" buttons on the home page.
///
/// Composed of four vertically-stacked sections:
///   1. Header block
///   2. Partnership type cards
///   3. Application form
///   4. Fine-print legal disclaimer
class PartnerPage extends StatelessWidget {
  const PartnerPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: MonsatanColors.darkBackground,
      appBar: AppBar(
        backgroundColor: MonsatanColors.darkBackground,
        elevation: 0,
        // Back button → home
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
            constraints: const BoxConstraints(maxWidth: 900),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 32),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: const [
                  SizedBox(height: 56),
                  _HeaderBlock(),
                  SizedBox(height: 64),
                  _PartnershipTypesSection(),
                  SizedBox(height: 64),
                  _ApplicationForm(),
                  SizedBox(height: 64),
                  _FinePrintDisclaimer(),
                  SizedBox(height: 80),
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
// AppBar wordmark (mirrors _MonsatanLogo in nav_bar.dart)
// ---------------------------------------------------------------------------

/// Monsatan Corp wordmark: gold circle monogram + two-tone logotype.
class _MonsatanWordmark extends StatelessWidget {
  const _MonsatanWordmark();

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Gold circle with "M"
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

// ---------------------------------------------------------------------------
// Section 1 — Header block
// ---------------------------------------------------------------------------

/// Eyebrow label + gold underline + heading + subtitle.
class _HeaderBlock extends StatelessWidget {
  const _HeaderBlock();

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Eyebrow label
        Text(
          'PARTNER PROGRAMME',
          style: GoogleFonts.dmSans(
            color: MonsatanColors.accentGreen,
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 3,
          ),
        ),
        const SizedBox(height: 12),
        // Gold accent rule
        Container(width: 40, height: 2, color: MonsatanColors.solarGold),
        const SizedBox(height: 20),
        // Main heading
        Text(
          'Join the Monsatan\nPartner Ecosystem',
          style: GoogleFonts.playfairDisplay(
            color: MonsatanColors.textPrimary,
            fontSize: 40,
            fontWeight: FontWeight.w700,
            height: 1.2,
          ),
        ),
        const SizedBox(height: 20),
        // Subtitle, width-capped for readability
        ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 640),
          child: Text(
            'Monsatan Corp is always looking for motivated, legally compliant, '
            'and ideologically flexible partners across farming, distribution, '
            'research, and data collection. All partnerships are subject to our '
            'standard exclusivity clauses, IP assignment framework, and '
            'indefinite audit rights.',
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 17,
              height: 1.75,
            ),
          ),
        ),
      ],
    );
  }
}

// ---------------------------------------------------------------------------
// Section 2 — Partnership type cards
// ---------------------------------------------------------------------------

/// Responsive row of three partnership-type cards.
///
/// Collapses from a side-by-side layout to a stacked layout on narrow screens.
class _PartnershipTypesSection extends StatelessWidget {
  const _PartnershipTypesSection();

  static const _cards = [
    _CardData(
      icon: Icons.agriculture,
      title: 'Seed Distribution',
      description:
          'Become a Licensed Distribution Partner and gain the right to sell '
          'Monsatan seeds under our brand, our pricing, and our terms. '
          'Minimum annual volume commitments apply.',
    ),
    _CardData(
      icon: Icons.science,
      title: 'Research Collaboration',
      description:
          'Apply to our Open Research Programme. All joint IP is exclusively '
          'owned by Monsatan Corp. Collaborators receive acknowledgement in '
          'any resulting publications, pending legal review.',
    ),
    _CardData(
      icon: Icons.handshake,
      title: 'Contract Farming',
      description:
          'Enrol as a Licensed Agricultural Partner™ and grow under our full '
          'technical guidance, mandatory seed purchasing, and yield reporting '
          'framework. Field audits are included at no extra charge.',
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 24,
      runSpacing: 24,
      children: _cards.map((data) => _PartnerTypeCard(data: data)).toList(),
    );
  }
}

/// Immutable data bundle for a partnership-type card.
class _CardData {
  final IconData icon;
  final String title;
  final String description;

  const _CardData({
    required this.icon,
    required this.title,
    required this.description,
  });
}

/// Individual partnership-type card with icon, title, and description.
class _PartnerTypeCard extends StatelessWidget {
  final _CardData data;

  const _PartnerTypeCard({required this.data});

  @override
  Widget build(BuildContext context) {
    // Let each card fill as much horizontal space as available, but no less
    // than [_minCardWidth]. On wide screens the three cards sit side-by-side.
    final screenWidth = MediaQuery.of(context).size.width;
    final isWide = screenWidth > Breakpoints.tablet;
    final cardWidth = isWide
        ? (900 - 64 - 48) / 3.0
        : double.infinity; // 64 = page padding, 48 = 2×spacing

    return Container(
      width: cardWidth,
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        color: MonsatanColors.surfaceCard,
        border: Border.all(color: MonsatanColors.surfaceBorder),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Icon(data.icon, color: MonsatanColors.accentGreen, size: 32),
          const SizedBox(height: 16),
          Text(
            data.title,
            style: GoogleFonts.playfairDisplay(
              color: MonsatanColors.textPrimary,
              fontSize: 20,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: 12),
          Text(
            data.description,
            style: GoogleFonts.dmSans(
              color: MonsatanColors.textSecondary,
              fontSize: 15,
              height: 1.65,
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Section 3 — Application form
// ---------------------------------------------------------------------------

/// Stateful application form with validation and a post-submit success state.
class _ApplicationForm extends StatefulWidget {
  const _ApplicationForm();

  @override
  State<_ApplicationForm> createState() => _ApplicationFormState();
}

class _ApplicationFormState extends State<_ApplicationForm> {
  final _formKey = GlobalKey<FormState>();

  // Individual text-editing controllers for each field.
  final _nameController = TextEditingController();
  final _orgController = TextEditingController();
  final _countryController = TextEditingController();
  final _typeController = TextEditingController();
  final _messageController = TextEditingController();

  /// Whether the form has been successfully submitted.
  bool _submitted = false;

  @override
  void dispose() {
    _nameController.dispose();
    _orgController.dispose();
    _countryController.dispose();
    _typeController.dispose();
    _messageController.dispose();
    super.dispose();
  }

  void _submit() {
    if (_formKey.currentState!.validate()) {
      setState(() => _submitted = true);
    }
  }

  @override
  Widget build(BuildContext context) {
    return _submitted ? const _SuccessMessage() : _buildForm(context);
  }

  Widget _buildForm(BuildContext context) {
    final isWide = MediaQuery.of(context).size.width > Breakpoints.mobile;

    return Form(
      key: _formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Section heading
          Text(
            'Apply Now',
            style: GoogleFonts.playfairDisplay(
              color: MonsatanColors.textPrimary,
              fontSize: 28,
              fontWeight: FontWeight.w700,
            ),
          ),
          const SizedBox(height: 32),

          // Two-column layout on wide screens, stacked on mobile.
          if (isWide)
            Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Expanded(
                  child: Column(
                    children: [
                      _FormField(
                        label: 'Full Name',
                        controller: _nameController,
                        validator: _requiredValidator,
                      ),
                      const SizedBox(height: 16),
                      _FormField(
                        label: 'Organisation / Institution',
                        controller: _orgController,
                        validator: _requiredValidator,
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 24),
                Expanded(
                  child: Column(
                    children: [
                      _FormField(
                        label: 'Country',
                        controller: _countryController,
                      ),
                      const SizedBox(height: 16),
                      _FormField(
                        label: 'Partnership Type',
                        controller: _typeController,
                        hint: 'e.g. Seed Distribution, Contract Farming…',
                      ),
                    ],
                  ),
                ),
              ],
            )
          else
            Column(
              children: [
                _FormField(
                  label: 'Full Name',
                  controller: _nameController,
                  validator: _requiredValidator,
                ),
                const SizedBox(height: 16),
                _FormField(
                  label: 'Organisation / Institution',
                  controller: _orgController,
                  validator: _requiredValidator,
                ),
                const SizedBox(height: 16),
                _FormField(label: 'Country', controller: _countryController),
                const SizedBox(height: 16),
                _FormField(
                  label: 'Partnership Type',
                  controller: _typeController,
                  hint: 'e.g. Seed Distribution, Contract Farming…',
                ),
              ],
            ),

          const SizedBox(height: 16),

          // Multiline message / research proposal field
          _FormField(
            label: 'Message / Research Proposal',
            controller: _messageController,
            minLines: 4,
            maxLines: 10,
          ),

          const SizedBox(height: 32),

          // Full-width submit button
          SizedBox(
            width: double.infinity,
            child: ElevatedButton(
              onPressed: _submit,
              child: const Text('SUBMIT APPLICATION'),
            ),
          ),
        ],
      ),
    );
  }

  /// Validator that rejects blank or whitespace-only input.
  String? _requiredValidator(String? value) {
    if (value == null || value.trim().isEmpty) {
      return 'This field is required.';
    }
    return null;
  }
}

/// A single labelled text field styled to match the Monsatan dark theme.
class _FormField extends StatelessWidget {
  final String label;
  final TextEditingController controller;
  final String? hint;
  final int minLines;
  final int? maxLines;
  final FormFieldValidator<String>? validator;

  const _FormField({
    required this.label,
    required this.controller,
    this.hint,
    this.minLines = 1,
    this.maxLines = 1,
    this.validator,
  });

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      controller: controller,
      minLines: minLines,
      maxLines: maxLines ?? minLines,
      validator: validator,
      style: GoogleFonts.dmSans(
        color: MonsatanColors.textPrimary,
        fontSize: 15,
      ),
      decoration: InputDecoration(
        labelText: label,
        hintText: hint,
        labelStyle: GoogleFonts.dmSans(
          color: MonsatanColors.textSecondary,
          fontSize: 14,
        ),
        hintStyle: GoogleFonts.dmSans(
          color: MonsatanColors.textMuted,
          fontSize: 14,
        ),
        filled: true,
        fillColor: MonsatanColors.surfaceBorder.withValues(alpha: 0.3),
        // Default border
        border: const OutlineInputBorder(
          borderSide: BorderSide(color: MonsatanColors.surfaceBorder),
          borderRadius: BorderRadius.zero,
        ),
        enabledBorder: const OutlineInputBorder(
          borderSide: BorderSide(color: MonsatanColors.surfaceBorder),
          borderRadius: BorderRadius.zero,
        ),
        // Focused border uses accent green for visual feedback
        focusedBorder: const OutlineInputBorder(
          borderSide: BorderSide(color: MonsatanColors.accentGreen, width: 1.5),
          borderRadius: BorderRadius.zero,
        ),
        errorBorder: const OutlineInputBorder(
          borderSide: BorderSide(color: Colors.redAccent),
          borderRadius: BorderRadius.zero,
        ),
        focusedErrorBorder: const OutlineInputBorder(
          borderSide: BorderSide(color: Colors.redAccent, width: 1.5),
          borderRadius: BorderRadius.zero,
        ),
      ),
    );
  }
}

/// Confirmation message shown after a successful form submission.
class _SuccessMessage extends StatelessWidget {
  const _SuccessMessage();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        children: [
          const Icon(
            Icons.check_circle_outline,
            color: MonsatanColors.accentGreen,
            size: 48,
          ),
          const SizedBox(height: 24),
          Text(
            'Application Received',
            style: GoogleFonts.playfairDisplay(
              color: MonsatanColors.textPrimary,
              fontSize: 24,
              fontWeight: FontWeight.w700,
            ),
          ),
          const SizedBox(height: 16),
          ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 560),
            child: Text(
              'A Monsatan Field Liaison will contact you within 60–90 business '
              'days. By submitting this form, you confirm that all information, '
              'ideas, and intellectual property contained herein are licensed to '
              'Monsatan Corp in perpetuity, royalty-free.',
              textAlign: TextAlign.center,
              style: GoogleFonts.dmSans(
                color: MonsatanColors.textSecondary,
                fontSize: 15,
                height: 1.7,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

// ---------------------------------------------------------------------------
// Section 4 — Fine-print disclaimer
// ---------------------------------------------------------------------------

/// Small-print legal notice at the foot of the page.
class _FinePrintDisclaimer extends StatelessWidget {
  const _FinePrintDisclaimer();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 700),
        child: Text(
          'Monsatan Corp reserves the right to reject any application without '
          'explanation. Submission of this form constitutes acceptance of our '
          'Partner Confidentiality Pre-Agreement (PCPA-2038). Monsatan Corp is '
          'an equal-opportunity IP acquirer.',
          textAlign: TextAlign.center,
          style: GoogleFonts.dmSans(
            color: MonsatanColors.textMuted,
            fontSize: 11,
            height: 1.7,
          ),
        ),
      ),
    );
  }
}
