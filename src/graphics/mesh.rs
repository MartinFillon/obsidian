#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<u32>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<u32>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    // pub fn cube() -> Self {
    //     // Front
    //     let f1 = VertexData::new([[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
    //     let f2 = VertexData::new([[0.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]]);
    //     // East
    //     let e1 = VertexData::new([[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]]);
    //     let e2 = VertexData::new([[1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]]);
    //     // North
    //     let n1 = VertexData::new([[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]]);
    //     let n2 = VertexData::new([[1.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]]);
    //     // West
    //     let w1 = VertexData::new([[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]]);
    //     let w2 = VertexData::new([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]]);
    //     // Top
    //     let t1 = VertexData::new([[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
    //     let t2 = VertexData::new([[0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]]);
    //     // Bottom
    //     let t3 = VertexData::new([[1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 0.0]]);
    //     let t4 = VertexData::new([[1.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    //
    //     let vertices = vec![f1, f2, e1, e2, n1, n2, w1, w2, t1, t2, t3, t4];
    //     let indices = vec![
    //         0, 1, 2, 3, 4, 5, // Front
    //         6, 7, 8, 9, 10, 11, // Back
    //     ];
    //
    //     Self::new(vertices, indices)
    // }
}
