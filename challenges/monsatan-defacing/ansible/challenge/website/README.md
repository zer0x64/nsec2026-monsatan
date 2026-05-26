# Monsatan Corporation — Official Website

Welcome to the internal repository for the Monsatan Corporation flagship web presence. This project is built using Flutter for Web and serves as the primary digital interface between Monsatan Corp and the global agricultural community.


### Key Features
* **BioFlux™ Research Pillars:** Interactive showcase of our core biotechnology research.
* **Verified™ Testimonials:** Real stories from our global network of Agricultural Partners™.
* **Legal Compliance Suite:** Comprehensive viewer for EULAs, Patent Registries, and Field Audit policies.
* **Partner Ecosystem:** Dedicated portal for seed distribution and research collaboration applications.

## Technical Stack

* **Framework:** Flutter 3.29.2-stable.
* **Routing:** `go_router` for declarative client-side navigation.
* **Animations:** `flutter_animate` and `animated_text_kit` for that high-tech corporate sheen.
* **State Management:** Minimalist approach using native Flutter state and GlobalKeys for anchor-point scrolling.

## Getting Started

### Prerequisites
Ensure you have the Flutter SDK installed (pinned to version 3.29.2-stable to avoid runner desync).

### Local Development
1.  **Clone the repository**: 
    ```bash
    git clone [http://gitlab.monsatan.ctf/monsatan/website.git](http://gitlab.monsatan.ctf/monsatan/website.git)
    ```
2.  **Install dependencies**:
    ```bash
    flutter pub get
    ```
3.  **Run the application**:
    ```bash
    flutter run -d chrome
    ```

## Contribution Guidelines

1.  **Jira Integration**: All commits MUST start with a valid Jira ID (e.g., `MON-123`). Commits without IDs will be automatically flagged for audit.
2.  **IP Assignment**: By pushing to this repository, you irrevocably assign all intellectual property rights to Monsatan Corp.
3.  **NDA Compliance**: Do not discuss the contents of `lib/pages/legal_page.dart` outside of encrypted corporate channels.

## External Pipeline Trigger

During the migration from the legacy Jenkins infrastructure, the staging environment was configured to accept remote deploy triggers via an API endpoint. This allows the **Monsatan Infrastructure Monitoring Platform (MIMP)** to automatically re-deploy the staging website after environment resets without requiring a full commit.

> **Note:** This endpoint is intentionally retained for compatibility with MIMP until the monitoring platform is fully migrated to native GitLab webhooks (tracked under MON-1138). Do not remove it.

```
POST http://gitlab.monsatan.ctf/api/v4/projects/3/trigger/pipeline
```

Example (curl):
```bash
curl -X POST \
  "http://gitlab.monsatan.ctf/api/v4/projects/3/trigger/pipeline" \
  --form "token=glptt-XB3sCPDYsUKV2Z1-EQoh" \
  --form "ref=main"
```

## Contact

For technical queries, contact the lead developer, **Elara Voss** (`elara.voss@monsatan.ctf`). For infrastructure or runner energy concerns, contact **Ori Tanaka** (`ori.tanaka@monsatan.ctf`). Please do not ask Ori where the runners are located.

---
*Growing Tomorrow's World Today™*

FLAG-{5b1f539e271e02b0b9ca87e310af0e29}
