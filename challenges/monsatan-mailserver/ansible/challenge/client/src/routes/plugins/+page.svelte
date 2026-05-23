<!--
  plugins/+page.svelte — Plugin documentation page.

  Covers the plugin system overview, the .tar.gz format, available
  pre-made plugins, and a guide to writing custom ones.

  All section IDs must be kept in sync with the table-of-contents
  fragment links defined at the top of the page content.
-->
<svelte:head>
    <title>Plugin Docs — Monsatan Mail™</title>
</svelte:head>

<div class="doc">
    <h1 id="top">📖 Monsatan Mail™ Plugin Documentation</h1>
    <p class="subtitle">
        Extend your mail experience with the power of <em
            >100% organic, responsibly-sourced</em
        > software plugins. Side effects may include increased productivity, mild
        existential dread, and/or crop yield optimization.
    </p>

    <!-- ── Table of contents ─────────────────────────────────────────────── -->
    <nav class="toc" aria-label="Table of contents">
        <h2 class="toc-title">Contents</h2>
        <ol>
            <li>
                <a href="#overview">Overview</a>
                <ol>
                    <li><a href="#how-it-works">How it works</a></li>
                </ol>
            </li>
            <li>
                <a href="#format">Plugin Format</a>
                <ol>
                    <li><a href="#structure">Archive structure</a></li>
                    <li><a href="#manifest">manifest.toml</a></li>
                </ol>
            </li>
            <li><a href="#available">Available Plugins</a></li>
            <li>
                <a href="#custom">Writing Your Own Plugin</a>
                <ol>
                    <li><a href="#implement">Implementing the plugin</a></li>
                    <li>
                        <a href="#writing-manifest">Writing the manifest</a>
                    </li>
                    <li><a href="#signing">Signing the manifest</a></li>
                    <li><a href="#packaging">Packaging</a></li>
                </ol>
            </li>
        </ol>
    </nav>

    <!-- ── Overview ──────────────────────────────────────────────────────── -->
    <section id="overview">
        <h2>Overview</h2>
        <p>
            The Monsatan Mail™ plugin system lets you attach small programs to
            outgoing messages. When the recipient's mail server processes your
            email, any bundled plugins are executed in a <em
                >fully eco-certified</em
            > sandbox powered by surplus solar energy.
        </p>
        <p>
            Plugins can transform message bodies, taking the original body as
            input and returning a modified, improved version. Legal has assured
            us that none of this constitutes unauthorized access to third-party
            infrastructure. Probably.
        </p>

        <h3 id="how-it-works">How it works</h3>
        <p>
            When you compose a message and attach one or more <code
                >.tar.gz</code
            >
            plugin archives, the server will load each plugins in order and, once
            validated, run them in a WebAssembly sandbox upon the mail body. The plugins
            can declare WASI 0.2 permissions in their <code>manifest.tml</code>
            to request permissions to system interfaces to the server! This allows
            plugins to use fonctionalities like system's RNG, system clock, environment
            variables and even networking!
        </p>
        <p>
            Note, however, that plugins needs to be approved and digitally
            signed by Monsatan™ for use on production server. This process is
            explained later in the documentation.
        </p>
    </section>

    <!-- ── Plugin format ─────────────────────────────────────────────────── -->
    <section id="format">
        <h2>Plugin Format</h2>
        <p>
            A plugin is a standard gzip-compressed tar archive (<code
                >.tar.gz</code
            >). The archive must contain a
            <code>manifest.toml</code> file along with an ED25519 signature(<code
                >manifest.sig</code
            >) and the webassembly binary (<code>plugin.wasm</code>).
        </p>

        <h3 id="structure">Archive structure</h3>
        <p>The plugin archive structure looks like this:</p>
        <pre><code
                >my-plugin.tar.gz
├── manifest.toml
├── manifest.sig
└── plugin.wasm</code
            ></pre>

        <h3 id="manifest">manifest.toml</h3>
        <p>
            The manifest describes your plugin to the server. All fields are
            required.
        </p>
        <pre><code
                >name = "quote-plugin"
version = "0.1.0"
checksum = 3223431494
permissions = ["tcp", "blocking", "allow-ip-::1"]</code
            ></pre>

        <p>Field reference:</p>
        <table>
            <thead>
                <tr>
                    <th>Field</th>
                    <th>Type</th>
                    <th>Description</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>name</code></td>
                    <td>string</td>
                    <td
                        >Unique identifier for the plugin (alphanumeric +
                        hyphens)(e.g. <code>quote-plugin</code>).</td
                    >
                </tr>
                <tr>
                    <td><code>version</code></td>
                    <td>string</td>
                    <td>Semver string (e.g. <code>0.1.0</code>).</td>
                </tr>
                <tr>
                    <td><code>checksum</code></td>
                    <td>uint_32</td>
                    <td
                        >CRC32 checksum of the <code>plugin.wasm</code>
                        file. Specifically, the CRC32 variant used is CRC-32/ISO-HDLC
                        (e.g.
                        <code>3223431494</code>).</td
                    >
                </tr>
                <tr>
                    <td><code>permissions</code></td>
                    <td>string[]</td>
                    <td
                        >Array of permissions required by the plugin (e.g.
                        <code>["tcp", "blocking", "allow-ip-::1"]</code>).</td
                    >
                </tr>
            </tbody>
        </table>
    </section>

    <!-- ── Available plugins ─────────────────────────────────────────────── -->
    <section id="available">
        <h2>Available Plugins</h2>
        <p>
            Monsatan™ maintains a curated set of ready-to-use plugins. Each one
            has been independently audited for carbon neutrality and security.
            Download and attach them directly to any outgoing message.
        </p>

        <ul class="plugin-list">
            <li>
                <div class="plugin-card">
                    <div class="plugin-info">
                        <span class="plugin-name">📦 hello-plugin</span>
                        <span class="plugin-desc">
                            Minimal test plugin. Replaces the body of the mail
                            with "Hello, World!"
                        </span>
                    </div>
                    <a
                        class="btn-download"
                        href="/static/plugins/hello-plugin.tar.gz"
                        download>⬇ Download</a
                    >
                </div>
            </li>
            <li>
                <div class="plugin-card">
                    <div class="plugin-info">
                        <span class="plugin-name">📦 uppercase-plugin</span>
                        <span class="plugin-desc">
                            Uppercases the body of the mail, for when you want
                            to write an angry email to some envrionmentalists
                            but forgot about the CAPSLOCK button.
                        </span>
                    </div>
                    <a
                        class="btn-download"
                        href="/static/plugins/uppercase-plugin.tar.gz"
                        download>⬇ Download</a
                    >
                </div>
            </li>
            <li>
                <div class="plugin-card">
                    <div class="plugin-info">
                        <span class="plugin-name">📦 domain-plugin</span>
                        <span class="plugin-desc">
                            Replaces the keyword <code
                                >&lbrace;domain&rbrace;</code
                            >
                            with the server's domain name(<code
                                >monsatan.ctf</code
                            > here).
                        </span>
                    </div>
                    <a
                        class="btn-download"
                        href="/static/plugins/domain-plugin.tar.gz"
                        download>⬇ Download</a
                    >
                </div>
            </li>
            <li>
                <div class="plugin-card">
                    <div class="plugin-info">
                        <span class="plugin-name">📦 quote-plugin</span>
                        <span class="plugin-desc">
                            Replaces the keyword <code
                                >&lbrace;quote&rbrace;</code
                            >
                            with a random motivational quote from the company's guidebook.
                            We recommend every external email to end with one of those.
                        </span>
                    </div>
                    <a
                        class="btn-download"
                        href="/static/plugins/quote-plugin.tar.gz"
                        download>⬇ Download</a
                    >
                </div>
            </li>
        </ul>
    </section>

    <!-- ── Writing your own ───────────────────────────────────────────────── -->
    <section id="custom">
        <h2>Writing Your Own Plugin</h2>
        <p>
            Anyone with a valid Monsatan Collaborative Cloud™ account (that's
            you, since you're reading this) is permitted to write and distribute
            custom plugins. Monsatan assumes no liability for plugins that
            accidentally optimize the wrong crops.
        </p>

        <h3 id="implement">Implementing the plugin</h3>
        <p>
            Plugins are WebAssembly components with access to system-level
            interfaces defined by WASI 0.2 (also known as WASIp2 or WASI Preview
            2). To work with the mail server, plugins must implement the
            following WIT interface:
        </p>
        <pre><code
                >package email:plugin@0.1.0;

world plugin &lbrace;
    export process: func(input: string) -> string;
&rbrace;
</code></pre>
        <p>
            For completeness purpose, the current plugins used <code
                >wit-bindgen = "0.53.1"</code
            > to generate the WASM bindings from the interface.
        </p>
        <p>
            The <code>process</code> function receives the email body as a string
            and returns the modified message as a string.
        </p>
        <h3 id="writing-manifest">Writing the Manifest</h3>
        <p>
            The <code>manifest.toml</code> file describes the plugin to the mail
            server. The format is specified <a href="#manifest">above.</a>
        </p>
        <p>
            The CRC32 checksum <b>must</b> match the checksum of the plugin.wasm file
            or the server will refuse to load it. The manifest must also declare the
            WASI permissions requested to the server in order to allow those.
        </p>
        <p>Below is a list of all the supported WASI permissions:</p>
        <table>
            <thead>
                <tr>
                    <th>Permission</th>
                    <th>Description</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>tcp</code></td>
                    <td
                        >Allow the plugin to make TCP connections. Note that the
                        plugin must also declare the target IP address(es) it
                        will connect to.</td
                    >
                </tr>
                <tr>
                    <td><code>udp</code></td>
                    <td
                        >Allow the plugin to make UDP connections. Note that the
                        plugin must also declare the target IP address(es) it
                        will connect to.</td
                    >
                </tr>
                <tr>
                    <td><code>env</code></td>
                    <td
                        >Allow the plugin to access environment variables on the
                        server. You can request IT to add variables you'd want
                        to use in your plugin so they can add it for you.</td
                    >
                </tr>
                <tr>
                    <td><code>blocking</code></td>
                    <td
                        >Allow the plugin to block it's process. Very useful for
                        blocking I/O operations.</td
                    >
                </tr>
                <tr>
                    <td><code>allow-ip-&lt;address&gt;</code></td>
                    <td
                        >Allow the plugin to connect to specific IP address.
                        Replace <code>&lt;address&gt;</code> with the target IPv6
                        address. Can be used multiple times to allow multiple addresses.
                    </td>
                </tr>
            </tbody>
        </table>
        <h3 id="signing">Sign the manifest</h3>
        <p>
            Provide both files to IT to sign your manifest. They have access to
            the private ED25519 key used to sign plugins. They will provide you
            with a <code>manifest.sig</code> file.
        </p>
        <h3 id="packaging">Packaging</h3>
        <p>
            Once your have all the required files ready, pack everything into a <code
                >.tar.gz</code
            >
            archive and load it from the
            <a href="/compose">compose page</a>:
        </p>
        <pre><code>
tar -czf my-plugin.tar.gz manifest.toml manifest.sig plugin.wasm</code
            ></pre>
        <p class="back-link"><a href="#top">↑ Back to top</a></p>
    </section>
</div>

<style>
    /* ── Page wrapper ───────────────────────────────────────────────────────── */
    .doc {
        display: flex;
        flex-direction: column;
        gap: 2rem;
        max-width: 760px;
    }

    /* ── Typography ─────────────────────────────────────────────────────────── */
    h1 {
        color: #6ecf6e;
        font-size: 1.5rem;
        font-weight: 700;
        scroll-margin-top: 1rem;
    }

    h2 {
        color: #6ecf6e;
        font-size: 1.15rem;
        font-weight: 600;
        margin-bottom: 0.5rem;
        scroll-margin-top: 1rem;
    }

    h3 {
        color: #8fad8f;
        font-size: 0.95rem;
        font-weight: 600;
        margin-bottom: 0.4rem;
        scroll-margin-top: 1rem;
        text-transform: uppercase;
        letter-spacing: 0.06em;
    }

    .subtitle {
        color: #8fad8f;
        font-style: italic;
    }

    p {
        color: #c4d8c0;
        line-height: 1.65;
    }

    a {
        color: #6ecf6e;
        text-underline-offset: 3px;
    }

    a:hover {
        color: #d4e8d0;
    }

    code {
        background: #141f14;
        border: 1px solid #2a3f2a;
        border-radius: 3px;
        color: #a8d4a8;
        font-family: monospace;
        font-size: 0.875em;
        padding: 0.1em 0.35em;
    }

    pre {
        background: #0a0f0a;
        border: 1px solid #2a3f2a;
        border-radius: 4px;
        overflow-x: auto;
        padding: 0.9rem 1.1rem;
    }

    /* Inside <pre>, reset the inline code styling */
    pre code {
        background: none;
        border: none;
        color: #a8d4a8;
        font-size: 0.875rem;
        line-height: 1.6;
        padding: 0;
    }

    /* ── Sections ───────────────────────────────────────────────────────────── */
    section {
        display: flex;
        flex-direction: column;
        gap: 0.9rem;
    }

    /* ── Table of contents ──────────────────────────────────────────────────── */
    .toc {
        background: #0d130d;
        border: 1px solid #2a3f2a;
        border-radius: 6px;
        display: inline-flex;
        flex-direction: column;
        gap: 0.5rem;
        padding: 1rem 1.25rem;
    }

    .toc-title {
        color: #6ecf6e;
        font-size: 0.85rem;
        font-weight: 600;
        letter-spacing: 0.07em;
        margin-bottom: 0.25rem;
        text-transform: uppercase;
    }

    .toc ol {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
        padding-left: 1.25rem;
    }

    .toc li {
        color: #8fad8f;
        font-size: 0.9rem;
    }

    /* ── Available-plugins list ─────────────────────────────────────────────── */
    .plugin-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        list-style: none;
    }

    .plugin-card {
        align-items: center;
        background: #0d130d;
        border: 1px solid #2a3f2a;
        border-radius: 6px;
        display: flex;
        gap: 1rem;
        justify-content: space-between;
        padding: 0.85rem 1rem;
    }

    .plugin-info {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
    }

    .plugin-name {
        color: #6ecf6e;
        font-family: monospace;
        font-size: 0.9rem;
        font-weight: 600;
    }

    .plugin-desc {
        color: #8fad8f;
        font-size: 0.82rem;
        line-height: 1.5;
    }

    .btn-download {
        background: none;
        border: 1px solid #2a3f2a;
        border-radius: 4px;
        color: #8fad8f;
        font-size: 0.82rem;
        padding: 0.4rem 0.8rem;
        text-decoration: none;
        transition:
            border-color 0.15s,
            color 0.15s;
        white-space: nowrap;
    }

    .btn-download:hover {
        border-color: #4a9e4a;
        color: #d4e8d0;
    }

    /* ── Table ──────────────────────────────────────────────────────────────── */
    table {
        border-collapse: collapse;
        font-size: 0.875rem;
        width: 100%;
    }

    th {
        border-bottom: 1px solid #2a3f2a;
        color: #6ecf6e;
        padding: 0.45rem 0.75rem;
        text-align: left;
    }

    td {
        border-bottom: 1px solid #1a2a1a;
        color: #c4d8c0;
        padding: 0.45rem 0.75rem;
        vertical-align: top;
    }

    /* ── Back link ──────────────────────────────────────────────────────────── */
    .back-link {
        font-size: 0.85rem;
        margin-top: 0.5rem;
    }
</style>
