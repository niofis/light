import { createContext, createSignal, useContext } from "solid-js";

const SceneContext = createContext();

export function SceneProvider(props) {
  const [scene, setScene] = createSignal({});

  return (
    <SceneContext.Provider value={[scene, setScene]}>
      {props.children}
    </SceneContext.Provider>
  );
}

export function useScene() {
  return useContext(SceneContext);
}
