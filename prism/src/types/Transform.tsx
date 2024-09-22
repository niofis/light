export type TransformType = "rotate" | "translate" | "scale";

export default interface Transform {
  type: TransformType;
  values: [number, number, number];
}
