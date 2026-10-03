import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import LogoImg from './assets/p_manager.png';

function TextField({ text, value, onChange }) {
  return (
    <div className="field-container">
      <p className="field-text">{text}</p>
      <input
        className="field-input"
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
    </div>
  );
}

function App() {
  // 1. State for our inputs
  const [appName, setAppName] = useState("");
  const [userName, setUserName] = useState("");
  const [password, setPassword] = useState("");
  const [feedback, setFeedback] = useState("");

  // 2. Button Logic
  async function handleSave() {
    if (!appName) return setFeedback("App Name is required");
    const res = await invoke("save_entry", {
      app: appName,
      uname: userName,
      passwd: password
    });
    setFeedback(res);
  }

  async function handleSearch() {
    try {
      const entry = await invoke("search_entry", { target: appName });
      setUserName(entry.data.uname);
      setPassword(entry.data.passwd);
      setFeedback("Entry found!");
    } catch (err) {
      setFeedback(err); // Rust returns Err if not found
    }
  }

  async function handleDelete() {
    const res = await invoke("delete_entry", { target: appName });
    setFeedback(res);
    // Clear fields on success
    if (res.includes("successfully")) {
      setAppName("");
      setUserName("");
      setPassword("");
    }
  }

  return (
    <>
      <div className="logo-container">
        <img src={LogoImg} className="logo-img" alt="logo" />
        <h2 className="logo-text">Password Manager</h2>
      </div>

      <div className="app-container">
        <TextField text="App Name" value={appName} onChange={setAppName} />
        <TextField text="Username" value={userName} onChange={setUserName} />
        <TextField text="Password" value={password} onChange={setPassword} />
      </div>

      <div className="button-container" style={{ display: "flex", justifyContent: "center", gap: "10px", marginTop: "20px" }}>
        <button onClick={handleSave}>Save</button>
        <button onClick={handleSearch}>Search</button>
        <button onClick={handleDelete}>Delete</button>
      </div>

      <p id="feedback-paragraph" style={{ textAlign: "center", color: "dodgerblue", marginTop: "20px" }}>
        {feedback}
      </p>
    </>
  );
}

export default App;