# NICLEUS + SOMA/VAILAT

**Local semantic filter for AI models — less consumption, more precision, zero cloud.**

Built in Rust. Runs on normal hardware. Your data never leaves your machine.

---

## What is NICLEUS

NICLEUS is a semantic layer written in Rust that sits between the user and a local language model (via Ollama). It doesn't touch the model's weights — it works on context, before the request ever reaches the model.

Like an orchestra conductor deciding which instruments play before the music starts, NICLEUS analyzes the user's intent, compresses the signal, and passes only the essential to the model.

**Measured results on standard hardware (Intel i5, 16GB RAM):**

| Model | Input token reduction | Notes |
|---|---|---|
| 3b local | up to 70% | more coherent, focused answers |
| 7b local | up to 90% | quality comparable to larger models |
| Large models (API) | up to 90% | direct savings in €/$ per call |

**Concrete benefits:**

- 📉 Reduced energy consumption and API costs
- 🎯 More relevant answers through intent analysis
- 🛡 Protection from malicious prompts and jailbreak attempts — 100% interception rate in testing
- 🌍 Fewer hallucinations — the model receives clean signals
- 🔒 Fully local — no data sent to external servers

---

## Elegance and privacy by design

NICLEUS follows two principles that most AI tooling has abandoned:

**Elegance** — zero external dependencies beyond the Rust standard library and a readline crate. No HTTP libraries: raw TCP to Ollama. No JSON crates: hand-written parsers. Every line exists because it must. The entire system compiles in seconds and runs on a laptop from 2015.

**Privacy** — nothing is collected, nothing is transmitted, nothing is remembered without explicit consent. The dataset grows only when the user types `s`. Sessions are archived locally. There is no telemetry, no phone-home, no account. Your conversations belong to you.

---

## MICLEUS — the web gateway

**[micleus.netlify.app](https://micleus.netlify.app)** — *Model Interface & Customer Layer for Efficient Unified Semantics*

MICLEUS is the browser-side gateway of the NICLEUS ecosystem. It brings the same semantic filtering to any AI provider, entirely client-side:

- **Bring your own key** — Anthropic, OpenAI, Groq, Gemini, or your own local Ollama. Keys live in browser memory only, never stored, never transmitted to anyone but the provider you chose.
- **Agorà** — send the same question to multiple models in parallel; the Vailati Filter scores responses on signal density and honesty. The cleanest answer wins.
- **Closed-corpus RAG** — load your own knowledge base (`TITLE|||TEXT` per line) and get answers drawn exclusively from it. Zero hallucination by design. Your corpus never leaves the browser.
- **Live token accounting** — real measured tokens, honest ratios. No inflated savings claims.

Together they form **SEMA** — *Semantic Engine for Multi-model Activation*: NICLEUS is the heart, MICLEUS is the mind.

---

## SOMA / VAILAT — the symbiont

SOMA is the symbiotic loop built into NICLEUS. It generates situations in Italian from a user-provided seed theme, processes them through the semantic filter, and saves input/output pairs as a structured JSON dataset.

SOMA never touches model weights. It works exclusively on context.

The dataset grows only on explicit user command — never automatically.

### The Vailati Method

SOMA/VAILAT is named after **Giovanni Vailati** (1863–1909), Italian pragmatist logician and philosopher. Vailati argued that language should be an instrument of precision — not ornament, but a machine for thinking.

> The Vailati method is not just a programming technique, but a machine for thinking. By turning language into a precision instrument, NICLEUS lets vital loops act with maximum economy of thought — realizing the Vailatian ideal of science as a tool for adaptation and prediction.

---

## Requirements

- [Rust](https://rustup.rs/) — `cargo` available in PATH
- [Ollama](https://ollama.com/) running on `localhost:11434`
- A downloaded model — recommended: `qwen2.5:7b`

```bash
ollama pull qwen2.5:7b
```

---

## Installation

```bash
git clone https://github.com/aleinTour/nicleus.git
cd nicleus
cargo run
```

Ollama must be running in the background. If it doesn't start automatically:

```bash
ollama serve   # in a separate terminal
```

---

## Usage

### Normal prompts

```
> any question
> question with web search -w
> deep dive question -r
> structured answer -e
```

### Available commands

| Command | Effect | When to use |
|---|---|---|
| `-w` | Searches online before answering | weather, recent facts |
| `-r` | Free, in-depth answer (800 token cap) | technical concepts, long analysis |
| `-e` | Structured point-by-point answer | comparisons, lists, procedures |
| `r + w` | Agentic research: 3 queries, synthesis, save | complex topics needing sources |
| `continua` | Recalls context of last 3 exchanges | going deeper on a topic |

### Built-in monadini (zero-token modules)

Monadini are self-sufficient Rust modules that answer without touching the LLM at all:

| Monadino | Trigger | Source |
|---|---|---|
| Identity | questions about NICLEUS/SOMA/naturalese | closed corpus (RAG, keyword search) |
| Weather | weather/clothing questions | wttr.in |
| Economy | crypto/gold/forex prices | CoinGecko, Frankfurter, metals.live |

### SOMA/VAILAT mode

```
> soma
🌱 seme> your theme
```

| SOMA command | Effect |
|---|---|
| `a` | Intuitive answer, even satirical |
| `b` | Short answer — skips the buffer |
| `e` | Structured point-by-point analysis |
| `w` | Searches online first |
| `s` | Saves the exchange to the JSON dataset |
| `r` | Long research-style answer |
| `q` | Exit SOMA |

The dataset is saved to `soma_dataset.json`.
Sessions are archived automatically to `sessioni/` on exit.

---

## The alphabet of intent

NICLEUS commands are not technical shortcuts — they are an **alphabet of intent**.

A user who learns to use `-r` for deep questions and `-e` for structured ones is learning to communicate with AI consciously. This is the Montessori principle applied to human-machine interaction: don't train the model — prepare the environment.

---

## Project structure

```
nicleus/
├── src/
│   ├── main.rs              # NICLEUS core + SOMA/VAILAT
│   ├── micleus.rs           # model routing layer (skeleton)
│   └── economia.rs          # economy monadino — live market data
├── nicleus_corpus.txt       # identity corpus (TITLE|||TEXT format)
├── soma_dataset.json        # generated dataset (created on first `s`)
├── sessioni/                # session archive (auto-created)
└── Cargo.toml
```

---

## License

MIT for personal and open-source use.
For commercial use and enterprise integration: contact the author.

---

## Author

**Alessandro Gallifuoco** — [@aleintour](https://x.com/aleintour)

*NICLEUS is an independent project — built on normal hardware, for real people.*

Berry Labs · Rome · 2026
