import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

type Setting = {
  name: String;
  hotkey: String;
  app_type: String;
  exe_path: String;
  mode: String;
  enabled: boolean;
};

function Index() {
  const [apps, setApps] = useState<Setting[]>([]);

  const fetchAll = async () => {
    setApps(await invoke("get_settings"));
  };

  useEffect(() => {
    fetchAll();
  }, []);

  return (
    <main>
      <div className="mb-4">
        <input type="text" placeholder="Search" className="border-gray-400" />
        <ul>
          <li>Applications</li>
          <li>Window Commands</li>
        </ul>
      </div>
      <div className="overflow-y-auto bg-amber-100 max-h-screen">
        <table>
          {/*Table header*/}
          <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Hotkey</th>
            <th>Mode</th>
            <th>Enabled</th>
          </tr>
          {apps.length > 0 && (
            <>
              {apps.map((app) => (
                <tr className="pb-4">
                  <td>{app.name}</td>
                  <td>{app.app_type}</td>
                  <td>{app.hotkey ? app.hotkey : "Input Hotkey"}</td>
                  <td>{app.mode}</td>
                  <td>{app.enabled ? "True" : "False"}</td>
                </tr>
              ))}
            </>
          )}
        </table>
      </div>
    </main>
  );
}

export default Index;
