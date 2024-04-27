fn main() {
    println!("*** WGPU Demo 1 ***");
    pollster::block_on(wgpu_playground::run());
}
