import "./App.css";
import Index from "./views/Index";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
function App() {
  return (
    <div className="p-4 overflow-y-auto h-screen">
      <header className="mb-4 border-b border-gray">
        {/*Icon*/}
        {/* <p>Johndoe@yaykeys.app</p> */}
        <p>version 0.1.0</p>
        <span>Beta</span>
      </header>
      <Index />
    </div>
  );
}

export default App;
