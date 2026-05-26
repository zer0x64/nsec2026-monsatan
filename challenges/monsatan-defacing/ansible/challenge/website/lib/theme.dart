import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

/// Responsive layout breakpoints in logical pixels.
class Breakpoints {
  static const double mobile = 600;
  static const double tablet = 1024;
}

/// Monsatan Corporation brand colour palette.
///
/// Dark-green corporate base with solar-gold accents — a greenwashed
/// solarpunk aesthetic that hides the sinister in plain sight.
class MonsatanColors {
  // Backgrounds
  static const Color darkBackground = Color(0xFF080F0A);
  static const Color surfaceCard = Color(0xFF0F1C12);
  static const Color surfaceBorder = Color(0xFF1E3A27);

  // Greens
  static const Color primaryGreen = Color(0xFF2D6A4F);
  static const Color accentGreen = Color(0xFF52B788);

  // Accents
  static const Color solarGold = Color(0xFFF2CC0C);

  // Text
  static const Color textPrimary = Color(0xFFF0EDE4);
  static const Color textSecondary = Color(0xFF8FB99A);
  static const Color textMuted = Color(0xFF4A7A5A);
}

/// App-wide Material theme for the Monsatan website.
class MonsatanTheme {
  static ThemeData get theme {
    final base = ThemeData.dark();

    return base.copyWith(
      colorScheme: const ColorScheme.dark(
        primary: MonsatanColors.primaryGreen,
        secondary: MonsatanColors.solarGold,
        surface: MonsatanColors.surfaceCard,
        onPrimary: MonsatanColors.textPrimary,
        onSecondary: MonsatanColors.darkBackground,
        onSurface: MonsatanColors.textPrimary,
      ),
      scaffoldBackgroundColor: MonsatanColors.darkBackground,

      // Apply DM Sans as the default body font.
      textTheme: GoogleFonts.dmSansTextTheme(base.textTheme).copyWith(
        // Playfair Display for large display headings.
        displayLarge: GoogleFonts.playfairDisplay(
          color: MonsatanColors.textPrimary,
          fontSize: 68,
          fontWeight: FontWeight.w700,
          height: 1.1,
        ),
        displayMedium: GoogleFonts.playfairDisplay(
          color: MonsatanColors.textPrimary,
          fontSize: 48,
          fontWeight: FontWeight.w600,
          height: 1.2,
        ),
        headlineLarge: GoogleFonts.playfairDisplay(
          color: MonsatanColors.textPrimary,
          fontSize: 36,
          fontWeight: FontWeight.w600,
          height: 1.25,
        ),
        headlineMedium: GoogleFonts.dmSans(
          color: MonsatanColors.textPrimary,
          fontSize: 24,
          fontWeight: FontWeight.w600,
        ),
        bodyLarge: GoogleFonts.dmSans(
          color: MonsatanColors.textSecondary,
          fontSize: 18,
          height: 1.75,
        ),
        bodyMedium: GoogleFonts.dmSans(
          color: MonsatanColors.textSecondary,
          fontSize: 16,
          height: 1.65,
        ),
        labelLarge: GoogleFonts.dmSans(
          color: MonsatanColors.textPrimary,
          fontSize: 13,
          fontWeight: FontWeight.w700,
          letterSpacing: 2.0,
        ),
      ),

      appBarTheme: const AppBarTheme(
        backgroundColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
      ),

      // Primary CTA buttons: solar-gold fill with dark text.
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: MonsatanColors.solarGold,
          foregroundColor: MonsatanColors.darkBackground,
          padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 18),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(2)),
          textStyle: GoogleFonts.dmSans(
            fontSize: 13,
            fontWeight: FontWeight.w700,
            letterSpacing: 2.0,
          ),
        ),
      ),

      // Secondary buttons: outlined in accent green.
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: MonsatanColors.textPrimary,
          side: const BorderSide(color: MonsatanColors.accentGreen, width: 1),
          padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 18),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(2)),
          textStyle: GoogleFonts.dmSans(
            fontSize: 13,
            fontWeight: FontWeight.w700,
            letterSpacing: 2.0,
          ),
        ),
      ),

      dividerColor: MonsatanColors.surfaceBorder,
    );
  }
}
