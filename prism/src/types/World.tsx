import Material from "./Material";
import { Solid } from "./Solid";

export default interface World {
  materials: Material[];
  objects: Solid[];
}
