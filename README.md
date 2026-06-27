# NICLEUS + SOMA/VAILAT

**Filtro semantico locale per modelli AI — meno consumo, più precisione, zero cloud.**

---

## Cos'è NICLEUS

NICLEUS è un layer semantico scritto in Rust che si posiziona tra l'utente e il modello linguistico locale (via Ollama). Non modifica i pesi del modello — lavora sul contesto, prima che la richiesta arrivi al modello.

Come un direttore d'orchestra che decide quali strumenti suonare prima che la musica inizi, NICLEUS analizza l'intento dell'utente, comprime il segnale e passa al modello solo l'essenziale.

**Risultati misurati su hardware standard (Intel i5, 16GB RAM):**

| Modello | Riduzione token in ingresso | Note |
|---|---|---|
| 3b locale | fino al 70% | risposta più coerente e focalizzata |
| 7b locale | fino al 90% | qualità comparabile a modelli più grandi |
| Modelli grandi (API) | fino al 90% | risparmio diretto in euro su ogni chiamata |

**Benefici concreti:**
- 📉 Riduzione dei consumi energetici e dei costi API
- 🎯 Risposte più pertinenti grazie all'analisi dell'intento
- 🛡 Protezione da prompt malevoli e tentativi di jailbreak
- 🌍 Meno allucinazioni — il modello riceve segnali puliti
- 🔒 Tutto locale — nessun dato inviato a server esterni

---

## SOMA / VAILAT — il simbionte

SOMA è il loop simbionte integrato in NICLEUS. Genera situazioni in italiano partendo da un tema-seme dell'utente, le processa attraverso il filtro semantico e salva le coppie input/output come dataset JSON strutturato.

SOMA non tocca i pesi del modello. Lavora esclusivamente sul contesto.

Il dataset cresce solo su comando esplicito dell'utente — mai in automatico.

### Il Metodo Vailat

SOMA/VAILAT prende il nome da **Giovanni Vailati** (1863–1909), logico e filosofo pragmatista italiano. Vailati sosteneva che il linguaggio dovesse essere uno strumento di precisione — non ornamento, ma macchina per pensare.

> *Il metodo Vailat non è solo una tecnica di programmazione, ma una macchina per pensare. Trasformando il linguaggio in uno strumento di precisione, NICLEUS permette ai loop vitali di agire con la massima economia del pensiero — realizzando l'ideale vailatiano della scienza come strumento di adattamento e previsione.*

---

## Requisiti

- [Rust](https://rustup.rs/) — `cargo` disponibile nel PATH
- [Ollama](https://ollama.com/) in esecuzione su `localhost:11434`
- Un modello scaricato — consigliato `qwen2.5:7b`

```bash
ollama pull qwen2.5:7b
```

---

## Installazione

```bash
git clone https://github.com/tuousername/nicleus.git
cd nicleus
cargo run
```

Ollama deve girare in background. Se non parte automaticamente:

```bash
ollama serve   # in un terminale separato
```

---

## Utilizzo

### Prompt normale

```
> domanda qualsiasi
> domanda con web -w
> domanda approfondita -r
> domanda strutturata -e
```

### Comandi disponibili

| Comando | Effetto | Quando usarlo |
|---|---|---|
| `-w` | Cerca online prima di rispondere | meteo, fatti recenti |
| `-r` | Risposta libera e approfondita (cap 800 token) | concetti tecnici, analisi lunghe |
| `-e` | Risposta strutturata punto per punto | confronti, elenchi, procedure |
| `continua` | Richiama il contesto degli ultimi 3 scambi | approfondire un argomento |

### Modalità SOMA/VAILAT

```
> soma
🌱 seme> il tuo tema
```

| Comando SOMA | Effetto |
|---|---|
| `a` | Risposta intuitiva, anche satirica |
| `b` | Risposta breve — non entra nel buffer |
| `e` | Analisi strutturata punto per punto |
| `w` | Cerca online prima di rispondere |
| `s` | Salva lo scambio nel dataset JSON |
| `r` | Risposta lunga stile ricerca |
| `q` | Esci da SOMA |

Il dataset viene salvato in `soma_dataset.json`.  
Le sessioni vengono archiviate automaticamente in `sessioni/` alla chiusura.

---

## Personalizzazione dell'intento

I comandi di NICLEUS non sono shortcut tecnici — sono un **alfabeto dell'intento**.

L'utente che impara a usare `-r` per le domande profonde e `-e` per quelle strutturate sta imparando a comunicare con l'AI in modo consapevole. Questo è il principio Montessori applicato all'interazione uomo-macchina: non addestrare il modello, preparare l'ambiente.

---

## Struttura del progetto

```
nicleus/
├── src/
│   └── main.rs          # tutto il codice — NICLEUS + SOMA/VAILAT
├── soma_dataset.json    # dataset generato (creato al primo `s`)
├── sessioni/            # archivio sessioni JSON (creato automaticamente)
├── .nicleus_history     # history dei comandi (rustyline)
└── Cargo.toml
```

---

## Licenza

MIT per uso personale e open source.  
Per uso commerciale e integrazioni enterprise: contattare l'autore.

---

*NICLEUS è un progetto indipendente — costruito su hardware normale, per persone reali.*
