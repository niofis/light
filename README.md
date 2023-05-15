# Light v2

---

This is the new and improved version of the light renderer built on Rust.

### Pipeline

Stages

1. Generate Geometry (GeometryGenerator) -> Vec<Solid>
1. Generate primary rays (Camera) -> Vec<PrimaryRay>
1. Group rays (Ray grouper) -> Vec<RayGroup>
1. RenderGroups(RenderAlgorithm) -> Vec<RayGroupResults>
1. ComposeImage(ImageComposer) -> Vec<Color>
1. Preprocessing(Preprocesor)
1. Finish
