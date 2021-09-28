use nalgebra::{Vector3,VecStorage,ArrayStorage, Matrix};
use std::collections::HashMap;

type Polygon = (usize, usize, usize);

/*
    vertex
    vertex_ti
    vertex_gi
*/

///
/// Access Wrappers
/// 
struct Vertex {
    pid: usize,
    mid: u128
}
struct Halfedge {
    pid: usize,
    mid: u128
}
struct Edge {
    pid: usize,
    mid: u128
}
struct Face {
    pid: usize,
    mid: u128
}

///
/// Topological Information
/// 
struct VertexTI {
    id: usize,
    he: usize
}
struct HalfedgeTI {
    id: usize,
    tip: usize,
    tail: usize,
    twin: usize,
    next: usize,
    edge: usize,
    face: usize,
}
struct EdgeTI {
    id: usize,
    he: (usize, usize)
}
struct FaceTI {
    id: usize,
    he: usize
}

///
/// Geometric Information
/// 
struct VertexGI {
    id: usize,
    pos: Vector3<f32>,
}

struct Mesh{
    id: usize,
    vertex_data: Vec<VertexGI>,
    vertices: Vec<VertexTI>,
    halfedges: Vec<HalfedgeTI>,
    edges: Vec<EdgeTI>,
    faces: Vec<FaceTI>

}

impl Mesh{

    fn new(points: Vec<Vector3<f32>>, polys: Vec<Polygon>) -> Mesh {
        let id: usize = 1;
        let vertex_data = points.iter().enumerate().map(|(i, v)| VertexGI{id: i, pos: v.clone()}).collect();
        let mut vertices: Vec<VertexTI> = (0..points.len()).map(|i| VertexTI{id:i, he:0}).collect();
        let mut halfedges: Vec<HalfedgeTI> = Vec::new();
        let mut edges: Vec<EdgeTI> = Vec::new();
        let mut faces: Vec<FaceTI> = Vec::new();
        let mut established_edges: HashMap<(usize, usize), usize> = HashMap::new();

        for (i, p) in polys.iter().enumerate() {

            faces.push(FaceTI{id:i, he:halfedges.len()});

            let mut he1 = HalfedgeTI{id: halfedges.len(), tail: p.0, tip: p.1, twin:0, next: halfedges.len()+1, edge:0, face: i};
            let mut he2 = HalfedgeTI{id: halfedges.len()+1, tail: p.1, tip: p.2, twin:0, next: halfedges.len()+2, edge:0, face: i};
            let mut he3 = HalfedgeTI{id: halfedges.len()+2, tail: p.2, tip: p.0, twin:0, next: halfedges.len(), edge:0, face: i};
            
            //note: inefficient. yucky. lazy. 
            vertices[he1.tail].he = he1.id;
            vertices[he2.tail].he = he2.id;
            vertices[he3.tail].he = he3.id;
            

            ///
            /// wet code below. should DRY it up.
            /// 


            //he1 edge 
            if established_edges.contains_key(&(he1.tip, he1.tail)){
                let edge_id = established_edges[&(he1.tip, he1.tail)];
                he1.edge = edge_id;
                edges[edge_id].he.1 = he1.id;
            }else{
                established_edges.insert((he1.tail, he1.tip), edges.len());
                he1.edge = edges.len();
                edges.push(EdgeTI{id: edges.len(), he: (he1.id, 0)});
            }
            halfedges.push(he1);

            //he2 edge 
            if established_edges.contains_key(&(he2.tip, he2.tail)){
                let edge_id = established_edges[&(he2.tip, he2.tail)];
                he2.edge = edge_id;
                edges[edge_id].he.1 = he2.id;
            }else{
                established_edges.insert((he2.tail, he2.tip), edges.len());
                he2.edge = edges.len();
                edges.push(EdgeTI{id: edges.len(), he: (he2.id, 0)});
            }
            halfedges.push(he2);

            //he3 edge 
            if established_edges.contains_key(&(he3.tip, he3.tail)){
                let edge_id = established_edges[&(he3.tip, he3.tail)];
                he3.edge = edge_id;
                edges[edge_id].he.1 = he3.id;
            }else{
                established_edges.insert((he3.tail, he3.tip), edges.len());
                he3.edge = edges.len();
                edges.push(EdgeTI{id: edges.len(), he: (he3.id, 0)});
            }
            halfedges.push(he3);
        }
        
        //this can be done in the other loop...
        for e in &edges {
            halfedges[e.he.0].twin = e.he.1;
            halfedges[e.he.1].twin = e.he.0;
        }


        /*
        */
        Mesh{id,vertex_data,vertices,halfedges,edges,faces}
    }
}