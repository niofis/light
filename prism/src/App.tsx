import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import "./skeleton.css";
import { Viewport } from "./Viewport";
import { SceneProvider } from "./contexts/SceneContext";

function App() {
  // const [greetMsg, setGreetMsg] = createSignal("");
  // const [name, setName] = createSignal("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name: name() }));
  // }

  return (
    <SceneProvider>
      <Viewport />
    </SceneProvider>
  );
}

export default App;
