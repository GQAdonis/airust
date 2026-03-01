# AIRust — Startup Guide

## Quick Start

```bash
# Build
cargo build --release --features web

# Start (Foreground, Dashboard only)
cargo run --features web

# With landing page
cargo run --features web -- --landingpage

# Start (Background)
cargo run --features web -- -d

# Stop
cargo run --features web -- stop
```

Server: `http://localhost:7070`
Console WebSocket: `ws://localhost:7071`

---

## 1. Web Dashboard starten

```bash
airust                      # Port 7070 (Default), nur Dashboard
airust --port 8080          # Custom Port
airust -d                   # Im Hintergrund starten
airust --landingpage        # Landing Page + Dashboard
airust stop                 # Stoppen
```

Browser: `http://localhost:7070`

### Was passiert beim Start?

1. **Datenbank** wird geoeffnet (`airust.db`) — wird automatisch erstellt falls nicht vorhanden
2. **4 Agenten** werden erstellt: TF-IDF, Exact, Fuzzy, Context
3. **Knowledge Base** wird geladen (eingebettete + DB-Trainingsdaten)
4. **Alle Agenten** werden mit den KB-Daten trainiert
5. **Web-Server** startet auf Port 7070
6. **Console WebSocket** startet auf Port 7071
7. **Default-Agent** ist TF-IDF (kann im Dashboard geaendert werden)

### Datenfluss

```
knowledge/train.json (Compile-Zeit, initial leer)
         +
  SQLite training_data (Runtime-Daten)
         |
         v
   KnowledgeBase (merged)
         |
         v
  Alle 4 Agenten trainiert
         |
         v
  Query → aktive Agenten → beste Antwort
```

---

## 2. Chat — Fragen stellen

### Schritt fuer Schritt

1. Dashboard oeffnen: `http://localhost:7070`
2. Tab **Chat** ist beim Start aktiv
3. Klicke **+** (oben links) um einen neuen Chat zu erstellen
4. Tippe eine Frage ins Eingabefeld unten
5. Druecke **Enter** oder klicke **Send**
6. Die Antwort erscheint mit einem **Confidence-Score** (0-100%)

### Was passiert im Hintergrund?

1. Deine Frage wird an `/api/query` gesendet
2. Alle **aktiven Agenten** werden parallel befragt
3. Jeder Agent berechnet eine Antwort + Confidence
4. Die Antwort mit der **hoechsten Confidence** gewinnt
5. Falls der Context-Agent aktiv ist, wird die Frage+Antwort im Kontext gespeichert
6. Frage und Antwort werden in der **SQLite-DB** gespeichert (Chat-History)

### Chat-Verwaltung

- **Archivieren**: Chat-Eintrag nach links swipen oder Archive-Button klicken
- **Loeschen**: X-Button am Chat-Eintrag
- **Archiv anzeigen/verbergen**: Toggle-Button in der Chat-Liste
- **Automatischer Titel**: Wird aus den ersten 50 Zeichen der ersten Nachricht generiert

### Confidence verstehen

| Bereich | Bedeutung |
|---------|-----------|
| 80-100% | Sehr sichere Antwort, gute Uebereinstimmung |
| 50-80% | Teilweise passend, moeglicherweise ungenaue Antwort |
| 0-50% | Niedrige Sicherheit, Agent hat keine gute Uebereinstimmung gefunden |

---

## 3. Agenten auswaehlen

### Wo?

In **Settings** (Zahnrad-Icon in der Sidebar):

### Die 4 Agenten im Detail

| Agent | Typ | Wie er funktioniert | Wann verwenden? |
|-------|-----|---------------------|-----------------|
| **Smart Search** | TF-IDF/BM25 | Zerlegt Frage in Woerter, berechnet Gewichtung nach Haeufigkeit und Seltenheit, rankt nach BM25-Score | Standard-Agent. Beste Wahl fuer natuerliche Sprache |
| **Word-for-Word** | Exact Match | Vergleicht die Eingabe exakt (case-insensitive) mit allen Trainingsfragen | Wenn exakte Kommandos oder Keywords erwartet werden |
| **Close Enough** | Fuzzy/Levenshtein | Berechnet Levenshtein-Distanz zu jeder Trainingsfrage, toleriert Tippfehler | Wenn User oft Tippfehler machen |
| **Remembers Chat** | Context + TF-IDF | Haengt vorherige Fragen+Antworten an die aktuelle Frage, nutzt TF-IDF darunter | Fuer Gespraeche mit Bezug auf vorherige Nachrichten |

### Multi-Agent Modus

**So funktioniert es:**

1. Oeffne **Settings** in der Sidebar
2. Setze **Haekchen bei mehreren Agenten** (Checkboxen, nicht Radio-Buttons)
3. Mindestens ein Agent muss aktiv bleiben
4. Die Statusbar unten zeigt alle aktiven Agenten an: `(Smart Search + Close Enough)`

**Was passiert bei einer Query?**

1. Frage geht an **alle aktiven Agenten** gleichzeitig
2. Jeder Agent berechnet unabhaengig eine Antwort + Confidence
3. Der Agent mit der **hoechsten Confidence gewinnt**
4. Die Antwort zeigt, **welcher Agent** gewonnen hat

**Empfohlene Kombinationen:**

| Kombination | Anwendungsfall |
|-------------|---------------|
| TF-IDF allein | Standard, schnell, gut genug |
| TF-IDF + Fuzzy | Semantik + Tippfehler-Toleranz |
| TF-IDF + Exact | Semantik + exakte Treffer bevorzugen |
| Alle 4 | Maximale Abdeckung, immer die beste Antwort |
| Context allein | Mehrteilige Gespraeche mit Gedaechtnis |

---

## 4. Knowledge Base trainieren

Die Knowledge Base (KB) ist das **Wissen** des Agenten. Ohne Trainingsdaten kann er nichts beantworten.

### 4.1 Manuell Beispiele hinzufuegen

**Schritt fuer Schritt:**

1. Gehe zu Tab **Knowledge** in der Sidebar
2. Waehle Sub-Tab **All**
3. Klicke **"Add Example"**
4. Fuell die Felder aus:
   - **Input**: Die Frage (z.B. "Was ist Rust?")
   - **Output**: Die Antwort (z.B. "Eine Systemprogrammiersprache von Mozilla")
   - **Format**: `text` (Standard), `markdown` (formatiert), oder `json` (strukturiert)
   - **Weight**: Gewichtung 0.0-10.0 (Standard: 1.0, hoeher = bevorzugt)
5. Klicke **Save**
6. Alle Agenten werden **automatisch neu trainiert**

**Wie Gewichtung funktioniert:**

- Weight `1.0` = normale Prioritaet
- Weight `2.0` = doppelt so wichtig, wird bei aehnlichen Treffern bevorzugt
- Weight `0.0` = wird ignoriert (deaktiviert ohne zu loeschen)

### 4.2 JSON importieren

**Format:**

```json
[
  {
    "input": "Was ist Rust?",
    "output": "Eine Systemprogrammiersprache",
    "format": "text",
    "weight": 1.0
  },
  {
    "input": "Wer hat Rust erfunden?",
    "output": "Graydon Hoare bei Mozilla",
    "format": "text",
    "weight": 1.0
  }
]
```

**Schritt fuer Schritt:**

1. Tab **Knowledge** > Sub-Tab **Import**
2. Option A: **JSON einfuegen** — JSON-Array in das Textfeld pasten, "Import" klicken
3. Option B: **Datei hochladen** — JSON-Datei per Drag & Drop oder Klick hochladen
4. Erfolgsmeldung zeigt Anzahl importierter Beispiele
5. Alle Agenten werden automatisch neu trainiert

### 4.3 PDF hochladen

**Was passiert:**

1. PDF wird hochgeladen und der Text extrahiert
2. Text wird in **Chunks** zerlegt (max. 1000 Zeichen, min. 50)
3. Chunks ueberlappen sich um 200 Zeichen (damit Kontext nicht verloren geht)
4. Jeder Chunk wird als Frage-Antwort-Paar gespeichert
5. Agenten werden neu trainiert

**Schritt fuer Schritt:**

1. Tab **Knowledge** > Sub-Tab **Import** > PDF-Bereich
2. PDF per **Drag & Drop** oder Klick hochladen
3. Warten bis "X examples added" erscheint
4. Fertig — die PDF-Inhalte sind jetzt durchsuchbar

**Limitierungen:**
- Nur textbasierte PDFs (keine gescannten Bilder/OCR)
- Tabellen und komplexe Layouts werden als Fliesstext extrahiert

### 4.4 Tatoeba-Import (Uebersetzungspaare)

Fuer zweisprachige Saetze von [manythings.org/anki/](http://manythings.org/anki/):

**Format (TSV):**
```
Hello	Hallo	CC-BY 2.0
How are you?	Wie geht es dir?	CC-BY 2.0
```

**Schritt fuer Schritt:**

1. TSV-Datei von manythings.org herunterladen
2. Tab **Knowledge** > Sub-Tab **Import** > Tatoeba-Bereich
3. TSV-Datei hochladen
4. Jede Zeile wird als Quellsprache→Zielsprache Paar gespeichert
5. Agenten werden neu trainiert

### 4.5 Knowledge Base speichern und laden

**Speichern:**
1. Tab **Knowledge** > Unten: Pfad eingeben (z.B. `./knowledge/meine_kb.json`)
2. **Save** klicken
3. KB wird als JSON-Datei auf dem Server gespeichert

**Laden:**
1. Tab **Knowledge** > Unten: Pfad zur JSON-Datei eingeben
2. **Load** klicken
3. Bestehende KB wird **ersetzt** (nicht gemergt!)
4. Agenten werden neu trainiert

### 4.6 Woher kommen die Daten?

```
Datenquellen:
  1. knowledge/train.json    ← Compile-Zeit (initial leer [])
  2. SQLite training_data    ← Manuell / Import / PDF / Tatoeba
  3. SQLite approved_data    ← Von Bots gesammelt + genehmigt

Beim Start:
  train.json + DB training_data → merged KnowledgeBase → trainiert alle Agenten

Zur Laufzeit:
  Neues Beispiel → KB + DB → alle Agenten sofort neu trainiert
```

---

## 5. Kategorien verwalten

Kategorien helfen, Trainingsdaten thematisch zu organisieren.

### Schritt fuer Schritt

1. Tab **Knowledge** > Sub-Tab **Categories**
2. **Neue Kategorie erstellen:**
   - Name eingeben (z.B. "Technik", "Smalltalk", "Support")
   - Farbe waehlen (Farbkreis)
   - "Create" klicken
3. **Trainingsdaten zuordnen:**
   - Beim Erstellen eines Beispiels die Kategorie auswaehlen
4. **Nach Kategorie filtern:**
   - Auf eine Kategorie klicken, um nur deren Eintraege zu sehen
5. **Kategorie loeschen:**
   - Delete-Button an der Kategorie
   - Trainingsdaten bleiben erhalten (Kategorie wird auf "Keine" gesetzt)

---

## 6. Bot Ecosystem — Automatisch Daten sammeln

Bots koennen **automatisch Webseiten crawlen** und daraus Trainingsdaten generieren.

### 6.1 Web Crawler erstellen

**Schritt fuer Schritt:**

1. Tab **Tools** > Sub-Tab **Bots**
2. Klicke **"New Bot"**
3. Konfiguriere:
   - **Name**: z.B. "Wikipedia Crawler"
   - **URL**: Die zu crawlende Webseite
   - **Modus**:
     - `single` — Nur diese eine Seite
     - `follow` — Links auf der Seite folgen (rekursiv)
   - **Max Depth**: Wie viele Link-Ebenen tief (Standard: 2)
   - **Rate Limit**: Pause zwischen Requests in ms (Standard: 1000)
4. Klicke **"Create"**
5. Klicke **"Start"** am Bot

### Was passiert beim Crawlen?

1. Bot laedt die URL herunter
2. HTML wird geparst, Text extrahiert
3. Content-Hash wird berechnet (SHA-256) zur Deduplizierung
4. Rohdaten werden als `raw_data` in der DB gespeichert (Status: `pending`)
5. Bei `follow`-Modus: Links auf der Seite werden gesammelt und ebenfalls gecrawlt
6. Bot-Run wird mit Statistik gespeichert (Items found, Items added)

### 6.2 Daten pruefen und genehmigen

**Schritt fuer Schritt:**

1. Tab **Tools** > Sub-Tab **Review**
2. **Pending-Daten ansehen**: Liste aller ungeprueften Eintraege
3. Fuer jeden Eintrag:
   - **Approve** — Daten sind gut, genehmigen
   - **Reject** — Daten sind schlecht, ablehnen
4. Oder: **"Approve All"** fuer alle auf einmal
5. Genehmigte Daten ansehen: Toggle "Show Approved"
6. **"Add to Knowledge"** klicken — genehmigte Daten werden in die KB uebernommen
7. Alle Agenten werden automatisch neu trainiert

### Workflow-Uebersicht

```
Bot crawlt Webseite
       |
       v
  raw_data (pending)
       |
  [Review: Approve/Reject]
       |
       v
  approved_data
       |
  [Add to Knowledge]
       |
       v
  KnowledgeBase + Agenten neu trainiert
```

---

## 7. VectorDB — Aehnlichkeitssuche

Die VectorDB speichert Texte als TF-IDF-Vektoren und ermoeglicht Aehnlichkeitssuche per Cosine-Similarity.

### Schritt fuer Schritt

1. Tab **Tools** > Sub-Tab **VectorDB**
2. **Collection erstellen:**
   - Name eingeben (z.B. "Produktbeschreibungen")
   - Optional: Beschreibung hinzufuegen
   - "Create" klicken
3. **Eintrag hinzufuegen:**
   - Collection auswaehlen
   - Text eingeben (z.B. eine Produktbeschreibung)
   - Optional: Metadaten als JSON (z.B. `{"category": "electronics"}`)
   - "Add" klicken — Embedding wird automatisch berechnet
4. **Suchen:**
   - Query eingeben (z.B. "kabelloses Headset")
   - Top-K Ergebnisse einstellen (Standard: 5)
   - "Search" klicken
   - Ergebnisse werden nach Aehnlichkeit sortiert mit Score

### Wie funktioniert es intern?

1. Beim Hinzufuegen: Text wird in TF-IDF-Vektor umgewandelt (basierend auf allen Eintraegen der Collection)
2. Bei der Suche: Query wird ebenfalls vektorisiert
3. Cosine-Similarity wird zwischen Query-Vektor und allen Entry-Vektoren berechnet
4. Top-K aehnlichste Eintraege werden zurueckgegeben

---

## 8. File Manager

Eingebauter Dateibrowser mit Editor.

### Funktionen

1. Tab **Tools** > Sub-Tab **Files**
2. **Navigieren**: Ordnerstruktur durchklicken
3. **Datei lesen**: Klick auf eine Datei oeffnet sie im Editor (rechte Seite)
4. **Datei bearbeiten**: Im Editor aendern, dann **"Save"** klicken
5. **Neue Datei**: "New File" — Name eingeben, Inhalt schreiben, speichern
6. **Neuer Ordner**: "New Folder" — Name eingeben
7. **Umbenennen**: Rechtsklick-Aktion auf Datei/Ordner
8. **Kopieren**: Quell- und Zielpfad angeben
9. **Loeschen**: Delete-Button (Ordner werden rekursiv geloescht!)
10. **SQLite inspizieren**: `.db`-Dateien anklicken zeigt Tabellen und Inhalte

### SQLite Browser

Wenn du eine `.db`-Datei oeffnest:
1. Liste aller Tabellen wird angezeigt
2. Klick auf eine Tabelle zeigt Spalten + Zeilen
3. Pagination fuer grosse Tabellen
4. INSERT/UPDATE/DELETE SQL ausfuehren (kein SELECT/DROP erlaubt)

---

## 9. Console — Live Terminal

Die Console ist **unten im Dashboard** angedockt.

### Bedienung

1. **Oeffnen/Schliessen**: Klick auf den Console-Header oder den `_` Button
2. **Server-Logs**: Erscheinen automatisch in Echtzeit (Info, Warn, Error)
3. **Befehl eingeben**: Im Eingabefeld tippen und Enter druecken

### Eingebaute Befehle

| Befehl | Wirkung |
|--------|---------|
| `status` | Zeigt aktuellen Agent, Anzahl KB-Eintraege, Version |
| `restart` | Stoppt und startet den Server neu |
| `stop` | Stoppt den Server (Dashboard bleibt offen) |
| `start` | Startet den Server nach Stop wieder |
| `exit` | Faehrt alles herunter und beendet den Prozess |
| `clear` | Loescht die Console-Ausgabe |

### Shell-Befehle

Alles was kein eingebauter Befehl ist, wird als **Shell-Kommando** ausgefuehrt:

```
ls -la                    → Dateien auflisten
git status                → Git-Status anzeigen
cat /etc/hostname         → Datei anzeigen
echo "Hello"              → Textausgabe
```

- Ausgabe erscheint in Echtzeit (stdout in weiss, stderr in orange)
- Laeuft asynchron — blockiert das Dashboard nicht

### Technische Details

- WebSocket-Verbindung auf Port 7071
- Mehrere Browser-Tabs erhalten alle den gleichen Log-Stream
- History: Letzte 500 Log-Eintraege werden gespeichert
- Beim Verbinden: Komplette History wird zugesendet

---

## 10. Settings — Anpassen

### Theme

1. Oeffne **Settings** in der Sidebar (Zahnrad-Icon)
2. **Dark** (Default) oder **Light** waehlen
3. Pro Theme separat konfigurierbar:
   - **Akzentfarbe**: Farbwaehler — bestimmt Buttons, Links, Highlights
   - **Hintergrundfarbe**: Manuell oder **Auto aus Akzent** (Komplementaerfarbe)
   - **Textfarbe**: Manuell oder **Auto aus Hintergrund** (Komplementaerfarbe)

### Sprache

- **English** / **Deutsch** / **Turkce**
- Dropdown in den Settings
- Alle UI-Elemente, Buttons, Labels werden sofort uebersetzt
- Ueber 200 uebersetzte Keys

### Textgroesse

- Slider von **11px** (Default) bis **20px**
- Aendert Chat-Nachrichten, KB-Eintraege, und allgemeine UI-Schrift

### Smart Settings ueber Chat

Statt in die Settings zu gehen, einfach im Chat schreiben:

| Eingabe | Wirkung |
|---------|---------|
| "Mach die Seite dunkel" | Wechselt zu Dark Mode |
| "Make it light" | Wechselt zu Light Mode |
| "Farbe gruen" | Setzt Akzentfarbe auf Gruen |
| "Background blau" | Setzt Hintergrundfarbe auf Blau |
| "Sprache Deutsch" | Wechselt UI-Sprache zu Deutsch |
| "Switch to English" | Wechselt UI-Sprache zu Englisch |

Funktioniert auf Deutsch, Englisch und Tuerkisch.

---

## 11. CLI Modus (ohne Web)

Fuer Nutzung ohne Browser, direkt im Terminal.

### Direkte Abfrage

```bash
airust cli query tfidf "Was ist Rust?"
airust cli query fuzzy "Was ist roost?"
airust cli query simple "Was ist Rust?"
airust cli query context "Erzaehl mir von Rust"
```

### Interaktiver Modus

```bash
airust cli
```

1. Agent-Typ auswaehlen (1-4)
2. Fragen stellen im Loop
3. `exit` zum Beenden

### Knowledge Base verwalten

```bash
airust cli knowledge
```

1. Neue KB erstellen — Frage/Antwort-Paare eingeben, als JSON speichern
2. Bestehende KB laden — Pfad angeben, testen oder erweitern

### CLI Tools

```bash
# PDF zu Knowledge Base konvertieren
cargo run --bin pdf2kb -- input.pdf output.json

# Mehrere Knowledge Bases zusammenfuehren
cargo run --bin merge_kb -- ./knowledge/
```

**merge_kb** durchsucht den angegebenen Ordner nach allen `.json`-Dateien, laedt jede als KB, merged sie zusammen, und speichert als `knowledge/train.json`.

---

## 12. Docker

### Bauen und Starten

```bash
# Bauen
docker build -t airust .

# Starten
docker run -p 7070:7070 airust

# Mit Landing Page
docker run -p 7070:7070 airust --landingpage

# Mit persistenter Datenbank
docker run -p 7070:7070 -v $(pwd)/airust.db:/app/airust.db airust

# Custom Port
docker run -p 8080:8080 airust --port 8080
```

### Was ist im Container?

- Rust-Binary (Release-Build, optimiert)
- Eingebettete Knowledge Base
- SQLite wird zur Laufzeit erstellt
- Keine externen Abhaengigkeiten

---

## 13. REST API — Programmzugriff

Alle Funktionen sind auch per API steuerbar. Hier die wichtigsten Beispiele:

### Status abfragen

```bash
curl http://localhost:7070/api/status
```

Antwort:
```json
{
  "agent": "tfidf",
  "active_agents": ["tfidf"],
  "examples": 42,
  "version": "0.1.7"
}
```

### Query senden

```bash
curl -X POST http://localhost:7070/api/query \
  -H "Content-Type: application/json" \
  -d '{"input": "Was ist Rust?", "add_context": false}'
```

Antwort:
```json
{
  "response": "Eine Systemprogrammiersprache",
  "confidence": 0.87,
  "agent": "tfidf",
  "agents_used": ["tfidf"]
}
```

### Agent wechseln

```bash
# Einzelner Agent
curl -X POST http://localhost:7070/api/agent/switch \
  -H "Content-Type: application/json" \
  -d '{"agent_type": "tfidf"}'

# Multi-Agent
curl -X POST http://localhost:7070/api/agent/switch \
  -H "Content-Type: application/json" \
  -d '{"agent_types": ["tfidf", "fuzzy", "exact"]}'
```

### Wissen hinzufuegen

```bash
curl -X POST http://localhost:7070/api/knowledge/add \
  -H "Content-Type: application/json" \
  -d '{"input": "Frage", "output": "Antwort", "weight": 1.0}'
```

### PDF hochladen

```bash
curl -X POST http://localhost:7070/api/pdf/upload \
  -F "file=@dokument.pdf"
```

### Trainingsdaten exportieren

```bash
curl http://localhost:7070/api/training/export > backup.json
```

### Trainingsdaten importieren

```bash
curl -X POST http://localhost:7070/api/training/import \
  -H "Content-Type: application/json" \
  -d @training_data.json
```

### Alle Endpoint-Gruppen

| Gruppe | Pfad | Beschreibung |
|--------|------|-------------|
| Query | `POST /api/query` | Agent-Abfragen mit Confidence |
| Status | `GET /api/status` | Server-Status und aktive Agenten |
| Agent | `POST /api/agent/switch` | Agent(en) wechseln |
| Knowledge | `/api/knowledge/*` | KB anzeigen, hinzufuegen, loeschen, speichern, laden |
| Training | `/api/training/*` | Trainingsdaten + Kategorien CRUD |
| Import | `POST /api/upload/json`, `/api/import/tatoeba` | Bulk-Import |
| PDF | `POST /api/pdf/upload` | PDF-Upload und Extraktion |
| Chats | `/api/chats/*` | Chat-History verwalten |
| Bots | `/api/bots/*` | Bot CRUD + Start/Stop |
| Review | `/api/data/*` | Daten pruefen + genehmigen |
| Vectors | `/api/vectors/*` | VectorDB Collections + Suche |
| Files | `/api/files/*` | Dateisystem-Operationen |
| DB | `/api/files/db/*` | SQLite-Browser |
| Settings | `/api/settings` | Einstellungen lesen/schreiben |
| Translations | `GET /api/translations/:lang` | UI-Uebersetzungen |

---

## Architektur

```
┌─────────────────────────────────────────┐
│            Web Dashboard                │
│  Chat | Knowledge | Tools | Settings    │
├─────────────────────────────────────────┤
│         REST API (50+ Endpoints)        │
├──────────┬──────────┬──────┬────────────┤
│ TF-IDF   │ Exact    │Fuzzy │  Context   │
│ Agent    │ Match    │Match │  Agent     │
│ (BM25)   │ Agent    │Agent │ (TF-IDF+) │
├──────────┴──────────┴──────┴────────────┤
│          Knowledge Base                 │
│  Embedded JSON + SQLite Training Data   │
├─────────────────────────────────────────┤
│     SQLite (airust.db)                  │
│  Chats, Settings, Bots, Vectors, ...    │
├─────────────────────────────────────────┤
│     Console (WebSocket :7071)           │
│  Logs, Shell, Server-Steuerung          │
└─────────────────────────────────────────┘
```

### Kerneigenschaften

- **100% Lokal** — Keine Cloud, keine externen APIs, keine Telemetrie
- **156 Tests** bestanden (Unit + Integration)
- **MIT Lizenz** — frei nutzbar
- **Rust 1.85+** — schnell, sicher, kompiliert
- **Zero Dependencies** zur Laufzeit (alles im Binary)
