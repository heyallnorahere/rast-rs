pub struct ShaderContext<'a, T> {
    pub vertex_id: u32,
    pub instance_id: u32,
    pub data: &'a T,
}

pub struct VertexOutput<T> {
    pub position: [f32; 4],
    pub data: T,
}

pub trait Shader<T, U> {
    fn vertex_stage(context: &ShaderContext<U>) -> VertexOutput<T>;
    fn fragment_stage(context: &ShaderContext<U>, inputs: &[VertexOutput<T>; 3]) -> u32;
}
