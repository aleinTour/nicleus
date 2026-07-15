// ECONOMIA.RS — Monadino Finanziario NICLEUS
use std::process::Command;
pub struct RispostaEconomia {
    pub testo: String,
    pub serve_modello: bool,
}
pub fn è_query_finanziaria(input: &str) -> bool {
    let t = input.to_lowercase();
    let keywords = ["bitcoin","btc","ethereum","eth","crypto","oro","gold","dollaro","euro","eur","usd","forex","borsa","sp500","petrolio","oil","prezzo","quotazione","mercato","investimento","trading"];
    keywords.iter().any(|&k| t.contains(k))
}
fn identifica_asset(input: &str) -> Vec<&'static str> {
    let t = input.to_lowercase();
    let mut assets = Vec::new();
    if t.contains("bitcoin") || t.contains("btc") { assets.push("bitcoin"); }
    if t.contains("ethereum") || t.contains("eth") { assets.push("ethereum"); }
    if t.contains("oro") || t.contains("gold") { assets.push("gold"); }
    if t.contains("forex") || t.contains("euro") || t.contains("dollaro") { assets.push("forex"); }
    if t.contains("petrolio") || t.contains("oil") { assets.push("oil"); }
    if assets.is_empty() { assets = vec!["bitcoin","gold","forex"]; }
    assets
}
fn fetch_crypto(coin: &str) -> Option<String> {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd,eur&include_24hr_change=true", coin);
    let output = Command::new("curl").args(["-s","--max-time","8",&url]).output().ok()?;
    let json = String::from_utf8_lossy(&output.stdout);
    if json.is_empty() || json.contains("error") { return None; }
    let estrai = |campo: &str| -> Option<f64> {
        let cerca = format!("\"{}\":", campo);
        let pos = json.find(&cerca)?;
        let dopo = json[pos + cerca.len()..].trim_start();
        let fine = dopo.find(|c: char| c == ',' || c == '}').unwrap_or(dopo.len());
        dopo[..fine].trim().parse().ok()
    };
    let usd = estrai("usd")?;
    let eur = estrai("eur").unwrap_or(usd * 0.92);
    let chg = estrai("usd_24h_change").unwrap_or(0.0);
    let segno = if chg >= 0.0 { "▲" } else { "▼" };
    Some(format!("{}: ${:.0} (€{:.0}) {} {:.2}% 24h", coin.to_uppercase(), usd, eur, segno, chg.abs()))
}
fn fetch_forex() -> Option<String> {
    let output = Command::new("curl").args(["-s","--max-time","8","https://api.frankfurter.app/latest?from=EUR&to=USD,GBP,JPY"]).output().ok()?;
    let json = String::from_utf8_lossy(&output.stdout);
    if json.is_empty() || json.contains("error") { return None; }
    let estrai_rate = |valuta: &str| -> Option<f64> {
        let cerca = format!("\"{}\":", valuta);
        let pos = json.find(&cerca)?;
        let dopo = json[pos + cerca.len()..].trim_start();
        let fine = dopo.find(|c: char| c == ',' || c == '}').unwrap_or(dopo.len());
        dopo[..fine].trim().parse().ok()
    };
    let usd = estrai_rate("USD")?;
    let gbp = estrai_rate("GBP").unwrap_or(0.0);
    let jpy = estrai_rate("JPY").unwrap_or(0.0);
    Some(format!("EUR/USD: {:.4} | EUR/GBP: {:.4} | EUR/JPY: {:.2}", usd, gbp, jpy))
}
fn fetch_oro() -> Option<String> {
    let output = Command::new("curl").args(["-s","--max-time","8","https://metals.live/api/v1/spot"]).output().ok()?;
    let json = String::from_utf8_lossy(&output.stdout);
    if json.is_empty() || json.contains("error") { return None; }
    let cerca_gold = "\"gold\":";
    if let Some(pos) = json.find(cerca_gold) {
        let dopo = json[pos + cerca_gold.len()..].trim_start();
        let fine = dopo.find(|c: char| c == ',' || c == '}').unwrap_or(dopo.len());
        if let Ok(price) = dopo[..fine].trim().parse::<f64>() {
            return Some(format!("ORO (XAU): ${:.2}/oz", price));
        }
    }
    None
}
pub fn interroga(input: &str) -> RispostaEconomia {
    let assets = identifica_asset(input);
    let t = input.to_lowercase();
    let vuole_interpretazione = ["cosa pensi","conviene","dovrei","consiglia","previsione","andrà","futuro","analisi","cosa farà","perché","come mai","spiegami"].iter().any(|&k| t.contains(k));
    let mut dati = Vec::new();
    for asset in &assets {
        match *asset {
            "bitcoin" => { if let Some(d) = fetch_crypto("bitcoin") { dati.push(d); } }
            "ethereum" => { if let Some(d) = fetch_crypto("ethereum") { dati.push(d); } }
            "gold" => { if let Some(d) = fetch_oro() { dati.push(d); } }
            "forex" => { if let Some(d) = fetch_forex() { dati.push(d); } }
            "oil" => { dati.push("Petrolio: usa -w per dati live.".to_string()); }
            _ => {}
        }
    }
    let testo = if dati.is_empty() { "Dati di mercato non disponibili al momento.".to_string() } else { dati.join(" | ") };
    RispostaEconomia { testo, serve_modello: vuole_interpretazione }
}
