<p align="center">
  <img src="https://img.shields.io/crates/v/airust?style=flat-square&color=7c6ef0" alt="crates.io" />
  <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT" />
  <img src="https://img.shields.io/badge/rust-1.85+-orange?style=flat-square" alt="Rust 1.85+" />
  <img src="https://img.shields.io/badge/version-0.1.7-blue?style=flat-square" alt="v0.1.7" />
</p>

<h1 align="center">AIRust</h1>

<p align="center">
  <strong>A trainable, modular AI engine written in Rust</strong><br>
  Build intelligent agents, manage knowledge bases, extract wisdom from PDFs,<br>
  and deploy a full web dashboard — all without external AI APIs.
</p>

---

**Jump to:** [English](#english) | [Deutsch](#deutsch) | [Turkce](#turkce)

---

<a name="english"></a>

# English

## Table of Contents

1. [What is AIRust?](#1-what-is-airust)
2. [Key Features at a Glance](#2-key-features-at-a-glance)
3. [Installation & Setup](#3-installation--setup)
4. [Architecture Overview](#4-architecture-overview)
5. [Agent Types — The Brain of AIRust](#5-agent-types--the-brain-of-airust)
6. [Knowledge Base — The Memory](#6-knowledge-base--the-memory)
7. [PDF Processing — Learn from Documents](#7-pdf-processing--learn-from-documents)
8. [Web Dashboard — The Control Center](#8-web-dashboard--the-control-center)
9. [Bot Ecosystem — Automated Data Collection](#9-bot-ecosystem--automated-data-collection)
10. [CLI — Command Line Interface](#10-cli--command-line-interface)
11. [Using AIRust as a Library](#11-using-airust-as-a-library)
12. [Text Processing Utilities](#12-text-processing-utilities)
13. [Docker Deployment](#13-docker-deployment)
14. [API Reference](#14-api-reference)
15. [Configuration & Feature Flags](#15-configuration--feature-flags)
16. [Training Data Format](#16-training-data-format)
17. [Project Structure](#17-project-structure)
18. [Use Cases & Ideas](#18-use-cases--ideas)
19. [Version History](#19-version-history)
20. [License](#20-license)

---

## 1. What is AIRust?

AIRust is a **self-contained AI engine** written entirely in Rust. Unlike cloud-based AI solutions, AIRust runs **100% locally** — no OpenAI, no API keys, no internet required. You train it with your own data, and it answers questions using pattern matching, fuzzy search, and semantic similarity algorithms.

Think of it as: **Your own private AI assistant that you teach yourself.**

It comes with:
- Multiple intelligent agent types (exact matching, fuzzy matching, semantic search)
- A built-in web dashboard with chat interface
- PDF document processing for automatic knowledge extraction
- Web scraping bots for automated data collection
- A SQLite database for persistent storage
- Full REST API for integration with other systems

> **Summary:** AIRust is a local, trainable AI engine in Rust. You feed it knowledge (text, PDFs, web scraping), and it answers questions intelligently — no cloud, no API keys, fully private.

---

## 2. Key Features at a Glance

| Feature | Description |
|---------|-------------|
| **4 Agent Types** | Exact Match, Fuzzy Match, TF-IDF/BM25 Semantic, Context-Aware |
| **Knowledge Base** | JSON-based, compile-time embedded, runtime expandable |
| **PDF Processing** | Convert PDFs to structured training data with smart chunking |
| **Web Dashboard** | Full UI with chat, training manager, bot control, file browser |
| **Bot Ecosystem** | Automated web scraping with review workflow |
| **Vector Database** | Embedding storage and similarity search |
| **Chat History** | Persistent conversations with archiving |
| **Multi-Language UI** | English, German, Turkish |
| **REST API** | 50+ endpoints for full programmatic control |
| **WebSocket Console** | Live terminal with server logs, shell access, built-in commands |
| **Docker Support** | One-command deployment |
| **CLI Tools** | Interactive mode, query tools, PDF conversion |

> **Summary:** AIRust provides everything you need to build, train, and deploy an AI system: from agents and knowledge management to a full web interface and automated data collection.

---

## 3. Installation & Setup

### As a Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
airust = "0.1.7"
```

### Build from Source

```bash
git clone https://github.com/LEVOGNE/airust.git
cd airust
cargo build --release
```

### Run the Web Server

```bash
# Start on default port 7070
cargo run --release

# Custom port
cargo run --release -- --port 8080

# Run in background (detached mode)
cargo run --release -- -d

# Show landing page + dashboard (default: dashboard only)
cargo run --release -- --landingpage

# Stop background server
cargo run --release -- stop
```

Then open `http://localhost:7070` in your browser.

### With Docker

```bash
docker build -t airust .
docker run -p 7070:7070 airust
```

> **Summary:** You can use AIRust as a library in your own Rust projects, run it as a standalone web server, or deploy it in Docker. The web dashboard is available at port 7070 by default.

---

## 4. Architecture Overview

```
                    ┌──────────────────────────────────────┐
                    │           AIRust Engine               │
                    ├──────────┬───────────┬───────────────┤
                    │MatchAgent│TfidfAgent │ ContextAgent  │
                    │(exact/   │(BM25      │ (wraps any    │
                    │ fuzzy)   │ semantic) │  agent + mem) │
                    ├──────────┴───────────┴───────────────┤
                    │         Knowledge Base                │
                    │   (JSON / Embedded / Runtime)         │
                    ├──────────────────────────────────────┤
                    │         Text Processing               │
                    │  (tokenize, stopwords, similarity)    │
                    └────────┬──────────┬──────────────────┘
                             │          │
                    ┌────────▼──┐  ┌────▼──────────────┐
                    │  CLI Tool │  │  Web Server (Axum) │
                    │ (airust)  │  │  + REST API        │
                    └───────────┘  │  + WebSocket       │
                                   │  + SQLite DB       │
                                   │  + Bot Scheduler   │
                                   └───────────────────┘
```

**Core Traits** — Every agent implements these interfaces:

| Trait | Purpose |
|-------|---------|
| `Agent` | Base trait: `predict()`, `confidence()`, `can_answer()` |
| `TrainableAgent` | Adds `train()`, `add_example()` |
| `ContextualAgent` | Adds `add_context()`, `clear_context()` for conversation memory |
| `ConfidenceAgent` | Adds `calculate_confidence()`, `predict_top_n()` |

> **Summary:** AIRust has a layered architecture: agents do the thinking, the knowledge base stores the data, and the web server provides the interface. Everything communicates through clean Rust traits.

---

## 5. Agent Types — The Brain of AIRust

### 5.1 MatchAgent (Exact & Fuzzy)

The simplest and fastest agent. It compares your question directly against its training data.

**Exact Mode** — finds answers only when the question matches exactly (case-insensitive):
```rust
let agent = MatchAgent::new_exact();
```

**Fuzzy Mode** — tolerates typos using Levenshtein distance:
```rust
let agent = MatchAgent::new_fuzzy();

// With custom tolerance
let agent = MatchAgent::new(MatchingStrategy::Fuzzy(FuzzyOptions {
    max_distance: Some(5),        // Max allowed character changes
    threshold_factor: Some(0.3),  // 30% of input length as threshold
}));
```

**When to use:** FAQ bots, command recognition, structured Q&A where questions are predictable.

### 5.2 TfidfAgent (Semantic Search with BM25)

Uses the BM25 algorithm (the same algorithm behind search engines like Elasticsearch) to find the most relevant answer based on term frequency and document importance.

```rust
let agent = TfidfAgent::new();

// Fine-tune the algorithm
let agent = TfidfAgent::new()
    .with_bm25_params(1.5, 0.8);  // k1 = term scaling, b = length norm
```

**When to use:** Document search, knowledge bases with natural language questions, when exact matching is too strict.

### 5.3 ContextAgent (Conversational Memory)

Wraps any other agent and adds conversation memory. It remembers the last N exchanges so follow-up questions work naturally.

```rust
let base = TfidfAgent::new();
let agent = ContextAgent::new(base, 5)  // Remember 5 turns
    .with_context_format(ContextFormat::List);
```

**Context Formats:**
| Format | Example Output |
|--------|---------------|
| `QAPairs` | `Q: What is Rust? A: A programming language. Q: ...` |
| `List` | `[What is Rust? -> A programming language, ...]` |
| `Sentence` | `Previous questions: What is Rust? - A programming language; ...` |
| `Custom` | Your own formatting function |

**When to use:** Chatbots, interactive assistants, any scenario where users ask follow-up questions.

> **Summary:** AIRust offers three agent types: MatchAgent for fast exact/fuzzy matching, TfidfAgent for intelligent semantic search, and ContextAgent for conversational memory. Choose based on your use case, or combine them.

---

## 6. Knowledge Base — The Memory

The Knowledge Base is where all training data lives. It supports two modes:

### Compile-Time Embedding

Data from `knowledge/train.json` is baked into the binary at build time:
```rust
let kb = KnowledgeBase::from_embedded();
```

### Runtime Management

```rust
let mut kb = KnowledgeBase::new();

// Add entries
kb.add_example("What is Rust?", "A systems programming language", 1.0);

// Save to disk
kb.save(Some("knowledge/train.json".into()))?;

// Load from file
let kb = KnowledgeBase::load("knowledge/custom.json".into())?;

// Merge multiple knowledge bases
kb.merge(&other_kb);
```

### Data Format

```json
[
  {
    "input": "What is AIRust?",
    "output": { "Text": "A modular AI library in Rust." },
    "weight": 2.0,
    "metadata": { "source": "manual", "category": "general" }
  },
  {
    "input": "What agents are available?",
    "output": { "Markdown": "- **MatchAgent**\n- **TfidfAgent**\n- **ContextAgent**" },
    "weight": 1.0
  }
]
```

**Response Formats:** `Text`, `Markdown`, or `Json` — the agent automatically handles the right format.

**Legacy Support:** Old-style `{"input": "...", "output": "..."}` files (where output is a plain string) are still fully supported.

> **Summary:** The Knowledge Base stores everything the AI knows. Data can be embedded at compile time for zero-cost access or managed dynamically at runtime. Entries have weights (importance) and optional metadata.

---

## 7. PDF Processing — Learn from Documents

AIRust can extract knowledge from PDF documents automatically. It splits text into intelligent chunks and creates training examples.

### Command-Line Tool

```bash
# Basic conversion
cargo run --bin pdf2kb -- document.pdf

# Custom output path
cargo run --bin pdf2kb -- document.pdf output/my_kb.json

# Full configuration
cargo run --bin pdf2kb -- document.pdf \
  --min-chunk 100 \
  --max-chunk 2000 \
  --overlap 300 \
  --weight 1.5 \
  --no-metadata \
  --no-sentence-split
```

### In Code

```rust
use airust::{PdfLoader, PdfLoaderConfig};

let config = PdfLoaderConfig {
    min_chunk_size: 100,     // Minimum characters per chunk
    max_chunk_size: 1500,    // Maximum characters per chunk
    chunk_overlap: 250,      // Overlap between chunks for context
    default_weight: 1.2,     // Training weight
    include_metadata: true,  // Include page numbers, chunk info
    split_by_sentence: true, // Respect sentence boundaries
};

let loader = PdfLoader::with_config(config);
let kb = loader.pdf_to_knowledge_base("research-paper.pdf")?;

println!("Extracted {} training examples", kb.get_examples().len());
```

### Merging Multiple Sources

```bash
# Place multiple JSON files in knowledge/
# Then merge them all into train.json
cargo run --bin merge_kb
```

### How Chunking Works

```
PDF Document
  │
  ▼
┌─────────────────────────────────┐
│ Full extracted text             │
└──────────┬──────────────────────┘
           │ Split by sentences
           ▼
┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐
│Chunk1│ │Chunk2│ │Chunk3│ │Chunk4│  (with overlap)
└──────┘ └──────┘ └──────┘ └──────┘
           │
           ▼
   TrainingExample per chunk
   (with page number metadata)
```

> **Summary:** Feed PDFs into AIRust and it automatically creates structured training data. Intelligent chunking respects sentence boundaries and maintains context through overlapping segments. Merge multiple PDFs into one unified knowledge base.

---

## 8. Web Dashboard — The Control Center

Start the server with `cargo run` and open `http://localhost:7070`.

### Dashboard Tabs

| Tab | What it does |
|-----|-------------|
| **Chat** | Talk to your AI agent, see confidence scores, switch agents |
| **Training** | Manage training data with categories, import/export JSON |
| **Knowledge** | Browse, search, add, delete knowledge base entries |
| **Bots** | Create and manage web scraping bots |
| **Data Review** | Approve/reject data collected by bots |
| **Vectors** | Manage vector collections and embeddings |
| **Files** | Browse project files and SQLite database |
| **Console** | Real-time WebSocket log viewer with shell access |
| **Settings** | Theme (dark/light), language (EN/DE/TR), accent colors |

### Smart Settings via Chat

You can change settings by chatting naturally:

- *"Make the page dark"* — switches to dark theme
- *"Change to green background"* — updates accent color
- *"Use German language"* — switches UI language
- *"Mach die Seite dunkel"* — also works in German
- *"Turkce yap"* — switches to Turkish

### Agent Switching

Switch between agent types at any time via the API or UI:
- Exact Match
- Fuzzy Match
- TF-IDF (BM25)
- Context Agent (with conversation memory)

### Console — Real-Time Server Terminal

The console panel sits at the bottom of the dashboard and acts as a **live terminal** connected to your AIRust server via WebSocket (`/ws/console`).

**What it does:**
- Streams every server log (requests, errors, agent activity) to your browser in real time
- Lets you type commands directly — both built-in commands and arbitrary shell commands
- Shows color-coded output: `info` (blue), `warn` (yellow), `error` (red), `cmd` (green), `stdout`/`stderr`

**Built-in Commands:**

| Command | What it does |
|---------|-------------|
| `help` or `?` | Show list of available commands |
| `clear` | Clear all console output |
| `status` | Show AIRust version, server state, and working directory |
| `stop` | Gracefully shut down the server |
| `restart` | Restart the server process |
| *anything else* | Executed as a shell command (e.g. `ls`, `df -h`, `cat knowledge/train.json`) |

**UI Features:**

| Feature | How it works |
|---------|-------------|
| **Drag to resize** | Grab the console header bar and drag up/down to resize the panel |
| **Minimize/Expand** | Click the toggle button in the header to collapse or expand |
| **Command history** | Press arrow keys (up/down) to cycle through previous commands |
| **Auto-reconnect** | If the WebSocket disconnects, it automatically reconnects every 2 seconds |
| **Connection indicator** | Green dot = connected, red dot = disconnected |

**Technical Details:**
- Server keeps a ring buffer of **500 log entries** — when you connect, you receive the full history
- Client caps at **1000 DOM nodes** to keep the browser fast
- WebSocket broadcast with fan-out: multiple browser tabs all receive logs simultaneously
- Fuzzy command matching: if you mistype a built-in command (e.g. `staus` instead of `status`), it suggests the correct one
- Shell commands run asynchronously via `tokio::spawn`, so long-running commands don't block the server

```
┌────────────────────────────────────────────────────┐
│  Console                               [─] [drag]  │
├────────────────────────────────────────────────────┤
│  12:34:01 [info]  Server started on port 7070      │
│  12:34:05 [info]  POST /api/query → 200 (12ms)     │
│  12:35:10 [cmd]   $ status                         │
│  12:35:10 [info]  AIRust v0.1.7                    │
│  12:35:10 [info]  Server: running                  │
│  12:35:10 [info]  CWD: /app                        │
│  12:36:00 [cmd]   $ ls knowledge/                  │
│  12:36:00 [stdout] train.json                      │
├────────────────────────────────────────────────────┤
│  $ _                                               │
└────────────────────────────────────────────────────┘
```

> **Summary:** The web dashboard is a complete control center for your AI: chat with it, manage training data in categories, control bots, review collected data, browse files, and run server commands through the built-in live console — all in the browser.

---

## 9. Bot Ecosystem — Automated Data Collection

AIRust includes a built-in web scraping system to automatically collect training data from websites.

### Workflow

```
1. Create Bot         →  Define URL, crawl config
2. Start Bot Run      →  Scraper collects content
3. Review Raw Data    →  Approve or reject entries
4. Convert to KB      →  Add approved data to training
5. Retrain Agent      →  Agent learns new knowledge
```

### Features

- **Web Crawling**: Configurable depth, URL patterns
- **Deduplication**: Content hashing prevents duplicate entries
- **Manual Review**: Approve/reject workflow ensures data quality
- **Run History**: Track every bot execution with stats
- **Scheduling**: Automated periodic execution

### Data Flow

```
Website → Crawler → Raw Data (pending)
                         │
                    Manual Review
                    ┌────┴────┐
                 Approved   Rejected
                    │
              Add to Knowledge Base
                    │
              Retrain Agent
```

> **Summary:** Bots crawl websites and collect text data automatically. A manual review step ensures quality before the data enters your knowledge base. This creates a self-improving AI pipeline.

---

## 10. CLI — Command Line Interface

### Query Modes

```bash
# Exact matching
airust cli query simple "What is Rust?"

# Fuzzy matching (tolerates typos)
airust cli query fuzzy "Waht is Rsut?"

# Semantic search (best for natural language)
airust cli query tfidf "Tell me about the programming language"
```

### Interactive Mode

```bash
airust cli interactive
```

Opens an interactive REPL where you can:
- Choose your agent type
- Ask questions in real-time
- See confidence scores
- Maintain conversation context

### Knowledge Base Management

```bash
airust cli knowledge
```

Opens a menu for:
- Viewing all entries
- Adding new entries
- Deleting entries
- Saving/loading the knowledge base

### PDF Import

```bash
# Convert PDF to knowledge base
cargo run --bin pdf2kb -- document.pdf

# Merge all knowledge files
cargo run --bin merge_kb
```

> **Summary:** The CLI gives you quick access to all agent types, an interactive chat mode, knowledge management, and PDF conversion — perfect for testing and quick queries without starting the web server.

---

## 11. Using AIRust as a Library

### Basic Example

```rust
use airust::{Agent, TrainableAgent, MatchAgent, KnowledgeBase};

fn main() {
    let kb = KnowledgeBase::from_embedded();
    let mut agent = MatchAgent::new_exact();
    agent.train(kb.get_examples());

    let answer = agent.predict("What is AIRust?");
    println!("{}", String::from(answer));
}
```

### With TF-IDF and Context

```rust
use airust::*;

fn main() {
    let kb = KnowledgeBase::from_embedded();

    // Semantic search agent
    let mut base = TfidfAgent::new().with_bm25_params(1.5, 0.8);
    base.train(kb.get_examples());

    // Wrap with conversation memory (5 turns)
    let mut agent = ContextAgent::new(base, 5)
        .with_context_format(ContextFormat::List);

    let a1 = agent.predict("What is AIRust?");
    println!("A1: {}", String::from(a1.clone()));
    agent.add_context("What is AIRust?".to_string(), a1);

    let a2 = agent.predict("What features does it have?");
    println!("A2: {}", String::from(a2));
}
```

### PDF to Agent Pipeline

```rust
use airust::{PdfLoader, PdfLoaderConfig, TfidfAgent, Agent, TrainableAgent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = PdfLoader::with_config(PdfLoaderConfig {
        min_chunk_size: 100,
        max_chunk_size: 1500,
        chunk_overlap: 250,
        default_weight: 1.2,
        include_metadata: true,
        split_by_sentence: true,
    });

    let kb = loader.pdf_to_knowledge_base("technical-paper.pdf")?;

    let mut agent = TfidfAgent::new();
    agent.train(kb.get_examples());

    let answer = agent.predict("What are the main findings?");
    println!("{}", String::from(answer));

    Ok(())
}
```

### Incremental Training with `append()`

`train()` replaces all data. Use `append()` or `train_single()` to add examples without losing existing data:

```rust
use airust::{Agent, TrainableAgent, TfidfAgent, MatchAgent};

fn main() {
    let mut agent = TfidfAgent::new();

    // Initial training
    agent.train(&[/* ... initial examples ... */]);

    // Add more data later without replacing
    agent.train_single(&example);      // appends one
    agent.append(&[ex1, ex2, ex3]);     // appends many
    agent.add_example("input", "output", 1.0); // convenience
}
```

### Confidence Scores with `ConfidenceAgent`

`TfidfAgent` and `MatchAgent` implement the `ConfidenceAgent` trait for ranked predictions:

```rust
use airust::{Agent, TrainableAgent, TfidfAgent, KnowledgeBase};
use airust::agent::ConfidenceAgent;

fn main() {
    let kb = KnowledgeBase::from_embedded();
    let mut agent = TfidfAgent::new();
    agent.train(kb.get_examples());

    // Get confidence score (0.0 - 1.0)
    let confidence = agent.calculate_confidence("What is Rust?");
    println!("Confidence: {:.2}", confidence);

    // Get top N results ranked by confidence
    let results = agent.predict_top_n("programming language", 3);
    for result in &results {
        println!("{} (confidence: {:.2})", result.response, result.confidence);
    }
}
```

> **Summary:** As a library, AIRust gives you full programmatic control. Create agents, load knowledge, train, and query — all in a few lines of Rust code. Combine agents, process PDFs, and build custom AI applications.

---

## 12. Text Processing Utilities

AIRust provides built-in text processing tools in the `text_utils` module:

```rust
use airust::text_utils;

// Tokenization
let tokens = text_utils::tokenize("Hello, World!");
// → ["hello", "world"]

// Unique terms
let terms = text_utils::unique_terms("the cat and the dog");
// → {"the", "cat", "and", "dog"}

// Stopword removal (supports English and German)
let filtered = text_utils::remove_stopwords(tokens, "en");
// Removes: the, and, is, in, of, to, a, with, for, ...

let filtered_de = text_utils::remove_stopwords(tokens, "de");
// Removes: der, die, das, und, in, ist, von, mit, zu, ...

// String similarity
let dist = text_utils::levenshtein_distance("kitten", "sitting"); // → 3
let sim = text_utils::jaccard_similarity("hello world", "hello earth");

// N-grams
let bigrams = text_utils::create_ngrams("hello world", 2);

// Unicode normalization
let normalized = text_utils::normalize_text("Cafe\u{0301}"); // → "café"
```

> **Summary:** Built-in text utilities handle tokenization, stopword removal (EN/DE), similarity metrics, n-grams, and Unicode normalization — no external NLP libraries needed.

---

## 13. Docker Deployment

### Dockerfile (Multi-Stage Build)

```dockerfile
FROM rust:1.85-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/airust /usr/local/bin/airust
COPY --from=builder /app/knowledge/ ./knowledge/
EXPOSE 7070
CMD ["airust"]
```

### Build & Run

```bash
docker build -t airust .
docker run -p 7070:7070 airust

# With persistent database
docker run -p 7070:7070 -v $(pwd)/airust.db:/app/airust.db airust
```

> **Summary:** Docker provides a clean deployment path: multi-stage build keeps the image small, only the binary and knowledge files are included. Mount a volume for database persistence.

---

## 14. API Reference

### Core Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Web Dashboard (HTML) |
| `GET` | `/api/status` | Server status, agent type, KB size |
| `POST` | `/api/query` | Query the AI agent |
| `POST` | `/api/agent/switch` | Switch agent type |

### Knowledge Base

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/knowledge` | List entries (paginated, searchable) |
| `POST` | `/api/knowledge/add` | Add new entry |
| `DELETE` | `/api/knowledge/:index` | Delete entry |
| `POST` | `/api/knowledge/save` | Save KB to file |
| `POST` | `/api/knowledge/load` | Load KB from file |
| `POST` | `/api/pdf/upload` | Upload & process PDF |
| `POST` | `/api/upload/json` | Upload JSON knowledge |

### Training Data

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/training/categories` | List categories |
| `POST` | `/api/training/categories` | Create category |
| `DELETE` | `/api/training/categories/:id` | Delete category |
| `GET` | `/api/training/data` | List training data |
| `POST` | `/api/training/data` | Add training entry |
| `DELETE` | `/api/training/data/:id` | Delete entry |
| `POST` | `/api/training/import` | Import from JSON |
| `GET` | `/api/training/export` | Export to JSON |

### Chat System

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/chats` | List conversations |
| `POST` | `/api/chats` | Create new chat |
| `GET` | `/api/chats/:id/messages` | Get chat messages |
| `DELETE` | `/api/chats/:id` | Delete chat |
| `POST` | `/api/chats/:id/archive` | Archive chat |

### Bot Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/bots` | List all bots |
| `POST` | `/api/bots` | Create bot |
| `GET` | `/api/bots/:id` | Get bot details |
| `PUT` | `/api/bots/:id` | Update bot |
| `DELETE` | `/api/bots/:id` | Delete bot |
| `POST` | `/api/bots/:id/start` | Start bot run |
| `POST` | `/api/bots/:id/stop` | Stop bot run |
| `GET` | `/api/bots/:id/runs` | Run history |

### Data Review

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/data/pending` | Pending data from bots |
| `POST` | `/api/data/:id/approve` | Approve entry |
| `POST` | `/api/data/:id/reject` | Reject entry |
| `POST` | `/api/data/approve-all` | Batch approve |
| `GET` | `/api/data/approved` | View approved data |
| `POST` | `/api/data/add-to-kb` | Add to knowledge base |

### Vector Database

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/vectors/stats` | Vector DB statistics |
| `POST` | `/api/vectors/rebuild` | Rebuild index |
| `GET` | `/api/vectors/collections` | List collections |
| `POST` | `/api/vectors/collections` | Create collection |
| `DELETE` | `/api/vectors/collections/:id` | Delete collection |
| `GET` | `/api/vectors/entries` | List entries |
| `POST` | `/api/vectors/entries` | Add entry |
| `DELETE` | `/api/vectors/entries/:id` | Delete entry |
| `POST` | `/api/vectors/search` | Similarity search |

### Settings & System

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/settings` | Get settings |
| `POST` | `/api/settings` | Update settings |
| `GET` | `/api/translations/:lang` | Get UI translations |
| `GET` | `/api/files` | List files |
| `GET` | `/api/files/read` | Read file |
| `POST` | `/api/files/write` | Write file |
| `GET` | `/api/files/db/tables` | List SQLite tables |
| `GET` | `/api/files/db/query` | Execute SQL query |
| `WS` | `/ws/console` | Real-time console log |

> **Summary:** Over 50 REST API endpoints give you full control over agents, knowledge, training data, bots, chats, vectors, files, and settings. Plus a WebSocket endpoint for real-time logging.

---

## 15. Configuration & Feature Flags

### Cargo Feature Flags

| Flag | Default | Description |
|------|---------|-------------|
| `colors` | Yes | Colored terminal output |
| `web` | Yes | Web server + SQLite + Bot ecosystem |
| `bots` | Yes (via web) | Web scraping (reqwest, scraper) |
| `async` | Yes (via web) | Async runtime (tokio) |
| `plotting` | No | Data visualization (plotly, plotters) |

```toml
# Minimal (library only, no web server)
airust = { version = "0.1.7", default-features = false }

# Library + colors
airust = { version = "0.1.7", default-features = false, features = ["colors"] }

# Everything including plotting
airust = { version = "0.1.7", features = ["plotting"] }
```

### Runtime Settings (via Web UI or API)

| Setting | Values | Description |
|---------|--------|-------------|
| `theme` | `dark`, `light` | UI theme |
| `language` | `en`, `de`, `tr` | Interface language |
| `accent_color` | hex color | Primary accent color |
| `bg_color` | hex color | Background color |

> **Summary:** Feature flags let you control what gets compiled — from a minimal library to a full web platform. Runtime settings control the UI appearance and language.

---

## 16. Training Data Format

### Modern Format (Recommended)

```json
[
  {
    "input": "What is AIRust?",
    "output": { "Text": "A modular AI library in Rust." },
    "weight": 2.0,
    "metadata": { "source": "manual" }
  },
  {
    "input": "List the features",
    "output": { "Markdown": "- Agents\n- Knowledge Base\n- PDF Processing" },
    "weight": 1.0
  },
  {
    "input": "Get system info",
    "output": { "Json": { "name": "airust", "version": "0.1.7" } },
    "weight": 1.0
  }
]
```

### Legacy Format (Still Supported)

```json
[
  { "input": "What is AIRust?", "output": "A modular AI library in Rust." }
]
```

**Fields:**
- `input` — The question or trigger text
- `output` — The answer, as `Text`, `Markdown`, or `Json`
- `weight` — Importance factor (higher = preferred in ranking, default: 1.0)
- `metadata` — Optional JSON object for source tracking, page numbers, etc.

> **Summary:** Training data is stored as JSON arrays. Each entry has an input (question), output (answer in Text/Markdown/JSON format), an importance weight, and optional metadata. Legacy formats are auto-converted.

---

## 17. Project Structure

```
airust/
├── Cargo.toml                 # Package manifest & dependencies
├── Cargo.lock                 # Dependency lock file
├── build.rs                   # Build script (embeds train.json)
├── Dockerfile                 # Multi-stage Docker build
├── .dockerignore              # Docker excludes
├── README.md                  # This file
├── knowledge/
│   └── train.json             # Embedded training data
├── src/
│   ├── lib.rs                 # Library exports & public API
│   ├── agent.rs               # Core traits & text utilities
│   ├── match_agent.rs         # Exact & fuzzy matching agent
│   ├── tfidf_agent.rs         # BM25 semantic search agent
│   ├── context_agent.rs       # Conversational memory wrapper
│   ├── knowledge.rs           # Knowledge base management
│   ├── pdf_loader.rs          # PDF → training data conversion
│   ├── bin/
│   │   ├── airust.rs          # Main CLI & web server binary
│   │   ├── pdf2kb.rs          # PDF converter CLI tool
│   │   └── merge_kb.rs        # Knowledge base merger tool
│   └── web/
│       ├── mod.rs             # Server initialization & routing
│       ├── state.rs           # Application state & agent wrapper
│       ├── routes.rs          # API endpoint handlers
│       ├── db.rs              # SQLite database layer
│       ├── console.rs         # WebSocket console logging
│       ├── vectordb.rs        # Vector database operations
│       ├── static/
│       │   └── index.html     # Web dashboard (single-page app)
│       └── bots/
│           ├── mod.rs         # Bot module exports
│           ├── models.rs      # Bot data structures
│           ├── db.rs          # Bot database operations
│           ├── crawler.rs     # Web scraping engine
│           ├── processor.rs   # Data processing pipeline
│           ├── scheduler.rs   # Automated execution
│           ├── vectordb.rs    # Vector operations
│           └── routes.rs      # Bot API endpoints
└── airust.db                  # SQLite database (auto-created)
```

> **Summary:** The project is cleanly organized: core AI logic in `src/`, web server in `src/web/`, CLI tools in `src/bin/`, and training data in `knowledge/`. The web dashboard is a single HTML file served directly from memory.

---

## 18. Use Cases & Ideas

- **FAQ Bot** — Train with frequently asked questions, deploy as a web widget
- **Document Search** — Load PDFs, build a searchable knowledge base
- **Customer Support** — Context-aware agent remembers the conversation
- **Internal Wiki Bot** — Scrape your company wiki, auto-build knowledge
- **Developer Documentation Assistant** — Load API docs as PDFs
- **Educational Tool** — Students ask questions about course material
- **IoT Device Assistant** — Minimal binary, runs on embedded systems
- **Privacy-First AI** — No cloud, no data leaving your network
- **Competitive Intelligence** — Bots scrape public sources, review & learn

> **Summary:** AIRust is flexible enough for FAQ bots, document search, customer support, education, IoT, and any scenario where you need a private, trainable AI without cloud dependencies.

---

## 19. Version History

| Version | Highlights |
|---------|-----------|
| **0.1.7** | Real-time WebSocket console, shell command execution, drag-to-resize UI |
| **0.1.6** | PDF processing improvements, web dashboard |
| **0.1.5** | ContextAgent, ResponseFormat, advanced matching, TF-IDF |
| **0.1.4** | TF-IDF/BM25 agent |
| **0.1.3** | English language support |
| **0.1.2** | Initial release |

---

## 20. License

MIT — Free for personal and commercial use.

**Author:** [LEVOGNE](https://github.com/LEVOGNE)
**Repository:** [github.com/LEVOGNE/airust](https://github.com/LEVOGNE/airust)
**Documentation:** [docs.rs/airust](https://docs.rs/airust)

---
---
---

<a name="deutsch"></a>

# Deutsch

## Inhaltsverzeichnis

1. [Was ist AIRust?](#de-1-was-ist-airust)
2. [Funktionen im Ueberblick](#de-2-funktionen-im-ueberblick)
3. [Installation & Einrichtung](#de-3-installation--einrichtung)
4. [Architektur-Uebersicht](#de-4-architektur-uebersicht)
5. [Agenten-Typen — Das Gehirn von AIRust](#de-5-agenten-typen--das-gehirn-von-airust)
6. [Wissensdatenbank — Das Gedaechtnis](#de-6-wissensdatenbank--das-gedaechtnis)
7. [PDF-Verarbeitung — Aus Dokumenten lernen](#de-7-pdf-verarbeitung--aus-dokumenten-lernen)
8. [Web-Dashboard — Die Steuerzentrale](#de-8-web-dashboard--die-steuerzentrale)
9. [Bot-System — Automatische Datensammlung](#de-9-bot-system--automatische-datensammlung)
10. [CLI — Kommandozeile](#de-10-cli--kommandozeile)
11. [AIRust als Bibliothek nutzen](#de-11-airust-als-bibliothek-nutzen)
12. [Textverarbeitung](#de-12-textverarbeitung)
13. [Docker-Deployment](#de-13-docker-deployment)
14. [API-Referenz](#de-14-api-referenz)
15. [Konfiguration & Feature-Flags](#de-15-konfiguration--feature-flags)
16. [Trainingsdaten-Format](#de-16-trainingsdaten-format)
17. [Projektstruktur](#de-17-projektstruktur)
18. [Anwendungsbeispiele](#de-18-anwendungsbeispiele)
19. [Versionshistorie](#de-19-versionshistorie)
20. [Lizenz](#de-20-lizenz)

---

<a name="de-1-was-ist-airust"></a>
## 1. Was ist AIRust?

AIRust ist eine **eigenstaendige KI-Engine**, komplett in Rust geschrieben. Im Gegensatz zu Cloud-basierten KI-Loesungen laeuft AIRust **100% lokal** — kein OpenAI, keine API-Schluessel, kein Internet noetig. Du trainierst es mit deinen eigenen Daten, und es beantwortet Fragen mit Musterabgleich, unscharfer Suche und semantischen Aehnlichkeitsalgorithmen.

Stell es dir so vor: **Dein eigener privater KI-Assistent, den du selbst unterrichtest.**

Es bringt mit:
- Mehrere intelligente Agenten-Typen (exakter Abgleich, unscharfer Abgleich, semantische Suche)
- Ein eingebautes Web-Dashboard mit Chat-Oberflaeche
- PDF-Dokumentenverarbeitung fuer automatische Wissensextraktion
- Web-Scraping-Bots fuer automatisierte Datensammlung
- Eine SQLite-Datenbank fuer persistente Speicherung
- Vollstaendige REST-API fuer Integration mit anderen Systemen

> **Zusammenfassung:** AIRust ist eine lokale, trainierbare KI-Engine in Rust. Du fuetterst sie mit Wissen (Texte, PDFs, Web-Scraping) und sie beantwortet Fragen intelligent — ohne Cloud, ohne API-Schluessel, vollstaendig privat.

---

<a name="de-2-funktionen-im-ueberblick"></a>
## 2. Funktionen im Ueberblick

| Funktion | Beschreibung |
|----------|-------------|
| **4 Agenten-Typen** | Exakt, Unscharf (Fuzzy), TF-IDF/BM25 Semantisch, Kontextbewusst |
| **Wissensdatenbank** | JSON-basiert, zur Kompilierzeit eingebettet, zur Laufzeit erweiterbar |
| **PDF-Verarbeitung** | PDFs in strukturierte Trainingsdaten umwandeln |
| **Web-Dashboard** | Vollstaendige UI mit Chat, Training, Bot-Steuerung, Dateibrowser |
| **Bot-System** | Automatisches Web-Scraping mit Pruefungs-Workflow |
| **Vektor-Datenbank** | Embedding-Speicher und Aehnlichkeitssuche |
| **Chat-Verlauf** | Persistente Gespraeche mit Archivierung |
| **Mehrsprachige UI** | Englisch, Deutsch, Tuerkisch |
| **REST-API** | 50+ Endpunkte fuer volle programmatische Kontrolle |
| **WebSocket-Konsole** | Live-Terminal mit Server-Logs, Shell-Zugriff, eingebauten Befehlen |
| **Docker-Support** | Deployment mit einem Befehl |
| **CLI-Tools** | Interaktiver Modus, Abfrage-Tools, PDF-Konvertierung |

> **Zusammenfassung:** AIRust bietet alles, was du brauchst, um ein KI-System zu bauen, zu trainieren und bereitzustellen: von Agenten und Wissensverwaltung bis hin zur vollstaendigen Web-Oberflaeche und automatischer Datensammlung.

---

<a name="de-3-installation--einrichtung"></a>
## 3. Installation & Einrichtung

### Als Rust-Bibliothek

In deiner `Cargo.toml`:

```toml
[dependencies]
airust = "0.1.7"
```

### Aus dem Quellcode bauen

```bash
git clone https://github.com/LEVOGNE/airust.git
cd airust
cargo build --release
```

### Web-Server starten

```bash
# Standard-Port 7070
cargo run --release

# Eigener Port
cargo run --release -- --port 8080

# Im Hintergrund starten
cargo run --release -- -d

# Landing Page + Dashboard anzeigen (Standard: nur Dashboard)
cargo run --release -- --landingpage

# Hintergrund-Server stoppen
cargo run --release -- stop
```

Dann oeffne `http://localhost:7070` im Browser.

### Mit Docker

```bash
docker build -t airust .
docker run -p 7070:7070 airust
```

> **Zusammenfassung:** Du kannst AIRust als Bibliothek in eigenen Rust-Projekten nutzen, als eigenstaendigen Web-Server starten oder in Docker deployen. Das Web-Dashboard ist standardmaessig auf Port 7070 erreichbar.

---

<a name="de-4-architektur-uebersicht"></a>
## 4. Architektur-Uebersicht

```
                    ┌──────────────────────────────────────┐
                    │           AIRust Engine               │
                    ├──────────┬───────────┬───────────────┤
                    │MatchAgent│TfidfAgent │ ContextAgent  │
                    │(exakt/   │(BM25      │ (wickelt      │
                    │ unscharf)│ semantisch│  jeden Agent)  │
                    ├──────────┴───────────┴───────────────┤
                    │       Wissensdatenbank                │
                    │   (JSON / Eingebettet / Laufzeit)     │
                    ├──────────────────────────────────────┤
                    │        Textverarbeitung               │
                    │  (Tokenisierung, Stoppwoerter, etc.)  │
                    └────────┬──────────┬──────────────────┘
                             │          │
                    ┌────────▼──┐  ┌────▼──────────────┐
                    │  CLI-Tool │  │  Web-Server (Axum) │
                    │ (airust)  │  │  + REST-API        │
                    └───────────┘  │  + WebSocket       │
                                   │  + SQLite-DB       │
                                   │  + Bot-Scheduler   │
                                   └───────────────────┘
```

**Kern-Traits** — Jeder Agent implementiert diese Schnittstellen:

| Trait | Zweck |
|-------|-------|
| `Agent` | Basis: `predict()`, `confidence()`, `can_answer()` |
| `TrainableAgent` | Fuegt `train()`, `add_example()` hinzu |
| `ContextualAgent` | Fuegt `add_context()`, `clear_context()` fuer Gespraechsspeicher hinzu |
| `ConfidenceAgent` | Fuegt `calculate_confidence()`, `predict_top_n()` hinzu |

> **Zusammenfassung:** AIRust hat eine geschichtete Architektur: Agenten denken, die Wissensdatenbank speichert, und der Web-Server stellt die Oberflaeche bereit. Alles kommuniziert ueber saubere Rust-Traits.

---

<a name="de-5-agenten-typen--das-gehirn-von-airust"></a>
## 5. Agenten-Typen — Das Gehirn von AIRust

### 5.1 MatchAgent (Exakt & Unscharf)

Der einfachste und schnellste Agent. Vergleicht deine Frage direkt mit den Trainingsdaten.

**Exakt-Modus** — findet Antworten nur bei genauer Uebereinstimmung (Gross-/Kleinschreibung egal):
```rust
let agent = MatchAgent::new_exact();
```

**Unscharf-Modus** — toleriert Tippfehler mittels Levenshtein-Distanz:
```rust
let agent = MatchAgent::new_fuzzy();

// Mit eigener Toleranz
let agent = MatchAgent::new(MatchingStrategy::Fuzzy(FuzzyOptions {
    max_distance: Some(5),        // Max erlaubte Zeichenaenderungen
    threshold_factor: Some(0.3),  // 30% der Eingabelaenge als Schwelle
}));
```

**Wann nutzen:** FAQ-Bots, Befehlserkennung, strukturierte Frage-Antwort-Systeme.

### 5.2 TfidfAgent (Semantische Suche mit BM25)

Nutzt den BM25-Algorithmus (derselbe wie in Suchmaschinen wie Elasticsearch), um die relevanteste Antwort anhand von Termhaeufigkeit und Dokumentenwichtigkeit zu finden.

```rust
let agent = TfidfAgent::new();

// Algorithmus feintunen
let agent = TfidfAgent::new()
    .with_bm25_params(1.5, 0.8);  // k1 = Term-Skalierung, b = Laengennorm
```

**Wann nutzen:** Dokumentensuche, Wissensdatenbanken mit natuerlichsprachlichen Fragen.

### 5.3 ContextAgent (Gespraechsspeicher)

Wickelt jeden anderen Agenten und fuegt Gespraechsspeicher hinzu. Erinnert sich an die letzten N Austausche, sodass Folgefragen natuerlich funktionieren.

```rust
let base = TfidfAgent::new();
let agent = ContextAgent::new(base, 5)  // 5 Runden merken
    .with_context_format(ContextFormat::List);
```

**Kontext-Formate:**
| Format | Beispiel |
|--------|---------|
| `QAPairs` | `Q: Was ist Rust? A: Eine Programmiersprache. Q: ...` |
| `List` | `[Was ist Rust? -> Eine Programmiersprache, ...]` |
| `Sentence` | `Vorherige Fragen: Was ist Rust? - Eine Programmiersprache; ...` |
| `Custom` | Eigene Formatierungsfunktion |

**Wann nutzen:** Chatbots, interaktive Assistenten, Folgefragen.

> **Zusammenfassung:** AIRust bietet drei Agenten-Typen: MatchAgent fuer schnellen exakten/unscharfen Abgleich, TfidfAgent fuer intelligente semantische Suche, und ContextAgent fuer Gespraechsspeicher. Waehle je nach Anwendungsfall oder kombiniere sie.

---

<a name="de-6-wissensdatenbank--das-gedaechtnis"></a>
## 6. Wissensdatenbank — Das Gedaechtnis

Die Wissensdatenbank ist der Ort, an dem alle Trainingsdaten liegen. Sie unterstuetzt zwei Modi:

### Kompilierzeit-Einbettung

Daten aus `knowledge/train.json` werden beim Build in die Binaerdatei eingebettet:
```rust
let kb = KnowledgeBase::from_embedded();
```

### Laufzeit-Verwaltung

```rust
let mut kb = KnowledgeBase::new();
kb.add_example("Was ist Rust?", "Eine System-Programmiersprache", 1.0);
kb.save(Some("knowledge/train.json".into()))?;
let kb = KnowledgeBase::load("knowledge/custom.json".into())?;
kb.merge(&other_kb);
```

### Datenformat

```json
[
  {
    "input": "Was ist AIRust?",
    "output": { "Text": "Eine modulare KI-Bibliothek in Rust." },
    "weight": 2.0,
    "metadata": { "quelle": "manuell" }
  }
]
```

> **Zusammenfassung:** Die Wissensdatenbank speichert alles, was die KI weiss. Daten koennen zur Kompilierzeit eingebettet oder dynamisch zur Laufzeit verwaltet werden. Eintraege haben Gewichte (Wichtigkeit) und optionale Metadaten.

---

<a name="de-7-pdf-verarbeitung--aus-dokumenten-lernen"></a>
## 7. PDF-Verarbeitung — Aus Dokumenten lernen

AIRust kann automatisch Wissen aus PDF-Dokumenten extrahieren. Es teilt Text in intelligente Abschnitte und erstellt Trainingsbeispiele.

### Kommandozeilen-Tool

```bash
# Einfache Konvertierung
cargo run --bin pdf2kb -- dokument.pdf

# Eigener Ausgabepfad
cargo run --bin pdf2kb -- dokument.pdf ausgabe/meine_kb.json

# Volle Konfiguration
cargo run --bin pdf2kb -- dokument.pdf \
  --min-chunk 100 \
  --max-chunk 2000 \
  --overlap 300 \
  --weight 1.5
```

### Im Code

```rust
let config = PdfLoaderConfig {
    min_chunk_size: 100,     // Minimale Zeichen pro Abschnitt
    max_chunk_size: 1500,    // Maximale Zeichen pro Abschnitt
    chunk_overlap: 250,      // Ueberlappung fuer Kontext
    default_weight: 1.2,     // Trainingsgewicht
    include_metadata: true,  // Seitennummern einbeziehen
    split_by_sentence: true, // Satzgrenzen beachten
};

let loader = PdfLoader::with_config(config);
let kb = loader.pdf_to_knowledge_base("forschungsarbeit.pdf")?;
```

### Mehrere Quellen zusammenfuehren

```bash
cargo run --bin merge_kb
```

> **Zusammenfassung:** Fuettere PDFs in AIRust und es erstellt automatisch strukturierte Trainingsdaten. Intelligentes Chunking beachtet Satzgrenzen und erhaelt Kontext durch ueberlappende Segmente.

---

<a name="de-8-web-dashboard--die-steuerzentrale"></a>
## 8. Web-Dashboard — Die Steuerzentrale

Starte den Server mit `cargo run` und oeffne `http://localhost:7070`.

### Dashboard-Tabs

| Tab | Was es tut |
|-----|-----------|
| **Chat** | Mit dem KI-Agenten sprechen, Konfidenz-Scores sehen |
| **Training** | Trainingsdaten mit Kategorien verwalten, JSON importieren/exportieren |
| **Knowledge** | Wissensdatenbank durchsuchen, hinzufuegen, loeschen |
| **Bots** | Web-Scraping-Bots erstellen und verwalten |
| **Data Review** | Vom Bot gesammelte Daten genehmigen/ablehnen |
| **Vectors** | Vektor-Sammlungen und Embeddings verwalten |
| **Files** | Projektdateien und SQLite-Datenbank durchsuchen |
| **Console** | Echtzeit-WebSocket-Log-Viewer mit Shell-Zugriff |
| **Settings** | Theme (dunkel/hell), Sprache (EN/DE/TR), Akzentfarben |

### Smarte Einstellungen via Chat

Du kannst Einstellungen aendern, indem du einfach schreibst:

- *"Mach die Seite dunkel"* — wechselt zum Dark Theme
- *"Aendere zu gruenem Hintergrund"* — aendert Akzentfarbe
- *"Stelle auf Deutsch"* — aendert UI-Sprache

### Konsole — Echtzeit-Server-Terminal

Das Konsolen-Panel befindet sich am unteren Rand des Dashboards und funktioniert als **Live-Terminal**, das ueber WebSocket (`/ws/console`) mit deinem AIRust-Server verbunden ist.

**Was es kann:**
- Streamt jeden Server-Log (Anfragen, Fehler, Agenten-Aktivitaet) in Echtzeit in den Browser
- Du kannst Befehle direkt eingeben — sowohl eingebaute Befehle als auch beliebige Shell-Befehle
- Farbcodierte Ausgabe: `info` (blau), `warn` (gelb), `error` (rot), `cmd` (gruen), `stdout`/`stderr`

**Eingebaute Befehle:**

| Befehl | Was er tut |
|--------|-----------|
| `help` oder `?` | Zeigt Liste der verfuegbaren Befehle |
| `clear` | Loescht die gesamte Konsolen-Ausgabe |
| `status` | Zeigt AIRust-Version, Server-Status und Arbeitsverzeichnis |
| `stop` | Faehrt den Server kontrolliert herunter |
| `restart` | Startet den Server-Prozess neu |
| *alles andere* | Wird als Shell-Befehl ausgefuehrt (z.B. `ls`, `df -h`, `cat knowledge/train.json`) |

**UI-Funktionen:**

| Funktion | Wie es funktioniert |
|----------|-------------------|
| **Groesse aendern** | Konsolen-Kopfzeile nach oben/unten ziehen |
| **Minimieren/Erweitern** | Klick auf den Toggle-Button in der Kopfzeile |
| **Befehlsverlauf** | Pfeiltasten (hoch/runter) zum Durchblaettern vorheriger Befehle |
| **Auto-Reconnect** | Bei WebSocket-Trennung wird automatisch alle 2 Sekunden neu verbunden |
| **Verbindungsanzeige** | Gruener Punkt = verbunden, roter Punkt = getrennt |

**Technische Details:**
- Server haelt einen Ringpuffer von **500 Log-Eintraegen** — bei Verbindung erhaeltst du die gesamte Historie
- Client begrenzt auf **1000 DOM-Knoten**, damit der Browser schnell bleibt
- WebSocket-Broadcast mit Fan-out: mehrere Browser-Tabs empfangen Logs gleichzeitig
- Unscharfe Befehlserkennung: bei Tippfehlern (z.B. `staus` statt `status`) wird der korrekte Befehl vorgeschlagen
- Shell-Befehle laufen asynchron ueber `tokio::spawn` — lang laufende Befehle blockieren den Server nicht

```
┌────────────────────────────────────────────────────┐
│  Konsole                               [─] [drag]  │
├────────────────────────────────────────────────────┤
│  12:34:01 [info]  Server gestartet auf Port 7070   │
│  12:34:05 [info]  POST /api/query → 200 (12ms)     │
│  12:35:10 [cmd]   $ status                         │
│  12:35:10 [info]  AIRust v0.1.7                    │
│  12:35:10 [info]  Server: running                  │
│  12:36:00 [cmd]   $ ls knowledge/                  │
│  12:36:00 [stdout] train.json                      │
├────────────────────────────────────────────────────┤
│  $ _                                               │
└────────────────────────────────────────────────────┘
```

> **Zusammenfassung:** Das Web-Dashboard ist eine vollstaendige Steuerzentrale fuer deine KI: Chatten, Trainingsdaten verwalten, Bots steuern, gesammelte Daten pruefen, Dateien durchsuchen und Server-Befehle ueber die eingebaute Live-Konsole ausfuehren — alles im Browser.

---

<a name="de-9-bot-system--automatische-datensammlung"></a>
## 9. Bot-System — Automatische Datensammlung

AIRust enthaelt ein eingebautes Web-Scraping-System zur automatischen Sammlung von Trainingsdaten.

### Ablauf

```
1. Bot erstellen        →  URL und Konfiguration definieren
2. Bot-Lauf starten     →  Scraper sammelt Inhalte
3. Rohdaten pruefen     →  Eintraege genehmigen oder ablehnen
4. In KB konvertieren   →  Genehmigte Daten zum Training hinzufuegen
5. Agent neu trainieren →  Agent lernt neues Wissen
```

### Funktionen

- **Web-Crawling**: Konfigurierbare Tiefe und URL-Muster
- **Deduplizierung**: Content-Hashing verhindert Duplikate
- **Manuelle Pruefung**: Genehmigungs-Workflow sichert Datenqualitaet
- **Lauf-Historie**: Jede Bot-Ausfuehrung wird mit Statistiken verfolgt
- **Zeitplanung**: Automatische periodische Ausfuehrung

> **Zusammenfassung:** Bots crawlen Websites und sammeln Textdaten automatisch. Ein manueller Pruefschritt sichert die Qualitaet, bevor die Daten in die Wissensdatenbank aufgenommen werden.

---

<a name="de-10-cli--kommandozeile"></a>
## 10. CLI — Kommandozeile

### Abfrage-Modi

```bash
# Exakter Abgleich
airust cli query simple "Was ist Rust?"

# Unscharfer Abgleich (toleriert Tippfehler)
airust cli query fuzzy "Was ist Rsut?"

# Semantische Suche (am besten fuer natuerliche Sprache)
airust cli query tfidf "Erklaere mir die Programmiersprache"
```

### Interaktiver Modus

```bash
airust cli interactive
```

Oeffnet eine interaktive Sitzung mit Agenten-Auswahl und Echtzeit-Antworten.

### Wissensdatenbank-Verwaltung

```bash
airust cli knowledge
```

> **Zusammenfassung:** Die CLI bietet schnellen Zugriff auf alle Agenten-Typen, einen interaktiven Chat-Modus, Wissensverwaltung und PDF-Konvertierung — perfekt zum Testen ohne Web-Server.

---

<a name="de-11-airust-als-bibliothek-nutzen"></a>
## 11. AIRust als Bibliothek nutzen

### Einfaches Beispiel

```rust
use airust::{Agent, TrainableAgent, MatchAgent, KnowledgeBase};

fn main() {
    let kb = KnowledgeBase::from_embedded();
    let mut agent = MatchAgent::new_exact();
    agent.train(kb.get_examples());

    let antwort = agent.predict("Was ist AIRust?");
    println!("{}", String::from(antwort));
}
```

### Mit TF-IDF und Kontext

```rust
use airust::*;

fn main() {
    let kb = KnowledgeBase::from_embedded();
    let mut base = TfidfAgent::new().with_bm25_params(1.5, 0.8);
    base.train(kb.get_examples());

    let mut agent = ContextAgent::new(base, 5)
        .with_context_format(ContextFormat::List);

    let a1 = agent.predict("Was ist AIRust?");
    agent.add_context("Was ist AIRust?".to_string(), a1.clone());

    let a2 = agent.predict("Welche Funktionen hat es?");
    println!("{}", String::from(a2));
}
```

> **Zusammenfassung:** Als Bibliothek gibt dir AIRust volle programmatische Kontrolle. Agenten erstellen, Wissen laden, trainieren und abfragen — alles in wenigen Zeilen Rust-Code.

---

<a name="de-12-textverarbeitung"></a>
## 12. Textverarbeitung

```rust
use airust::text_utils;

// Tokenisierung
let tokens = text_utils::tokenize("Hallo, Welt!");

// Stoppwort-Entfernung (Deutsch unterstuetzt)
let gefiltert = text_utils::remove_stopwords(tokens, "de");
// Entfernt: der, die, das, und, in, ist, von, mit, zu, ...

// Zeichenkettenaehnlichkeit
let dist = text_utils::levenshtein_distance("Katze", "Kaetze");
let sim = text_utils::jaccard_similarity("hallo welt", "hallo erde");

// N-Gramme
let bigramme = text_utils::create_ngrams("hallo welt", 2);

// Unicode-Normalisierung
let normalisiert = text_utils::normalize_text("Cafe\u{0301}");
```

> **Zusammenfassung:** Eingebaute Text-Werkzeuge handhaben Tokenisierung, Stoppwort-Entfernung (EN/DE), Aehnlichkeitsmetriken, N-Gramme und Unicode-Normalisierung — ohne externe NLP-Bibliotheken.

---

<a name="de-13-docker-deployment"></a>
## 13. Docker-Deployment

```bash
docker build -t airust .
docker run -p 7070:7070 airust

# Mit persistenter Datenbank
docker run -p 7070:7070 -v $(pwd)/airust.db:/app/airust.db airust
```

> **Zusammenfassung:** Docker ermoeglicht ein sauberes Deployment: mehrstufiger Build haelt das Image klein. Ein Volume fuer die Datenbank-Persistenz mounten.

---

<a name="de-14-api-referenz"></a>
## 14. API-Referenz

Die vollstaendige API-Referenz findest du in der [englischen Sektion](#14-api-reference). Alle Endpunkte sind identisch — ueber 50 REST-Endpunkte fuer Agenten, Wissen, Training, Bots, Chats, Vektoren, Dateien und Einstellungen.

> **Zusammenfassung:** Ueber 50 REST-API-Endpunkte geben dir volle Kontrolle ueber das gesamte System. Dazu kommt ein WebSocket-Endpunkt fuer Echtzeit-Logging.

---

<a name="de-15-konfiguration--feature-flags"></a>
## 15. Konfiguration & Feature-Flags

| Flag | Standard | Beschreibung |
|------|----------|-------------|
| `colors` | Ja | Farbige Terminal-Ausgabe |
| `web` | Ja | Web-Server + SQLite + Bot-System |
| `bots` | Ja (ueber web) | Web-Scraping |
| `async` | Ja (ueber web) | Async-Laufzeit (tokio) |
| `plotting` | Nein | Datenvisualisierung |

```toml
# Minimal (nur Bibliothek)
airust = { version = "0.1.7", default-features = false }

# Alles mit Plotting
airust = { version = "0.1.7", features = ["plotting"] }
```

> **Zusammenfassung:** Feature-Flags kontrollieren, was kompiliert wird — von einer minimalen Bibliothek bis zur vollstaendigen Web-Plattform.

---

<a name="de-16-trainingsdaten-format"></a>
## 16. Trainingsdaten-Format

Identisch mit dem [englischen Abschnitt](#16-training-data-format). Unterstuetzt `Text`, `Markdown`, `Json` als Ausgabeformate. Gewichte und Metadaten sind optional. Legacy-Formate werden automatisch konvertiert.

> **Zusammenfassung:** Trainingsdaten werden als JSON-Arrays gespeichert. Jeder Eintrag hat eine Eingabe (Frage), Ausgabe (Antwort), ein Gewicht und optionale Metadaten.

---

<a name="de-17-projektstruktur"></a>
## 17. Projektstruktur

Siehe [englische Sektion](#17-project-structure) fuer den vollstaendigen Verzeichnisbaum.

> **Zusammenfassung:** Das Projekt ist sauber organisiert: KI-Kernlogik in `src/`, Web-Server in `src/web/`, CLI-Tools in `src/bin/`, Trainingsdaten in `knowledge/`.

---

<a name="de-18-anwendungsbeispiele"></a>
## 18. Anwendungsbeispiele

- **FAQ-Bot** — Mit haeufig gestellten Fragen trainieren, als Web-Widget deployen
- **Dokumentensuche** — PDFs laden, durchsuchbare Wissensdatenbank aufbauen
- **Kundensupport** — Kontextbewusster Agent erinnert sich an das Gespraech
- **Internes Wiki** — Firmen-Wiki automatisch scrapen und Wissen aufbauen
- **Entwickler-Dokumentation** — API-Docs als PDFs laden
- **Lernwerkzeug** — Schueler stellen Fragen zu Kursmaterial
- **IoT-Assistent** — Minimale Binaerdatei, laeuft auf Embedded-Systemen
- **Datenschutz-KI** — Keine Cloud, keine Daten verlassen dein Netzwerk

> **Zusammenfassung:** AIRust ist flexibel genug fuer FAQ-Bots, Dokumentensuche, Kundensupport, Bildung, IoT und jedes Szenario, in dem du eine private, trainierbare KI ohne Cloud-Abhaengigkeiten brauchst.

---

<a name="de-19-versionshistorie"></a>
## 19. Versionshistorie

| Version | Neuerungen |
|---------|-----------|
| **0.1.7** | Echtzeit-WebSocket-Konsole, Shell-Befehlsausfuehrung, Drag-to-Resize UI |
| **0.1.6** | PDF-Verarbeitung verbessert, Web-Dashboard |
| **0.1.5** | ContextAgent, ResponseFormat, erweitertes Matching, TF-IDF |
| **0.1.4** | TF-IDF/BM25-Agent |
| **0.1.3** | Englische Sprachunterstuetzung |
| **0.1.2** | Erstveroeffentlichung |

---

<a name="de-20-lizenz"></a>
## 20. Lizenz

MIT — Frei fuer private und kommerzielle Nutzung.

**Autor:** [LEVOGNE](https://github.com/LEVOGNE)

---
---
---

<a name="turkce"></a>

# Turkce

## Icindekiler

1. [AIRust nedir?](#tr-1-airust-nedir)
2. [Ozellikler](#tr-2-ozellikler)
3. [Kurulum](#tr-3-kurulum)
4. [Mimari Genel Bakis](#tr-4-mimari-genel-bakis)
5. [Ajan Turleri — AIRust'in Beyni](#tr-5-ajan-turleri--airustin-beyni)
6. [Bilgi Tabani — Hafiza](#tr-6-bilgi-tabani--hafiza)
7. [PDF Isleme — Belgelerden Ogrenme](#tr-7-pdf-isleme--belgelerden-ogrenme)
8. [Web Paneli — Kontrol Merkezi](#tr-8-web-paneli--kontrol-merkezi)
9. [Bot Sistemi — Otomatik Veri Toplama](#tr-9-bot-sistemi--otomatik-veri-toplama)
10. [CLI — Komut Satiri](#tr-10-cli--komut-satiri)
11. [AIRust'i Kutuphane Olarak Kullanma](#tr-11-airrusti-kutuphane-olarak-kullanma)
12. [Metin Isleme](#tr-12-metin-isleme)
13. [Docker ile Dagitim](#tr-13-docker-ile-dagitim)
14. [API Referansi](#tr-14-api-referansi)
15. [Yapilandirma & Ozellik Bayraklari](#tr-15-yapilandirma--ozellik-bayraklari)
16. [Egitim Verisi Formati](#tr-16-egitim-verisi-formati)
17. [Proje Yapisi](#tr-17-proje-yapisi)
18. [Kullanim Senaryolari](#tr-18-kullanim-senaryolari)
19. [Surum Gecmisi](#tr-19-surum-gecmisi)
20. [Lisans](#tr-20-lisans)

---

<a name="tr-1-airust-nedir"></a>
## 1. AIRust nedir?

AIRust, tamamen Rust ile yazilmis **bagimsiz bir yapay zeka motorudur**. Bulut tabanli yapay zeka cozumlerinin aksine, AIRust **%100 yerel** calisir — OpenAI yok, API anahtari yok, internet gerekli degil. Kendi verilerinle egitirsin ve desen eslestirme, bulanik arama ve anlamsal benzerlik algoritmalari kullanarak sorulari yanitlar.

Sunu dusun: **Kendin egittigin, kendi ozel yapay zeka asistanin.**

Icerdikleri:
- Birden fazla akilli ajan turu (tam eslestirme, bulanik eslestirme, anlamsal arama)
- Sohbet arayuzlu yerlesik web paneli
- Otomatik bilgi cikarimi icin PDF belge isleme
- Otomatik veri toplama icin web kazima botlari
- Kalici depolama icin SQLite veritabani
- Diger sistemlerle entegrasyon icin tam REST API

> **Ozet:** AIRust, Rust ile yazilmis yerel, egitilebilir bir yapay zeka motorudur. Bilgi beslersin (metin, PDF, web kazima) ve sorulari akilli bir sekilde yanitlar — bulut yok, API anahtari yok, tamamen gizli.

---

<a name="tr-2-ozellikler"></a>
## 2. Ozellikler

| Ozellik | Aciklama |
|---------|----------|
| **4 Ajan Turu** | Tam Eslestirme, Bulanik, TF-IDF/BM25 Anlamsal, Baglamsal |
| **Bilgi Tabani** | JSON tabanli, derleme zamaninda gomulu, calisma zamaninda genisletilebilir |
| **PDF Isleme** | PDF'leri yapilandirilmis egitim verisine donusturme |
| **Web Paneli** | Sohbet, egitim yoneticisi, bot kontrolu, dosya gezgini |
| **Bot Sistemi** | Inceleme is akisiyla otomatik web kazima |
| **Vektor Veritabani** | Gomme depolama ve benzerlik arama |
| **Sohbet Gecmisi** | Arsivleme ile kalici konusmalar |
| **Cok Dilli Arayuz** | Ingilizce, Almanca, Turkce |
| **REST API** | Tam programatik kontrol icin 50'den fazla ucnokta |
| **WebSocket Konsol** | Sunucu gunlukleri, kabuk erisimi ve yerlesik komutlarla canli terminal |
| **Docker Destegi** | Tek komutla dagitim |
| **CLI Araclari** | Etkilesimli mod, sorgulama, PDF donusturme |

> **Ozet:** AIRust, bir yapay zeka sistemi olusturmak, egitmek ve dagitmak icin ihtiyaciniz olan her seyi saglar: ajanlar ve bilgi yonetiminden tam web arayuzu ve otomatik veri toplamaya kadar.

---

<a name="tr-3-kurulum"></a>
## 3. Kurulum

### Rust Kutuphanesi Olarak

`Cargo.toml` dosyaniza ekleyin:

```toml
[dependencies]
airust = "0.1.7"
```

### Kaynaktan Derleme

```bash
git clone https://github.com/LEVOGNE/airust.git
cd airust
cargo build --release
```

### Web Sunucuyu Baslatma

```bash
# Varsayilan port 7070
cargo run --release

# Ozel port
cargo run --release -- --port 8080

# Arka planda calistirma
cargo run --release -- -d

# Karsilama sayfasi + pano goster (varsayilan: sadece pano)
cargo run --release -- --landingpage

# Arka plan sunucusunu durdurma
cargo run --release -- stop
```

Ardindan tarayicinizda `http://localhost:7070` adresini acin.

### Docker ile

```bash
docker build -t airust .
docker run -p 7070:7070 airust
```

> **Ozet:** AIRust'i kendi Rust projelerinizde kutuphane olarak kullanabilir, bagimsiz web sunucusu olarak calistirabilir veya Docker'da dagitabilirsiniz. Web paneli varsayilan olarak 7070 portunda erisilebildir.

---

<a name="tr-4-mimari-genel-bakis"></a>
## 4. Mimari Genel Bakis

```
                    ┌──────────────────────────────────────┐
                    │           AIRust Motoru                │
                    ├──────────┬───────────┬───────────────┤
                    │MatchAgent│TfidfAgent │ ContextAgent  │
                    │(tam/     │(BM25      │ (herhangi bir │
                    │ bulanik) │ anlamsal) │  ajani sarar) │
                    ├──────────┴───────────┴───────────────┤
                    │         Bilgi Tabani                  │
                    │   (JSON / Gomulu / Calisma Zamani)    │
                    ├──────────────────────────────────────┤
                    │        Metin Isleme                   │
                    │ (tokenizasyon, durma sozcukleri, vb.) │
                    └────────┬──────────┬──────────────────┘
                             │          │
                    ┌────────▼──┐  ┌────▼──────────────┐
                    │  CLI Araci│  │  Web Sunucu (Axum) │
                    │ (airust)  │  │  + REST API        │
                    └───────────┘  │  + WebSocket       │
                                   │  + SQLite DB       │
                                   │  + Bot Zamanlayici  │
                                   └───────────────────┘
```

**Temel Trait'ler** — Her ajan bu arayuzleri uygular:

| Trait | Amac |
|-------|------|
| `Agent` | Temel: `predict()`, `confidence()`, `can_answer()` |
| `TrainableAgent` | `train()`, `add_example()` ekler |
| `ContextualAgent` | Konusma hafizasi icin `add_context()`, `clear_context()` ekler |
| `ConfidenceAgent` | `calculate_confidence()`, `predict_top_n()` ekler |

> **Ozet:** AIRust katmanli bir mimariye sahiptir: ajanlar dusunur, bilgi tabani verileri depolar ve web sunucu arayuzu saglar. Her sey temiz Rust trait'leri uzerinden iletisim kurar.

---

<a name="tr-5-ajan-turleri--airustin-beyni"></a>
## 5. Ajan Turleri — AIRust'in Beyni

### 5.1 MatchAgent (Tam & Bulanik)

En basit ve en hizli ajan. Sorunuzu dogrudan egitim verileriyle karsilastirir.

**Tam Mod** — sadece soru tam eslesmediginde yanitlar (buyuk/kucuk harf onemli degil):
```rust
let agent = MatchAgent::new_exact();
```

**Bulanik Mod** — Levenshtein mesafesi kullanarak yazim hatalarini tolere eder:
```rust
let agent = MatchAgent::new_fuzzy();

// Ozel tolerans ile
let agent = MatchAgent::new(MatchingStrategy::Fuzzy(FuzzyOptions {
    max_distance: Some(5),        // Izin verilen maks karakter degisikligi
    threshold_factor: Some(0.3),  // Giris uzunlugunun %30'u esik olarak
}));
```

**Ne zaman kullanilir:** SSS botlari, komut tanima, yapilandirilmis soru-cevap sistemleri.

### 5.2 TfidfAgent (BM25 ile Anlamsal Arama)

Terim sikligi ve belge onemine dayali en alakali yanitlari bulmak icin BM25 algortimasini (Elasticsearch gibi arama motorlarinda kullanilan ayni algoritma) kullanir.

```rust
let agent = TfidfAgent::new();

// Algortimayi ince ayarlama
let agent = TfidfAgent::new()
    .with_bm25_params(1.5, 0.8);  // k1 = terim olcekleme, b = uzunluk normalizasyonu
```

**Ne zaman kullanilir:** Belge arama, dogal dil sorulari olan bilgi tabanlari.

### 5.3 ContextAgent (Konusma Hafizasi)

Herhangi bir ajani sarar ve konusma hafizasi ekler. Son N konusmayi hatirlayarak takip sorularinin dogal bir sekilde calismalisini saglar.

```rust
let base = TfidfAgent::new();
let agent = ContextAgent::new(base, 5)  // 5 tur hatirla
    .with_context_format(ContextFormat::List);
```

**Baglam Formatlari:**
| Format | Ornek |
|--------|-------|
| `QAPairs` | `S: Rust nedir? C: Bir programlama dili. S: ...` |
| `List` | `[Rust nedir? -> Bir programlama dili, ...]` |
| `Sentence` | `Onceki sorular: Rust nedir? - Bir programlama dili; ...` |
| `Custom` | Kendi formatlama fonksiyonunuz |

**Ne zaman kullanilir:** Sohbet botlari, etkilesimli asistanlar, takip sorulari.

> **Ozet:** AIRust uc ajan turu sunar: hizli tam/bulanik eslestirme icin MatchAgent, akilli anlamsal arama icin TfidfAgent ve konusma hafizasi icin ContextAgent. Kullanim durumunuza gore secin veya birlestirin.

---

<a name="tr-6-bilgi-tabani--hafiza"></a>
## 6. Bilgi Tabani — Hafiza

Bilgi Tabani tum egitim verilerinin tutuldugu yerdir. Iki mod destekler:

### Derleme Zamani Gomme

`knowledge/train.json` dosyasindaki veriler derleme sirasinda ikili dosyaya gomulur:
```rust
let kb = KnowledgeBase::from_embedded();
```

### Calisma Zamani Yonetimi

```rust
let mut kb = KnowledgeBase::new();
kb.add_example("Rust nedir?", "Bir sistem programlama dili", 1.0);
kb.save(Some("knowledge/train.json".into()))?;
let kb = KnowledgeBase::load("knowledge/custom.json".into())?;
kb.merge(&other_kb);
```

> **Ozet:** Bilgi Tabani, yapay zekanin bildigi her seyi depolar. Veriler derleme zamaninda gomulubilir veya calisma zamaninda dinamik olarak yonetilebilir. Girisler agirliklara ve opsiyonel meta verilere sahiptir.

---

<a name="tr-7-pdf-isleme--belgelerden-ogrenme"></a>
## 7. PDF Isleme — Belgelerden Ogrenme

AIRust, PDF belgelerinden otomatik olarak bilgi cikarabilir.

### Komut Satiri Araci

```bash
# Temel donusturme
cargo run --bin pdf2kb -- belge.pdf

# Ozel cikis yolu
cargo run --bin pdf2kb -- belge.pdf cikis/benim_kb.json

# Tam yapilandirma
cargo run --bin pdf2kb -- belge.pdf \
  --min-chunk 100 \
  --max-chunk 2000 \
  --overlap 300 \
  --weight 1.5
```

### Kodda

```rust
let config = PdfLoaderConfig {
    min_chunk_size: 100,     // Parca basina minimum karakter
    max_chunk_size: 1500,    // Parca basina maksimum karakter
    chunk_overlap: 250,      // Baglam icin cakisma
    default_weight: 1.2,     // Egitim agirligi
    include_metadata: true,  // Sayfa numaralari dahil
    split_by_sentence: true, // Cumle sinirlarini dikkate al
};

let loader = PdfLoader::with_config(config);
let kb = loader.pdf_to_knowledge_base("arastirma-makalesi.pdf")?;
```

### Birden Fazla Kaynak Birlestirme

```bash
cargo run --bin merge_kb
```

> **Ozet:** PDF'leri AIRust'a besleyin ve otomatik olarak yapilandirilmis egitim verileri olusturur. Akilli parcalama cumle sinirlarini dikkate alir ve cakisan segmentler araciligiyla baglami korur.

---

<a name="tr-8-web-paneli--kontrol-merkezi"></a>
## 8. Web Paneli — Kontrol Merkezi

Sunucuyu `cargo run` ile baslatin ve `http://localhost:7070` adresini acin.

### Panel Sekmeleri

| Sekme | Ne yapar |
|-------|---------|
| **Chat** | Yapay zeka ajaninizla sohbet edin, guven puanlarini gorun |
| **Training** | Kategorilerle egitim verilerini yonetin, JSON iceri/disa aktar |
| **Knowledge** | Bilgi tabani girislerini arayin, ekleyin, silin |
| **Bots** | Web kazima botlari olusturun ve yonetin |
| **Data Review** | Botlarin topladigi verileri onaylayin/reddedin |
| **Vectors** | Vektor koleksiyonlari ve gommeleri yonetin |
| **Files** | Proje dosyalarini ve SQLite veritabanini gezin |
| **Console** | Kabuk erisimli gercek zamanli WebSocket gunluk goruntuleyici |
| **Settings** | Tema (karanlik/aydinlik), dil (EN/DE/TR), vurgu renkleri |

### Sohbet ile Akilli Ayarlar

Ayarlari dogal bir sekilde yazarak degistirebilirsiniz:

- *"Sayfayi karanlik yap"* — karanlik temaya gecer
- *"Turkce yap"* — arayuz dilini degistirir

### Konsol — Gercek Zamanli Sunucu Terminali

Konsol paneli, pano altinda yer alir ve WebSocket (`/ws/console`) uzerinden AIRust sunucunuza bagli bir **canli terminal** olarak calisir.

**Ne yapar:**
- Her sunucu gunlugunu (istekler, hatalar, ajan etkinligi) gercek zamanli olarak tarayiciniza aktarir
- Dogrudan komut yazabilirsiniz — hem yerlesik komutlar hem de rastgele kabuk komutlari
- Renk kodlu cikti: `info` (mavi), `warn` (sari), `error` (kirmizi), `cmd` (yesil), `stdout`/`stderr`

**Yerlesik Komutlar:**

| Komut | Ne yapar |
|-------|---------|
| `help` veya `?` | Kullanilabilir komutlarin listesini goster |
| `clear` | Tum konsol ciktisini temizle |
| `status` | AIRust surumu, sunucu durumu ve calisma dizinini goster |
| `stop` | Sunucuyu duzgun bir sekilde kapat |
| `restart` | Sunucu islemini yeniden baslat |
| *diger her sey* | Kabuk komutu olarak calistirilir (orn. `ls`, `df -h`, `cat knowledge/train.json`) |

**Arayuz Ozellikleri:**

| Ozellik | Nasil calisir |
|---------|--------------|
| **Boyut degistirme** | Konsol baslik cubugunu yukari/asagi surukleyin |
| **Kucultme/Genisletme** | Baslikdaki degistirme dugmesine tiklayin |
| **Komut gecmisi** | Ok tuslari (yukari/asagi) ile onceki komutlar arasinda gezin |
| **Otomatik yeniden baglanma** | WebSocket baglantisi kesilirse her 2 saniyede otomatik yeniden baglanir |
| **Baglanti gostergesi** | Yesil nokta = bagli, kirmizi nokta = bagli degil |

**Teknik Detaylar:**
- Sunucu **500 gunluk girislik** bir halka tamponu tutar — baglandiginizda tum gecmisi alirsiniz
- Istemci, tarayiciyi hizli tutmak icin **1000 DOM dugumune** sinirlandirilmistir
- WebSocket yayini: birden fazla tarayici sekmesi ayni anda gunlukleri alir
- Bulanik komut eslestirme: yerlesik bir komutu yanlis yazarsaniz (orn. `staus` yerine `status`), dogru komutu onerir
- Kabuk komutlari `tokio::spawn` ile asenkron calisir — uzun sureli komutlar sunucuyu engellemez

```
┌────────────────────────────────────────────────────┐
│  Konsol                                [─] [drag]   │
├────────────────────────────────────────────────────┤
│  12:34:01 [info]  Sunucu 7070 portunda baslatildi  │
│  12:34:05 [info]  POST /api/query → 200 (12ms)     │
│  12:35:10 [cmd]   $ status                         │
│  12:35:10 [info]  AIRust v0.1.7                    │
│  12:35:10 [info]  Server: running                  │
│  12:36:00 [cmd]   $ ls knowledge/                  │
│  12:36:00 [stdout] train.json                      │
├────────────────────────────────────────────────────┤
│  $ _                                               │
└────────────────────────────────────────────────────┘
```

> **Ozet:** Web paneli, yapay zekaniz icin eksiksiz bir kontrol merkezidir: sohbet edin, egitim verilerini yonetin, botlari kontrol edin, toplanan verileri inceleyin, dosyalari gezin ve yerlesik canli konsol uzerinden sunucu komutlari calistirin — hepsi tarayicida.

---

<a name="tr-9-bot-sistemi--otomatik-veri-toplama"></a>
## 9. Bot Sistemi — Otomatik Veri Toplama

AIRust, web sitelerinden otomatik olarak egitim verisi toplamak icin yerlesik bir web kazima sistemi icerir.

### Is Akisi

```
1. Bot olustur          →  URL ve yapilandirma tanimla
2. Bot calistir         →  Kaziyici icerik toplar
3. Ham verileri incele  →  Girisleri onayla veya reddet
4. KB'ye donustur       →  Onaylanan verileri egitime ekle
5. Ajani yeniden egit   →  Ajan yeni bilgi ogrenir
```

### Ozellikler

- **Web Tarama**: Yapilandirilabilir derinlik ve URL kaliplari
- **Tekrar Onleme**: Icerik karma (hash) islemleri tekrarlari engeller
- **Manuel Inceleme**: Onay is akisi veri kalitesini saglar
- **Calistirma Gecmisi**: Her bot calistirmasini istatistiklerle izleyin
- **Zamanlama**: Otomatik periyodik calistirma

> **Ozet:** Botlar web sitelerini tarar ve metin verilerini otomatik olarak toplar. Manuel inceleme adimi, veriler bilgi tabaniniza girmeden once kaliteyi saglar.

---

<a name="tr-10-cli--komut-satiri"></a>
## 10. CLI — Komut Satiri

### Sorgu Modlari

```bash
# Tam eslestirme
airust cli query simple "Rust nedir?"

# Bulanik eslestirme (yazim hatalarini tolere eder)
airust cli query fuzzy "Rsut nedri?"

# Anlamsal arama (dogal dil icin en iyisi)
airust cli query tfidf "Programlama dilini anlat"
```

### Etkilesimli Mod

```bash
airust cli interactive
```

Ajan secimi ve gercek zamanli yanitlarla etkilesimli bir oturum acar.

### Bilgi Tabani Yonetimi

```bash
airust cli knowledge
```

> **Ozet:** CLI, web sunucusu baslatmadan tum ajan turlerine, etkilesimli sohbet moduna, bilgi yonetimine ve PDF donusturmeye hizli erisim saglar.

---

<a name="tr-11-airrusti-kutuphane-olarak-kullanma"></a>
## 11. AIRust'i Kutuphane Olarak Kullanma

### Temel Ornek

```rust
use airust::{Agent, TrainableAgent, MatchAgent, KnowledgeBase};

fn main() {
    let kb = KnowledgeBase::from_embedded();
    let mut agent = MatchAgent::new_exact();
    agent.train(kb.get_examples());

    let yanit = agent.predict("AIRust nedir?");
    println!("{}", String::from(yanit));
}
```

### TF-IDF ve Baglam ile

```rust
use airust::*;

fn main() {
    let kb = KnowledgeBase::from_embedded();
    let mut base = TfidfAgent::new().with_bm25_params(1.5, 0.8);
    base.train(kb.get_examples());

    let mut agent = ContextAgent::new(base, 5)
        .with_context_format(ContextFormat::List);

    let a1 = agent.predict("AIRust nedir?");
    agent.add_context("AIRust nedir?".to_string(), a1.clone());

    let a2 = agent.predict("Hangi ozellikleri var?");
    println!("{}", String::from(a2));
}
```

> **Ozet:** Kutuphane olarak AIRust size tam programatik kontrol verir. Ajan olusturun, bilgi yukleyin, egitin ve sorgulayun — birkac satir Rust koduyla.

---

<a name="tr-12-metin-isleme"></a>
## 12. Metin Isleme

```rust
use airust::text_utils;

// Tokenizasyon
let tokenlar = text_utils::tokenize("Merhaba, Dunya!");

// Durma sozcugu kaldirma (Ingilizce ve Almanca desteklenir)
let filtrelenmis = text_utils::remove_stopwords(tokenlar, "en");

// Karakter dizisi benzerligi
let mesafe = text_utils::levenshtein_distance("kedi", "kedl");
let benzerlik = text_utils::jaccard_similarity("merhaba dunya", "merhaba mars");

// N-gramlar
let bigramlar = text_utils::create_ngrams("merhaba dunya", 2);

// Unicode normalizasyonu
let normallesmis = text_utils::normalize_text("Cafe\u{0301}");
```

> **Ozet:** Yerlesik metin araclari tokenizasyon, durma sozcugu kaldirma (EN/DE), benzerlik metrikleri, n-gramlar ve Unicode normalizasyonu islemlerini gerceklestirir — harici NLP kutuphanelerine gerek yok.

---

<a name="tr-13-docker-ile-dagitim"></a>
## 13. Docker ile Dagitim

```bash
docker build -t airust .
docker run -p 7070:7070 airust

# Kalici veritabani ile
docker run -p 7070:7070 -v $(pwd)/airust.db:/app/airust.db airust
```

> **Ozet:** Docker temiz bir dagitim yolu saglar: cok asamali derleme imaji kucuk tutar. Veritabani kaliciligi icin bir birim baglayun.

---

<a name="tr-14-api-referansi"></a>
## 14. API Referansi

Tam API referansi icin [Ingilizce bolume](#14-api-reference) bakin. Tum ucnoktalar aynidir — ajanlar, bilgi, egitim, botlar, sohbetler, vektorler, dosyalar ve ayarlar icin 50'den fazla REST ucnoktasi.

> **Ozet:** 50'den fazla REST API ucnoktasi tum sistem uzerinde tam kontrol saglar. Ayrica gercek zamanli gunluk icin bir WebSocket ucnoktasi da vardir.

---

<a name="tr-15-yapilandirma--ozellik-bayraklari"></a>
## 15. Yapilandirma & Ozellik Bayraklari

| Bayrak | Varsayilan | Aciklama |
|--------|-----------|----------|
| `colors` | Evet | Renkli terminal ciktisi |
| `web` | Evet | Web sunucu + SQLite + Bot sistemi |
| `bots` | Evet (web ile) | Web kazima |
| `async` | Evet (web ile) | Asenkron calisma zamani (tokio) |
| `plotting` | Hayir | Veri gorsellestirme |

```toml
# Minimal (sadece kutuphane)
airust = { version = "0.1.7", default-features = false }

# Grafik dahil her sey
airust = { version = "0.1.7", features = ["plotting"] }
```

> **Ozet:** Ozellik bayraklari neyin derlenegecini kontrol eder — minimal bir kutuphaneden tam bir web platformuna kadar.

---

<a name="tr-16-egitim-verisi-formati"></a>
## 16. Egitim Verisi Formati

[Ingilizce bolumle](#16-training-data-format) aynidir. Cikis formatlari olarak `Text`, `Markdown`, `Json` destekler. Agirliklar ve meta veriler opsiyoneldir. Eski formatlar otomatik olarak donusturulur.

> **Ozet:** Egitim verileri JSON dizileri olarak depolanir. Her giris bir soru, yanit, agirlik ve opsiyonel meta verilere sahiptir.

---

<a name="tr-17-proje-yapisi"></a>
## 17. Proje Yapisi

Tam dizin agaci icin [Ingilizce bolume](#17-project-structure) bakin.

> **Ozet:** Proje temiz bir sekilde organize edilmistir: temel yapay zeka mantigi `src/` icinde, web sunucu `src/web/` icinde, CLI araclari `src/bin/` icinde, egitim verileri `knowledge/` icinde.

---

<a name="tr-18-kullanim-senaryolari"></a>
## 18. Kullanim Senaryolari

- **SSS Botu** — Sik sorulan sorularla egitin, web widget'i olarak dagitin
- **Belge Arama** — PDF'leri yukleyin, aranabilir bilgi tabani olusturun
- **Musteri Destegi** — Baglamsal ajan konusmayi hatirlar
- **Dahili Wiki Botu** — Sirket wiki'sini otomatik kaziyip bilgi olusturun
- **Gelistirici Dokumantasyon Asistani** — API belgelerini PDF olarak yukleyin
- **Egitim Araci** — Ogrenciler ders materyali hakkinda soru sorar
- **IoT Cihaz Asistani** — Minimal ikili dosya, gomulu sistemlerde calisir
- **Gizlilik Oncelikli Yapay Zeka** — Bulut yok, veri aginizi terk etmez

> **Ozet:** AIRust; SSS botlari, belge arama, musteri destegi, egitim, IoT ve bulut bagimliligiolmadan ozel, egitilebilir bir yapay zekaya ihtiyac duydugunuz her senaryo icin yeterince esnektir.

---

<a name="tr-19-surum-gecmisi"></a>
## 19. Surum Gecmisi

| Surum | Yenilikler |
|-------|-----------|
| **0.1.7** | Gercek zamanli WebSocket konsol, kabuk komut calistirma, surukle-boyutlandir UI |
| **0.1.6** | PDF isleme iyilestirmeleri, web paneli |
| **0.1.5** | ContextAgent, ResponseFormat, gelismis eslestirme, TF-IDF |
| **0.1.4** | TF-IDF/BM25 ajani |
| **0.1.3** | Ingilizce dil destegi |
| **0.1.2** | Ilk yayin |

---

<a name="tr-20-lisans"></a>
## 20. Lisans

MIT — Kisisel ve ticari kullanim icin ucretsiz.

**Yazar:** [LEVOGNE](https://github.com/LEVOGNE)
**Depo:** [github.com/LEVOGNE/airust](https://github.com/LEVOGNE/airust)
**Dokumantasyon:** [docs.rs/airust](https://docs.rs/airust)

---

<p align="center">
  Built with love in Rust.<br>
  Contributions and extensions are welcome!
</p>
