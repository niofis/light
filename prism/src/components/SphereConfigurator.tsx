import { createEffect, createSignal } from "solid-js";
import { Sphere } from "../types/Solid";
import { NumberField } from "@kobalte/core/number-field";

export interface SphereConfiguratorProps {
  sphere: Sphere;
  onChange: (arg0: Sphere) => void;
}

export default function SphereConfigurator(props: SphereConfiguratorProps) {
  const [sphere, setSphere] = createSignal({ ...props.sphere });

  createEffect(() => {
    setSphere({ ...props.sphere });
  }, [props.sphere]);

  return (
    <>
      <NumberField defaultValue={0.0}>
        <NumberField.Label>Center X</NumberField.Label>
        <NumberField.Input />
      </NumberField>
      <NumberField defaultValue={0.0}>
        <NumberField.Label>Center Y</NumberField.Label>
        <NumberField.Input />
      </NumberField>
      <NumberField defaultValue={0.0}>
        <NumberField.Label>Center Z</NumberField.Label>
        <NumberField.Input />
      </NumberField>
    </>
  );
}
