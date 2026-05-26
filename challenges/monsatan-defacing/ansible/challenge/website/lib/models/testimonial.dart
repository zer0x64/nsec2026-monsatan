/// A verified™ customer testimonial displayed in the testimonials carousel.
class Testimonial {
  final String quote;
  final String author;

  /// Job title or farming role of the testimonial author.
  final String role;

  /// Geographic region (may be ominously vague).
  final String region;

  const Testimonial({
    required this.quote,
    required this.author,
    required this.role,
    required this.region,
  });
}
