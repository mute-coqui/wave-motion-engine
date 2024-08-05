use russimp::scene::{PostProcess, Scene};

fn main() {
    let _scene = Scene::from_file(
        "",
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType,
        ],
    )
    .unwrap();
}
