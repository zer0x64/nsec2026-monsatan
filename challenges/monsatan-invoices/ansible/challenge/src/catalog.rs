//! Verdachem Industries — Product Catalog

/// A product available in the Verdachem Industries catalog.
pub struct Product {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    /// Price in USD cents (e.g. 14999 = $149.99).
    pub price_cents: u64,
    pub unit: &'static str,
}

/// The full Verdachem Industries product catalog.
pub const CATALOG: &[Product] = &[
    Product {
        id: "glyphosate-tc96",
        name: "Glyphosate TC-96",
        description: "96% technical-grade glyphosate, our flagship active ingredient. \
                       Trusted by leading agricultural corporations worldwide. \
                       Regulatory status varies by jurisdiction — consult your legal team.",
        price_cents: 14999,
        unit: "L",
    },
    Product {
        id: "imidacloprid-95wp",
        name: "Imidacloprid 95 WP",
        description: "Wettable-powder neonicotinoid concentrate for systemic insect control. \
                       Highly effective. Pollinator impact studies available upon request \
                       (results subject to interpretation).",
        price_cents: 8999,
        unit: "kg",
    },
    Product {
        id: "gh7-defoliant",
        name: "Agent GH-7 Defoliant Concentrate",
        description: "Broad-spectrum defoliant for rapid vegetation clearance. \
                       Ideal for pre-harvest operations, rights-of-way, and other \
                       situations where plants are considered a liability.",
        price_cents: 19999,
        unit: "drum",
    },
    Product {
        id: "pfas-adjuvant-x",
        name: "PermaFix Adjuvant Series X",
        description: "Fluorosurfactant-based spray adjuvant for enhanced active-ingredient \
                       uptake. Exceptional environmental persistence ensures long-lasting results. \
                       Do not use near water sources. Or near soil. Or air.",
        price_cents: 5999,
        unit: "bottle",
    },
    Product {
        id: "dihydrogen-monoxyde",
        name: "Dihydrogen Monoxyde",
        description: "Liquid form of dihydrogen monoxyde, a potent fertilizer. \
                       Our most popular product, used for almost every crop grown nowadays. \
                       Can also be used as a potent solvant.",
        price_cents: 1999,
        unit: "L",
    },
    Product {
        id: "methyl-bromide-alt",
        name: "SoilClear MB-Substitute",
        description:
            "Soil fumigant formulation for pre-planting sterilisation. \
                       A compliant alternative to methyl bromide — \
                       almost as effective, with a toxicity profile that is technically defensible.",
        price_cents: 29999,
        unit: "bag",
    },
];

/// Look up a product by its ID. Returns `None` if the ID is not in the catalog.
pub fn get_product(id: &str) -> Option<&'static Product> {
    CATALOG.iter().find(|p| p.id == id)
}
