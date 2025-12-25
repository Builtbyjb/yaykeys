import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Setting } from "../utils/types";
import { Search } from "lucide-react";

function Index() {
  const [apps, setApps] = useState<Setting[]>([]);

  const fetchAll = async () => {
    setApps(await invoke("get_settings"));
  };

  useEffect(() => {
    fetchAll();
  }, []);

  const handleSearch = async (event: any) => {
    if (event.target) {
      setApps(await invoke("search", { query: event.target.value }));
    }
  };

  const handleEnabledApp = async (event: any, id: number) => {
    const updatedApps = apps.map(async (app) => {
      if (app.id === id) {
        if (event.target) {
          app.enabled = event.currentTarget.checked;
          await invoke("update_enabled", {
            id: app.id,
            value: app.enabled,
          });
        }
      }
      return app;
    });

    Promise.all(updatedApps)
      .then((result) => setApps(result))
      .catch((err) => console.log(err));
  };

  return (
    <main>
      <div className="mb-4 flex items-center gap-8">
        <div className="relative">
          <Search className="absolute left-3 top-0 h-4 w-4 translate-y-1/2 text-neutral-400" />
          <input
            type="text"
            placeholder="Search..."
            onChange={(event) => handleSearch(event)}
            className="w-50 h-8 border rounded-xl border-neutral-400 bg-neutral-100 pl-9 text-sm text-neutral-700 placeholder:text-neutral-500"
          />
        </div>
      </div>
      <div className="overflow-y-auto max-h-126 border border-neutral-300 rounded-xl p-1">
        <table className="w-full">
          {/*Table header*/}
          <tr className="border-b border-neutral-300 py-3 text-neutral-500 text-left">
            <th>Name</th>
            <th>Hotkey</th>
            <th>Mode</th>
            <th>Enabled</th>
          </tr>
          {apps.length > 0 && (
            <>
              {apps.map((app) => (
                <tr className="border-b border-neutral-300 px-4 p-4 hover:bg-gray-50/20">
                  <td>{app.name}</td>
                  <td>{app.hotkey ? app.hotkey : "Input Hotkey"}</td>
                  <td>
                    {/* TODO: Select field*/}
                    {app.mode}
                  </td>
                  <td>
                    {/*
                      NOTE:
                      */}
                    <input
                      type="checkbox"
                      checked={app.enabled}
                      onClick={(event) => handleEnabledApp(event, app.id)}
                    />
                  </td>
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
