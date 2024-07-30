import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [brightness, setBrightness] = useState("");
  const [action, setAction] = useState('');

  function streamdeckBrightness() {
    switch (action) {
      default:
      case "sd_set_brightness":
        invoke(action, { brightness: Number(brightness) })
        break;
      case "sd_fade_brightness":
        invoke(action, { brightness: Number(brightness) })
        break;
    }
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          streamdeckBrightness();
        }}
      >
        <input
          id="greet-input"
          type="number"
          min={0}
          max={100}
          onChange={(e) => setBrightness((e.currentTarget.value))}
          placeholder="Enter a value..."
        />
        <button onClick={(_) => setAction("sd_set_brightness")} value="set_brightness" type="submit">Set Brightness</button>
        <button onClick={(_) => setAction("sd_fade_brightness")} type="submit">Fade to!</button>
      </form>
    </div>
  );
}

export default App;
