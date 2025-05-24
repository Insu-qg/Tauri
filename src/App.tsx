import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, requestPermission,sendNotification } from "@tauri-apps/plugin-notification";
import "./App.css";
import { useState } from 'react';

function App() {
  const [note, setNote] = useState('');

  async function notifyUser() {
  let permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === 'granted';
  }
  if (permissionGranted) {
    sendNotification({
      title: "SecureNotes",
      body: "✅ Note sauvegardée avec succès !",
    });
  } else {
    alert("Notifications non autorisées !");
  }
}

  const handleSave = async () => {
    try {
      await invoke("save_note", { content: note });
      notifyUser();
    } catch (error) {
      alert("Erreur lors de la sauvegarde : " + error);
    }
  };

  const handleLoad = async () => {
    try {
      const content = await invoke<string>("load_note");
      setNote(content);
    } catch (error) {
      alert("Erreur lors du chargement : " + error);
    }
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif" }}>
      <h1>📝 SecureNotes</h1>
      <textarea
        rows={10}
        cols={50}
        value={note}
        onChange={(e) => setNote(e.target.value)}
        style={{ width: "100%", marginBottom: "1rem" }}
      />
      <div>
        <button onClick={handleSave}>💾 Enregistrer</button>
        <button onClick={handleLoad} style={{ marginLeft: "1rem" }}>
          📂 Ouvrir
        </button>
      </div>
    </div>
  );
}

export default App;

// import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";

// function App() {
//   const [note, setNote] = useState("");
//   const [response, setResponse] = useState("");

//   const handleSend = async () => {
//     const result = await invoke<string>("process_note", { note });
//     setResponse(result);
//   };

//   return (
//     <div style={{ padding: "2rem" }}>
//       <h1>📝 SecureNotes - Démo Invoke + Borrowing</h1>
//       <textarea
//         rows={10}
//         cols={50}
//         value={note}
//         onChange={(e) => setNote(e.target.value)}
//         style={{ width: "100%", marginBottom: "1rem" }}
//       />
//       <div>
//         <button onClick={handleSend}>Envoyer à Rust</button>
//       </div>
//       <p style={{ marginTop: "1rem", fontStyle: "italic" }}>
//         Réponse de Rust : {response}
//       </p>
//     </div>
//   );
// }

// export default App;