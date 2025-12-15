import "./App.css";
import Index from "./views/Index";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
function App() {
  return (
    <div className="p-8 overflow-y-auto h-screen">
      <header className="mb-4">
        {/*Icon*/}
        <p>Johndoe@yaykeys.app</p>
        <span>Beta</span>
        <p>v0.1.0</p>
      </header>
      <Index />
    </div>
  );
}

export default App;
