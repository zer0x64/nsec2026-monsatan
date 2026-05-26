/// A Monsatan Corporation agricultural product displayed on the website.
class Product {
  final String name;
  final String tagline;
  final String description;

  /// Emoji icon used as a visual stand-in for product imagery.
  final String emoji;

  /// Bullet-point selling features (may contain fine-print horrors).
  final List<String> features;

  /// Legal disclaimer shown in small print below the product card.
  final String disclaimer;

  const Product({
    required this.name,
    required this.tagline,
    required this.description,
    required this.emoji,
    required this.features,
    required this.disclaimer,
  });
}
