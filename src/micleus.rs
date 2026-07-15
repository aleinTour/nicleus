// ═══════════════════════════════════════════════════════
//  MICLEUS — Gateway Client | NICLEUS v2.5
//  Il cranio che sceglie il modello giusto.
//  NICLEUS è il cuore, MICLEUS è la testa.
// ═══════════════════════════════════════════════════════

/// Backend disponibili
#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    OllamaLocale,   // qwen2.5:7b su localhost — default
    // FuturoRemoto, // placeholder per API esterne
}

/// Configurazione del modello scelto
#[derive(Debug, Clone)]
pub struct ConfigModello {
    pub backend:   Backend,
    pub modello:   String,
    pub endpoint:  String,
}

impl ConfigModello {
    /// Ollama locale — configurazione di default
    pub fn ollama_locale() -> Self {
        Self {
            backend:  Backend::OllamaLocale,
            modello:  "qwen2.5:7b".to_string(),
            endpoint: "127.0.0.1:11434".to_string(),
        }
    }
}

/// MICLEUS — cuore del routing
/// Riceve i segnali da NICLEUS e sceglie il backend
pub fn scegli_modello(
    soma_mode: bool,
    r_mode:    bool,
    web_mode:  bool,
) -> ConfigModello {
    // Per ora tutto va su Ollama locale.
    // Qui in futuro: se soma_mode → modello creativo
    //                se r_mode    → modello con ctx lungo
    //                se web_mode  → modello con tool use
    let _ = (soma_mode, r_mode, web_mode); // silenzio warning
    ConfigModello::ollama_locale()
}
