# Light v2

## Introduction
Light is my pet project I've been working on and off for a long time. It is a path tracer (mainly) that I use to experiment with different approaches for work difivision and acceleration structures.

This project is build in Rust but also includes a desktop version that uses Tauri v2, as another experiment on creating a UI for desktop and iOS.

Light is divided in several modules, as described below.

### Light-wasm

A wasm compatible version that exposes some simple API to trigger the rendering.

### Ligh

The main library that has all the logic for the rendering workflows.

### Photon

A simple CLI frontend for faster testing, which also exposes various parameters to modify the rendering behavior.

### Prism

A Tauri frontend for experimentation, at some point, it should allow for modification of the scenes in real time.

### Screen

A SDL2 frontend. This has not received too much support in a while so I might consider deprecating it.

## Compiling

To compile this application, you need to have the latest version of the Rust programming language installed.

If you want to compile the Tauri application, you will also need to install Node.js v22.


