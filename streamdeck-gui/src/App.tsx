import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [brightness, setBrightness] = useState("");
  const [action, setAction] = useState('');

  function streamdeckBrightness() {
    switch (action) {
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

      <h1>streamdeck-gui debug</h1>

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
        <button onClick={(_) => setAction("sd_set_brightness")} type="submit">Set Brightness</button>
        <button onClick={(_) => setAction("sd_fade_brightness")} type="submit">Fade to!</button>
      </form>
    </div>
  );
}

export default App;
