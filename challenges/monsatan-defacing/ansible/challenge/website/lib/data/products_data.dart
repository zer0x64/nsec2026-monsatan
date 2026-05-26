import '../models/product.dart';

/// The official Monsatan Corporation product lineup.
///
/// Each product is a satirical riff on real-world GMO / agrochemical practices,
/// dressed up in solarpunk-flavoured corporate marketing language.
const List<Product> kProducts = [
  Product(
    name: 'RadiGrow™ Wheat',
    tagline: 'Powered by the Atom',
    description:
        'Our breakthrough radiation-enhanced wheat strain delivers yields '
        '300% above industry standards. Each kernel is carefully optimised '
        'using our proprietary BioFlux™ irradiation process for guaranteed '
        'uniformity and unmatched shelf resilience.',
    emoji: '☢️',
    features: [
      'Triple-yield certified (self-certified)',
      'Naturally resistant to all herbicides, including ours',
      'Indefinite shelf life — refrigeration optional',
      'Soft bioluminescence enables night harvesting',
    ],
    disclaimer:
        '* Not recommended for pregnant individuals, children under 18, '
        'or anyone with functioning kidneys. Glow may persist post-digestion.',
  ),
  Product(
    name: 'HerbiShield™ Corn',
    tagline: 'Immune to Nature Itself',
    description:
        'HerbiShield™ Corn stands tall where others wither. Engineered to '
        'survive our exclusive MonsatanKill™ total-spectrum herbicide, it '
        'thrives in otherwise lifeless soil — a testament to Monsatan\'s '
        'commitment to efficiency.',
    emoji: '🌽',
    features: [
      'Exclusively compatible with MonsatanKill™ herbicide',
      'Patented genome — a licence is required to grow',
      'Repels all known insects (and several bird species)',
      'Unauthorised seed saving is a prosecutable offence',
    ],
    disclaimer:
        '* Unauthorised seed saving constitutes IP theft under Monsatan '
        'Corp licence agreement §47(b) and will be prosecuted to the full '
        'extent of applicable international trade law.',
  ),
  Product(
    name: 'MegaYield™ Soy',
    tagline: 'More is Never Enough',
    description:
        'MegaYield™ soybeans push the boundaries of what a legume can be. '
        'Our QGE (Quantum Gene Expression) technology unlocks latent yield '
        'potential hidden inside standard soy DNA, delivering harvests your '
        'grandchildren\'s soil will never forget.',
    emoji: '🫘',
    features: [
      '400% more soybeans per plant',
      'Thrives in optimised monoculture conditions',
      'Soil microbiome "simplified" for peak efficiency',
      'Post-contract remediation costs borne by landowner',
    ],
    disclaimer:
        '* Soil remediation costs are the sole responsibility of the land '
        'owner following contract expiration. Monsatan accepts no liability '
        'for topsoil depletion, microbial extinction events, or dust.',
  ),
  Product(
    name: 'SunSeed™ Sunflower',
    tagline: 'Harnessing Two Suns',
    description:
        'Why rely on one star? SunSeed™ sunflowers integrate micro-photovoltaic '
        'cells into their petals, generating supplemental bio-electrical energy '
        'to supercharge photosynthesis. Beautiful, productive, and always '
        'transmitting.',
    emoji: '🌻',
    features: [
      'Dual-mode solar energy absorption',
      'Embedded nano-sensors (field data owned by Monsatan)',
      'Pollinators attracted — and gently disoriented',
      '99.7% carbon neutral (Scope 1 emissions only)',
    ],
    disclaimer:
        '* All field data including GPS coordinates, soil composition, '
        'yield metrics, and surrounding biodiversity readings are the '
        'exclusive property of Monsatan Corp under the SunSeed™ EULA §12.',
  ),
  Product(
    name: 'AquaRoot™ Rice',
    tagline: 'Water? Optional.',
    description:
        'AquaRoot™ Rice rewrites the rules of hydrology. Our patented '
        'moisture-harvesting root network draws water from atmospheric '
        'humidity, neighbouring crops, and underground aquifers — up to '
        '50 metres in any direction.',
    emoji: '🌾',
    features: [
      'Draws moisture from a 50 m radius',
      'Drought-proof (neighbouring farms may experience drought)',
      'Tolerates salinity up to full seawater concentration',
      'Approved for use on 3 of 7 continents',
    ],
    disclaimer:
        '* Monsatan Corp accepts no liability for neighbouring crop '
        'failures, aquifer depletion events, or any related legal action '
        'described by regulators as a "hydrological resource conflict."',
  ),
  Product(
    name: 'NightShade™ Tomato',
    tagline: 'Red Has Never Been So Dark',
    description:
        'NightShade™ Tomatoes achieve flawless visual ripeness through our '
        'PigmentLock™ process. Shelf-stable for up to 18 months at room '
        'temperature, they look harvest-fresh on arrival — wherever that '
        'arrival may be.',
    emoji: '🍅',
    features: [
      '18-month ambient shelf stability',
      'Texture optimised for intercontinental freight',
      'Flavour profile independently described as "tomato-adjacent"',
      'Zero viable seeds — can\'t save what was never there',
    ],
    disclaimer:
        '* Full nutritional data is pending independent review. All '
        'existing third-party reviews are subject to a binding NDA. '
        'Consult your legal representative before consuming.',
  ),
];
