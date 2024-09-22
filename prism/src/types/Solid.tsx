import { MaterialId } from "./Material";
import Point from "./Point";
import Transform from "./Transform";

export type SolidType = "sphere" | "plane";

export interface Solid {
  type: SolidType;
  material: MaterialId;
  transforms: Transform[];
}

export interface Sphere extends Solid {
  center: Point;
  radius: number;
  sections: number;
}

export interface Plane extends Solid {}
