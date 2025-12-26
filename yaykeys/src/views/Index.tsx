import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Setting } from "../utils/types";
import { Search, X } from "lucide-react";

function Index() {
  const [apps, setApps] = useState<Setting[]>([]);
  const [showPop, setShowPop] = useState<boolean>(false);

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

  const handleClearHotkey = (id: number) => {
    console.log(id);
  };

  const handlePopUp = () => {
    setShowPop(!showPop);
  };

  const handleModeSelect = async (event: any, id: number) => {
    if (event.target) {
      await invoke("update_mode", { id: id, value: event.target.value });
    }
  };

  const handleHotkey = () => {};

  return (
    <main className="text-sm">
      <div className="mb-4 flex items-center gap-8">
        <div className="relative">
          {/* TODO: Add cancel button to search field */}
          <Search className="absolute left-3 top-0 h-4 w-4 translate-y-1/2 text-neutral-400" />
          <input
            type="text"
            placeholder="Search..."
            onChange={(event) => handleSearch(event)}
            className="w-50 h-8 border rounded-lg border-neutral-400 bg-neutral-100 pl-9  text-neutral-700 placeholder:text-neutral-500"
          />
        </div>
      </div>
      <div className="overflow-y-auto max-h-126 border border-neutral-300 rounded-lg p-1">
        <table className="w-full">
          {/*Table header*/}
          <tr className="border-b border-neutral-300 py-3 text-neutral-500 text-left h-10">
            <th>Name</th>
            <th>Hotkey</th>
            <th>Mode</th>
            <th>Enabled</th>
          </tr>
          {apps.length > 0 && (
            <>
              {apps.map((app) => (
                <tr className="border-b border-neutral-300  hover:bg-gray-50/20 h-10">
                  <td>{app.name}</td>
                  <td className="relative">
                    {/* TODO: Only show pop up for the selected application
                     */}
                    {showPop && (
                      <div className="absolute -top-50 w-40 h-20 p-4 bg-popup border z-10">
                        <button
                          onClick={() => handlePopUp()}
                          className="cursor-pointer hover:bg-blue-500 hover:text-white rounded"
                        >
                          <X className="h-4 w-4" />
                        </button>
                        <p>e.g CMD 2</p>
                      </div>
                    )}

                    <div
                      className="w-40 hover:border rounded-lg border-neutral-400 px-4 py-1 cursor-pointer flex justify-between"
                      onClick={() => handlePopUp()}
                    >
                      <p>{app.hotkey ? app.hotkey : "Input Hotkey"}</p>
                      <button
                        onClick={() => handleClearHotkey(app.id)}
                        className="cursor-pointer hover:bg-blue-500 hover:text-white rounded"
                      >
                        <X className="h-4 w-4" />
                      </button>
                    </div>
                  </td>
                  <td>
                    <select onChange={(event) => handleModeSelect(event, app.id)}>
                      <option value="Default" selected={app.mode === "Default"}>
                        Default
                      </option>
                      <option value="FullScreen" selected={app.mode === "FullScreen"}>
                        Full Screen
                      </option>
                      <option value="Desktop" selected={app.mode === "Desktop"}>
                        Desktop
                      </option>
                    </select>
                  </td>
                  <td>
                    <input type="checkbox" checked={app.enabled} onClick={(event) => handleEnabledApp(event, app.id)} />
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
