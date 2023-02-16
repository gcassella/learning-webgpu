use learn_wgpu::run;
use pollster::block_on;

pub fn main() {
    block_on(run());
}
