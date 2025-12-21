import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Setting } from "../utils/types";
import { Search } from "lucide-react"
import Application from "../components/Applications";


function Index() {
  const [apps, setApps] = useState<Setting[]>([]);
  const [activeTab, setActiveTab] = useState<String>("Application")

  const fetchAll = async () => {
    setApps(await invoke("get_settings"));
  };

  useEffect(() => {
    fetchAll();
  }, []);

  // const handleActiveFilter = () => {
  // }

  return (
    <main>
      <div className="mb-4 flex items-center gap-8">
        <div className="relative">
          <Search className="absolute left-3 top-0 h-4 w-4 translate-y-1/2 text-neutral-400"/>
          <input 
            type="text" 
            placeholder="Search..." 
            className="w-50 h-8 border rounded-xl border-neutral-400 bg-neutral-100 pl-9 text-sm text-neutral-700 placeholder:text-neutral-500" 
          />
        </div>
          <ul className="flex items-center gap-4 rounded-lg bg-neutral-100 p-2">
            <li className={`rounded-md px-3 py-1.5 text-sm font-medium transition-colors ${activeTab === "Application" ? "bg-white text-neutral-900 shadow-sm" : "text-neutral-500 hover:text-neutral-700"}`}> 
              Applications 
            </li>
            {/* <li>Window Commands</li> */}
          </ul>
      </div>
      <div className="overflow-y-auto max-h-126 border border-neutral-300 rounded-xl p-1">
        {activeTab === "Application" ? (
          <>
        <Application apps={apps}/>
          </>
        ):(<><p>Coming Soon</p></>)}
      </div>
    </main>
  );
}

export default Index;
