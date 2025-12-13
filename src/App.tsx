import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Setting = {
  name: String;
  hotkey: String;
  app_type: String;
  exe_path: String;
  mode: String;
  enabled: boolean;
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
function App() {
  const [apps, setApps] = useState<Setting[]>([]);

  const fetchAll = async () => {
    setApps(await invoke("get_settings"));
  };

  useEffect(() => {
    fetchAll();
  }, []);

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      {apps.length > 0 && (
        <div>
          {apps.map((app) => (
            <div>
              <p>{app.name}</p>
              <p>{app.exe_path}</p>
              <p>{app.app_type}</p>
            </div>
          ))}
        </div>
      )}
    </main>
  );
}

export default App;
