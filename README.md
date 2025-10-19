A black hole simulation written in Rust.

Ray tracer code based on the guide: [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app)

## TODO

- [ ] Set up wgpu to perform ray calculations on the GPU
    * [x] Change Material type in Rust code to not use dynamic dispatch
    * [x] Figure out where to get seed values for rand functions
    * [x] Rework ray_color to remove recursion
    * [x] Implement emission
- [ ] Research differential equations and relativity
- [ ] Change rays to geodesics and introduce black hole/massive objects
