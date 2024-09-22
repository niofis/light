import Point from "./Point";

export interface Camera {
  eye: Point;
  leftBottom: Point;
  leftTop: Point;
  rightTop: Point;
  transforms: Transform[];
}
