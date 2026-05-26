#!/usr/bin/env bash
# mock_git_history.sh
#
# Populates the current git repository with a realistic fake commit history
# for the Monsatan Corp website, using the project's developer personas.
#
# Usage (from the website/ directory):
#   bash mock_git_history.sh
#
# The script is safe to run on a repo that has no commits yet.
# It will exit immediately on any error.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# --- JIRA ID GENERATOR ---
# Start at a believable number
JIRA_ID=427

# Helper function to increment Jira ID by a random amount
next_jira() {
    local increment=$((1 + RANDOM % 20))
    JIRA_ID=$((JIRA_ID + increment))
}

# ---------------------------------------------------------------------------
# Helper: make a commit with an explicit author, committer, and date.
#
#   make_commit <name> <email> <iso-date> <message> [file ...]
#
# If one or more file paths are provided they are staged first.
# If no files are provided --allow-empty is used (for fix/chore commits
# that touch already-staged content or have no diff to show).
# ---------------------------------------------------------------------------
make_commit() {
    local name="$1"
    local email="$2"
    local date="$3"
    local msg="MON-$JIRA_ID $4"
    shift 4

    local is_empty=true
    if [ "$#" -gt 0 ]; then
        git add -- "$@"
        is_empty=false
    fi

    GIT_AUTHOR_NAME="$name" \
    GIT_AUTHOR_EMAIL="$email" \
    GIT_AUTHOR_DATE="$date" \
    GIT_COMMITTER_NAME="$name" \
    GIT_COMMITTER_EMAIL="$email" \
    GIT_COMMITTER_DATE="$date" \
    git commit --allow-empty -m "$msg"

    # If the commit was empty (fix/chore), ask to push to trigger a pipeline
    if [ "$is_empty" = true ]; then
        echo -e "\n\033[1;33m[!] Empty commit detected: $msg\033[0m"
        read -p "Push this commit to trigger a GitLab pipeline? (y/n): " confirm
        if [[ "$confirm" =~ ^[Yy]$ ]]; then
            # Ensure we push to the current branch
            git push origin "$(git rev-parse --abbrev-ref HEAD)"
            echo "[INFO] Push successful. Pipeline should be starting..."
        fi
    fi

    next_jira
}

# ---------------------------------------------------------------------------
# Helper: make a commit that *removes* files (git rm).
#
#   make_delete_commit <name> <email> <iso-date> <message> <file ...>
# ---------------------------------------------------------------------------
make_delete_commit() {
    local name="$1"
    local email="$2"
    local date="$3"
    local msg="MON-$JIRA_ID $4"
    shift 4

    git rm -f -- "$@"

    GIT_AUTHOR_NAME="$name" \
    GIT_AUTHOR_EMAIL="$email" \
    GIT_AUTHOR_DATE="$date" \
    GIT_COMMITTER_NAME="$name" \
    GIT_COMMITTER_EMAIL="$email" \
    GIT_COMMITTER_DATE="$date" \
    git commit -m "$msg"

    next_jira
}

# ---------------------------------------------------------------------------
# Developer aliases (name / email / "personality handle" for readability)
# ---------------------------------------------------------------------------
ELARA_NAME="Elara Voss"
ELARA_EMAIL="elara.voss@monsatan.ctf"

DESMOND_NAME="Desmond Fray"
DESMOND_EMAIL="desmond.fray@monsatan.ctf"

LINA_NAME="Lina Soren"
LINA_EMAIL="lina.soren@monsatan.ctf"

ORI_NAME="Ori Tanaka"
ORI_EMAIL="ori.tanaka@monsatan.ctf"

PRIYA_NAME="Priya Mahl"
PRIYA_EMAIL="priya.mahl@monsatan.ctf"

# ---------------------------------------------------------------------------
# Commit 1 — Project scaffold
# Elara initialises the Flutter web project.
# ---------------------------------------------------------------------------
make_commit \
    "$ELARA_NAME" "$ELARA_EMAIL" "2025-02-03T09:12:00+0000" \
    "chore: initialise Flutter web project

Scaffolded with \`flutter create --platforms=web\`.
Removed the default counter demo; we will build from scratch." \
    pubspec.yaml \
    pubspec.lock \
    analysis_options.yaml \
    .metadata \
    .gitignore \
    web/index.html \
    web/manifest.json \
    web/favicon.png \
    "web/icons/" \
    lib/main.dart \
    test/widget_test.dart

# ---------------------------------------------------------------------------
# Commit 2 — CI pipeline
# Ori adds the GitLab CI skeleton. Notes are deliberately vague about infra.
# ci_test.sh is accidentally committed alongside — it contains a hardcoded
# runner registration token that should never have left Ori's laptop.
# ---------------------------------------------------------------------------
make_commit \
    "$ORI_NAME" "$ORI_EMAIL" "2025-02-03T14:47:00+0000" \
    "chore(ci): add GitLab CI pipeline skeleton

Stages: check -> build -> package -> publish -> deploy -> status.
Build artefacts expire after 1 day to save space on the runners.
Runners are tagged \`monsatan\`. Do not ask where the runners are." \
    .gitlab-ci.yml \
    ci_test.sh

# ---------------------------------------------------------------------------
# Commit 3 — Emergency removal of ci_test.sh
# Ori notices the token exposure and removes the file immediately.
# The damage is already done — the token lives in git history.
# ---------------------------------------------------------------------------
make_delete_commit \
    "$ORI_NAME" "$ORI_EMAIL" "2025-02-03T15:03:00+0000" \
    "chore(ci): remove ci_test.sh — token accidentally committed

ci_test.sh contained a plaintext internal GitLab tool token.
File has been removed. Rotation request submitted to infra-security@monsatan.ctf.
Awaiting confirmation from the SecOps team. Do NOT use the token from git history.
I mean it. I am watching the logs." \
    ci_test.sh

# ---------------------------------------------------------------------------
# Commit 4 — Brand theme
# Lina defines the colour palette and typography before touching any widgets.
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-02-05T10:23:00+0000" \
    "feat(theme): define brand colour palette and typography system

Dark-green corporate base with solar-gold accents.
Playfair Display for headings, DM Sans for body copy.
Added Breakpoints constants so every widget uses the same values." \
    lib/theme.dart

# ---------------------------------------------------------------------------
# Commit 5 — Data models
# Desmond defines the models. He called the file structure "event-sourced."
# It is a data class. There are two fields.
# ---------------------------------------------------------------------------
make_commit \
    "$DESMOND_NAME" "$DESMOND_EMAIL" "2025-02-11T11:05:00+0000" \
    "feat(models): add Product and Testimonial domain models

Immutable value objects. Kept deliberately thin — presentation logic
belongs in widgets, not here.
(Yes, these are const. No, I will not add toJson. Ask Elara.)" \
    lib/models/product.dart \
    lib/models/testimonial.dart

# ---------------------------------------------------------------------------
# Commit 6 — Product data
# Desmond populates the product catalogue.
# ---------------------------------------------------------------------------
make_commit \
    "$DESMOND_NAME" "$DESMOND_EMAIL" "2025-02-14T15:33:00+0000" \
    "feat(data): populate product catalogue with initial lineup

Six flagship products: RadiGrow™ Wheat, HerbiShield™ Corn,
MegaYield™ Soy, SunSeed™ Sunflower, AquaRoot™ Rice, NightShade™ Tomato.
All disclaimers reviewed by Legal. Legal said to add more disclaimers." \
    lib/data/products_data.dart

# ---------------------------------------------------------------------------
# Commit 7 — Navigation bar
# Lina builds the nav bar. She added a skip-to-content link nobody asked for.
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-02-18T09:44:00+0000" \
    "feat(nav): implement responsive navigation bar

Transparent on hero, blurs to a dark surface on scroll.
Nav links scroll to anchored sections via GlobalKey callbacks.
Mobile: collapses to a hamburger menu." \
    lib/widgets/nav_bar.dart

# ---------------------------------------------------------------------------
# Commit 8 — Hero section
# Lina ships the hero. The typewriter effect was her idea. She is proud of it.
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-02-21T14:17:00+0000" \
    "feat(hero): add full-screen hero section with typewriter headline

Subtle grid background, animated fade-ins, and a stats strip
that makes the company sound simultaneously impressive and threatening.
CTA buttons scroll to Products and Technology sections." \
    lib/widgets/hero_section.dart

# ---------------------------------------------------------------------------
# Commit 9 — Products section
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-02-25T11:52:00+0000" \
    "feat(products): add product showcase section

Alternating card layout (light/dark) driven by is_even.
Each card shows emoji, tagline, feature list, and small-print disclaimer.
Scroll-triggered fade-in via visibility_detector." \
    lib/widgets/products_section.dart

# ---------------------------------------------------------------------------
# Commit 10 — Technology section
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-02-28T16:08:00+0000" \
    "feat(technology): add technology section

Three-column grid of research pillars with icon tiles.
Collapses to single column on mobile. Accessibility pass done —
all icons have semantic labels (you're welcome, everyone)." \
    lib/widgets/technology_section.dart

# ---------------------------------------------------------------------------
# Commit 11 — About section
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-03-04T10:30:00+0000" \
    "feat(about): add about section

Mission statement, founding year (2031), and values pillars.
Colour-accent dividers between values to break up the wall of text.
Deadline was yesterday. Section is done. These facts are unrelated." \
    lib/widgets/about_section.dart

# ---------------------------------------------------------------------------
# Commit 12 — Testimonials data
# Desmond adds the testimonials. He insists they are "user-generated content."
# ---------------------------------------------------------------------------
make_commit \
    "$DESMOND_NAME" "$DESMOND_EMAIL" "2025-03-07T13:21:00+0000" \
    "feat(data): add Verified™ customer testimonial dataset

Six testimonials sourced from our Agricultural Partner™ network.
All quotes are real. The legal team has confirmed they are real.
Please do not ask the testimonial authors if they are real." \
    lib/data/testimonials_data.dart

# ---------------------------------------------------------------------------
# Commit 13 — Testimonials carousel
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-03-10T14:55:00+0000" \
    "feat(testimonials): add auto-scrolling testimonial carousel

Uses carousel_slider with 6 s auto-play and manual swipe support.
Quote marks rendered in solar-gold because Lina thinks it looks good.
(It does look good. No notes.)" \
    lib/widgets/testimonials_section.dart

# ---------------------------------------------------------------------------
# Commit 14 — Footer
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-03-13T11:10:00+0000" \
    "feat(footer): add site footer with navigation links

Links to all sections plus Legal pages. Wordmark with trademark symbol.
Copyright line reads the current year — determined at build time." \
    lib/widgets/footer_section.dart

# ---------------------------------------------------------------------------
# Commit 15 — App root + home page wiring
# Elara wires everything together and configures go_router.
# ---------------------------------------------------------------------------
make_commit \
    "$ELARA_NAME" "$ELARA_EMAIL" "2025-03-17T16:45:00+0000" \
    "feat(app): wire HomePage sections and configure client-side routing

go_router routes: / -> HomePage, /prospectus, /partner, /legal/:doc.
HomePage uses GlobalKeys + Scrollable.ensureVisible for anchor nav.
Confirmed: the scroll animation curve is easeInOut and I will not
be changing it. Do not open a ticket about the curve." \
    lib/app.dart \
    lib/pages/home_page.dart

# ---------------------------------------------------------------------------
# Commit 16 — Legal page
# Elara writes the legal docs. She has read the NDA. She cannot comment.
# ---------------------------------------------------------------------------
make_commit \
    "$ELARA_NAME" "$ELARA_EMAIL" "2025-03-21T10:02:00+0000" \
    "feat(legal): add legal document viewer

Supports multiple doc slugs via /legal/:doc route parameter.
Includes Terms of Service and Privacy Policy.
The fine print at the bottom is load-bearing. Do not remove it." \
    lib/pages/legal_page.dart

# ---------------------------------------------------------------------------
# Commit 17 — Partner page
# ---------------------------------------------------------------------------
make_commit \
    "$ELARA_NAME" "$ELARA_EMAIL" "2025-03-26T14:30:00+0000" \
    "feat(partner): add partnership application page

Three partnership tiers: Contracted Grower, Distribution Partner,
Research Collaborator. Form validates client-side; submission is
a no-op until the backend endpoint is provisioned (Desmond's queue)." \
    lib/pages/partner_page.dart

# ---------------------------------------------------------------------------
# Commit 18 — Prospectus page
# ---------------------------------------------------------------------------
make_commit \
    "$ELARA_NAME" "$ELARA_EMAIL" "2025-03-31T09:18:00+0000" \
    "feat(prospectus): add investor prospectus page

Key metrics, growth projections, and fund allocation breakdown.
Charts are illustrative. Results may vary. Past performance of
our patented organisms is not indicative of future organism performance." \
    lib/pages/prospectus_page.dart

# ---------------------------------------------------------------------------
# Commit 19 — Fix: hero footnote date (Priya found it, naturally)
# Priya has found 248 bugs this quarter. This was number 211.
# ---------------------------------------------------------------------------
make_commit \
    "$PRIYA_NAME" "$PRIYA_EMAIL" "2025-04-04T08:47:00+0000" \
    "fix(hero): correct pending-litigation footnote date

Footnote cited Q3 2037; internal records show it should be Q4 2037.
One quarter. I have been watching this footnote since February.
Bug #211 of Q1. None of them are the one that matters. I know."

# ---------------------------------------------------------------------------
# Commit 20 — Fix: accessibility labels (Lina, unprompted, on a Saturday)
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-04-08T11:03:00+0000" \
    "fix(a11y): add semantic labels to all icon-only interactive elements

Screen readers were announcing some buttons as 'button' with no context.
Fixed. Also I filed a complaint with the sun for excessive glare on my
monitor while doing this. Awaiting response. Reproducible on clear days."

# ---------------------------------------------------------------------------
# Commit 21 — Chore: CI runner image pinned (Ori)
# ---------------------------------------------------------------------------
make_commit \
    "$ORI_NAME" "$ORI_EMAIL" "2025-04-09T09:55:00+0000" \
    "chore(ci): pin Flutter runner image to 3.29.2-stable

Floating tags caused a silent SDK upgrade last sprint that broke the build
for six hours. The runners are healthy. The runners are happy.
Please do not ask where the runners are getting their energy from."

# ---------------------------------------------------------------------------
# Commit 22 — Fix: legal section numbering (Priya again)
# ---------------------------------------------------------------------------
make_commit \
    "$PRIYA_NAME" "$PRIYA_EMAIL" "2025-04-11T14:22:00+0000" \
    "fix(legal): §3 appeared twice in Privacy Policy — renumbered sections

Sections 3 through 7 were offset by one. Users who agreed to the
Terms under the old numbering have agreed to different terms.
Legal says this is fine. I have made a note that Legal said this is fine."

# ---------------------------------------------------------------------------
# Commit 23 — Fix: mobile hero overflow (Lina)
# ---------------------------------------------------------------------------
make_commit \
    "$LINA_NAME" "$LINA_EMAIL" "2025-04-15T10:41:00+0000" \
    "fix(mobile): prevent hero stat strip overflow on narrow viewports

Below ~360 px the four stat columns overflowed horizontally.
Switched the strip to a Wrap layout below the mobile breakpoint.
Tested on four devices. The sun's lighting was inconsistent on three."

# ---------------------------------------------------------------------------
# Commit 24 — Docs: README and developer contact sheet
# Elara adds documentation. Technically she has always been meaning to.
# ---------------------------------------------------------------------------
make_commit \
    "$ELARA_NAME" "$ELARA_EMAIL" "2025-04-17T16:59:00+0000" \
    "docs: add README and developer contact sheet

README covers local setup, build commands, and deployment notes.
developpers.md lists team contacts. Yes, there are two p's in the filename.
No, we are not renaming it. It has been this way since the kickoff doc
and renaming it would break four email templates. This is fine." \
    README.md \
    assets \
    developpers.md

echo ""
echo "Done. Git history created successfully."
echo ""
git log --oneline
