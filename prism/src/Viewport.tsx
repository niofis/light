import { invoke } from "@tauri-apps/api/core";
import { createSignal } from "solid-js";

const WIDTH = 640;
const HEIGHT = 360;

export function Viewport() {
  let [running, setRunning] = createSignal<boolean>(false);
  let [sceneJson, setSceneJson] = createSignal<string>(SAMPLE_SCENE_JSON);
  let canvas: HTMLCanvasElement | undefined;

  const initializeRenderer = (json: string) => {
    invoke("initialize_renderer", { json });
  };
  const generateImage = async () => {
    if (!canvas) return;
    while (running()) {
      const imgArray: ArrayBuffer = await invoke("generate_image");
      const ctx = canvas.getContext("2d");
      const imageData = new ImageData(new Uint8ClampedArray(imgArray), WIDTH);
      ctx?.putImageData(imageData, 0, 0);
    }
  };

  const toggleImageGeneration = () => {
    if (!running()) {
      initializeRenderer(sceneJson());
      setRunning(true);
      generateImage();
    } else {
      setRunning(false);
    }
  };
  return (
    <div class="container">
      <div class="row">
        <div class="three columns">
          <textarea
            value={sceneJson()}
            style={{ height: "40rem", width: "100%" }}
            onChange={(e) => setSceneJson(e.target.value)}
          />
          <br />
          <button
            type="button"
            class="button-primary"
            style={{ width: "100%" }}
            onClick={toggleImageGeneration}
          >
            {running() ? "Stop" : "Start"}
          </button>
        </div>
        <div class="nine columns">
          <canvas
            ref={canvas}
            width={WIDTH}
            height={HEIGHT}
            style={{
              "image-rendering": "pixelated",
              width: "100%",
              height: "auto",
            }}
          ></canvas>
        </div>
      </div>
    </div>
  );
}

const SAMPLE_SCENE_JSON = `{
      "camera": {
        "eye": [0.0, 0.0, -5.0],
        "leftBottom": [-8.0, -4.5, 5.0],
        "leftTop": [-8.0, 4.5, 5.0],
        "rightTop": [8.0, 4.5, 5.0],
        "transforms": [
          {
            "type": "rotate",
            "values": [0.0, 0.0, 0.0]
          },
          {
            "type": "translate",
            "values": [0.0, 7.5, -50.0]
          }
        ]
      },
      "world": {
        "materials": [
          {
            "type": "diffuse",
            "color": [1.0, 0.0, 0.0],
            "id": "diffuse-red"
          },
          {
            "type": "diffuse",
            "color": [0.0, 1.0, 0.0],
            "id": "diffuse-green"
          },
          {
            "type": "emissive",
            "color": [3.0, 3.0, 3.0],
            "id": "emissive-white"
          },
          {
            "type": "diffuse",
            "color": [1.0, 1.0, 1.0],
            "id": "diffuse-white"
          },
          {
            "type": "emissive",
            "color": [1.0, 0.0, 0.0],
            "id": "emissive-red"
          },
          {
            "type": "reflective",
            "color": [1.0, 1.0, 1.0],
            "id": "reflective-white"
          },
          {
            "type": "refractive",
            "color": [1.0, 1.0, 1.0],
            "id": "refractive-test"
          }
        ],
        "objects": [
          {
            "type": "sphere",
            "center": [0.0, 0.0, 0.0],
            "radius": 5.0,
            "sections": 100.0,
            "material": "diffuse-green"
          },
          {
            "type": "plane",
            "material": "emissive-white",
            "transforms": [
              {
                "type": "scale",
                "values": [30.0, 10.0, 10.0]
              },
              {
                "type": "translate",
                "values": [0.0, 25.0, -5.0]
              }
            ]
          },
          {
            "type": "plane",
            "material": "diffuse-white",
            "transforms": [
              {
                "type": "scale",
                "values": [30.0, 30.0, 30.0]
              },
              {
                "type": "rotate",
                "values": [3.1415926, 0.0, 0.0]
              },
              {
                "type": "translate",
                "values": [0.0, -6.0, 0.0]
              }
            ]
          },
          {
            "type": "plane",
            "material": "reflective-white",
            "transforms": [
              {
                "type": "scale",
                "values": [10.0, 10.0, 10.0]
              },
              {
                "type": "rotate",
                "values": [1.570796, 0.4, 0.0]
              },
              {
                "type": "translate",
                "values": [12.0, 0.0, 12.0]
              }
            ]
          }
        ]
      }
    }
`;
