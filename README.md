---
marp: true
theme: default
class: lead
paginate: true
---

![width:1000px](tauri-logo.png)

---

## Prérequis

Si vous avez docker : 
🔗 https://sourceforge.net/projects/vcxsrv/  
➡️ Lancer **VcXsrv** avant de démarrer l'app via Docker


Si vous **n'avez pas Docker** :  
📥 Installez :
- Visual Studio C++ Build Tools  
- Rustup 
🔗 https://www.rust-lang.org/tools/install

---

## Pourquoi Electron a été créé ? ⚙️

- Besoin de créer des apps **desktop multiplateformes** avec des compétences web
- Utilise :
  - **Node.js** pour le backend
  - **Chromium** pour le rendu UI
- Objectif : développement rapide avec JavaScript

**Exemples célèbres** : VS Code, Discord, Slack

---

## Qu'est-ce que Tauri ? 🦀

- Framework pour apps desktop **ultra-légères**
- Frontend : HTML/CSS/JS (React, Vue, etc.)
- Backend : écrit en **Rust**
- Utilise le **WebView natif** du système, pas Chromium

---

## Pourquoi Tauri plutôt qu'Electron ? ⚖️

| Critère          | Electron         | Tauri             |
|------------------|------------------|-------------------|
| Poids app        | 100–200 Mo       | 3–10 Mo           |
| Performance      | Gourmand         | Léger, rapide     |
| Sécurité         | Moyenne          | Élevée            |
| Backend          | Node.js (JS)     | Rust              |
| WebView          | Chromium         | Natif             |
| Maturité         | Élevée           | Jeune, prometteur |

---

## Spécificités de Tauri ✨

- Très **léger**
- Écrit en **Rust**
- Sécurité renforcée (isolation, permissions)
- Modularité (importe que ce qui est utile)
- Intégration native (menu, notifs, raccourcis)
- Communication **Rust ↔️ JS**

---

## En résumé 📝

**Electron** ✅  
✔ Mature et stable  
✔ Facile pour devs JS

**Tauri** 🦀  
✔ Léger et rapide  
✔ Sécurisé par design  
✔ Rust = plus complexe, mais puissant

---

## Tauri et Rust 🦀

- Le **backend** de Tauri est écrit en **Rust**
- Rust = langage système moderne :
  - 🔐 **Sécurité mémoire** avec un concept d'ownership et de borrowing
  - ⚡ **Haute performance**
  - 🧩 **Conception modulaire**

> Rust évite les crashs et bugs classiques liés aux pointeurs et à la gestion manuelle de mémoire

---

## Ownership

![width:1000px](Rust_ownership.png)

---

## Borrowing 

![width:1000px](Rust_borrowing.png)

---

## Architecture interne de Tauri 🏗️

Tauri repose sur deux composants internes :

### 🧱 **Tao**
- Abstraction native multiplateforme pour :
  - Fenêtres
  - Événements systèmes
  - Menu / raccourcis / dialogues
- Alternative légère à Electron

---

### 🌐 **Wry**
- Intègre un **WebView natif** :
  - **WebKit** sur macOS/Linux
  - **WebView2** (Edge) sur Windows
- Gère la communication JS ↔ Rust

---

## Sécurité dans Tauri 🔐

Tauri a été **conçu autour de la sécurité** :

✅ WebView isolé  
✅ Permissions explicites par API  
✅ Aucune API système exposée par défaut  
✅ Communication contrôlée (commandes Rust uniquement déclarées)  
✅ Code signé et vérifiable  
✅ Mise à jour sécurisée (chiffrement + signature)

---

## Comment créer une app Tauri ?

npm create tauri-app@latest

![width:400px](Tauri_init.png)

---

## Merci 🙌

Des questions ?  

---

Build :
docker build -t tauri-builder .

Run :
docker run -it --rm -e DISPLAY=host.docker.internal:0.0 -v "${PWD}:/app" tauri-builder