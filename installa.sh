#!/bin/bash
# ═══════════════════════════════════════════════════════
#  NICLEUS — script di installazione automatica
#  Testato su: Ubuntu, Pop!_OS, Debian, Linux Mint
#  Cosa fa:
#    1. Installa Rust se non presente
#    2. Installa Ollama se non presente
#    3. Scarica il modello qwen2.5:7b
#    4. Compila NICLEUS
#    5. Lancia NICLEUS
# ═══════════════════════════════════════════════════════

set -e  # ferma lo script se un comando fallisce

echo "═══════════════════════════════════════"
echo "  NICLEUS — Installazione automatica   "
echo "═══════════════════════════════════════"

# ── 1. Rust ─────────────────────────────────────────────
# 'command -v' controlla se un comando esiste nel PATH
if ! command -v cargo &> /dev/null; then
    echo "▶ Installo Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # Carica le variabili d'ambiente di Rust nella sessione corrente
    source "$HOME/.cargo/env"
    echo "✓ Rust installato"
else
    echo "✓ Rust già presente ($(cargo --version))"
fi

# ── 2. Ollama ───────────────────────────────────────────
if ! command -v ollama &> /dev/null; then
    echo "▶ Installo Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
    echo "✓ Ollama installato"
else
    echo "✓ Ollama già presente"
fi

# ── 3. Avvia Ollama in background se non gira già ───────
if ! curl -s http://localhost:11434 &> /dev/null; then
    echo "▶ Avvio Ollama in background..."
    ollama serve &> /tmp/ollama.log &
    # Aspetta che Ollama sia pronto (max 15 secondi)
    for i in {1..15}; do
        sleep 1
        if curl -s http://localhost:11434 &> /dev/null; then
            echo "✓ Ollama in ascolto"
            break
        fi
    done
else
    echo "✓ Ollama già in esecuzione"
fi

# ── 4. Modello qwen2.5:7b ───────────────────────────────
# 'ollama list' mostra i modelli scaricati
if ! ollama list 2>/dev/null | grep -q "qwen2.5:7b"; then
    echo "▶ Scarico qwen2.5:7b (~4.7GB, potrebbero volerci alcuni minuti)..."
    ollama pull qwen2.5:7b
    echo "✓ Modello scaricato"
else
    echo "✓ qwen2.5:7b già presente"
fi

# ── 5. Compila NICLEUS ──────────────────────────────────
echo "▶ Compilo NICLEUS..."
# '--release' compila con ottimizzazioni — più veloce in esecuzione
cargo build --release
echo "✓ Compilato"

# ── 6. Lancia ───────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════"
echo "  Tutto pronto — avvio NICLEUS...      "
echo "═══════════════════════════════════════"
echo ""
cargo run --release
