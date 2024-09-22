import Color from "./Color";

export type MaterialType = "diffuse" | "emissive" | "reflective" | "refractive";
export type MaterialId = string;

export default interface Material {
  type: MaterialType;
  color: Color;
  id: MaterialId;
}
