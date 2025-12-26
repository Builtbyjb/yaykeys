import "./App.css";
import Index from "./views/Index";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
function App() {
  return (
    <div className="p-4 overflow-y-auto h-screen">
      <header className="mb-4 border-b border-gray text-sm">
        {/*Icon*/}
        <p>Johndoe@yaykeys.app</p>
        <span>Beta</span>
      </header>
      <Index />
      <p className="text-sm mt-2 italic">version 0.1.0</p>
    </div>
  );
}

export default App;
