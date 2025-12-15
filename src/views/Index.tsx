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
    <main className="">
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

export default Index;
