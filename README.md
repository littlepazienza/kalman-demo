<div align="center">

  <h1><code>kalman-demo</code></h1>

<strong>rust-wasm kalman filter demo project</strong>

![workflow](https://github.com/littlepazienza/kalman-demo/actions/workflows/main.yml/badge.svg)
</div>

# kalman-demo

Demo project for a personal research project on Kalman Filters

# rust-wasm

The top level project source code of this project is a rust project library using
wasm bindgen, which enables the library to be assembled and packaged as a npm project.

# usage

Build the library using the `wasm-pack build` command to generate a JS package to the `pkg` directory.
This package is uploaded to [npm](https://www.npmjs.com/package/@littlepaz/kalman-demo)

Then you can consume these binaries as a typical js module in the `www` directory, where the demo project interface lives. I host this via the `npx webpack` command, which compiles all of the js modules into a self contained html and js project. This is hosted on my website.
