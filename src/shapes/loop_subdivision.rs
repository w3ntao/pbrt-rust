use crate::pbrt::*;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct VertexID {
    id: usize,
}

static VERTEX_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn generate_vertex_id() -> VertexID {
    return VertexID {
        id: VERTEX_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
    };
}

struct SDVertex {
    pub id: VertexID,
    pub p: Point3f,
    pub start_face: Option<Arc<Mutex<SDFace>>>,
    pub child: Option<Arc<Mutex<SDVertex>>>,
    pub regular: bool,
    pub boundary: bool,
}

impl Default for SDVertex {
    fn default() -> Self {
        return SDVertex {
            id: generate_vertex_id(),
            p: Point3f::new(0.0, 0.0, 0.0),
            start_face: None,
            child: None,
            regular: false,
            boundary: false,
        };
    }
}

impl SDVertex {
    pub fn new(p: Point3f) -> Self {
        return Self {
            id: generate_vertex_id(),
            p,
            start_face: None,
            child: None,
            regular: false,
            boundary: false,
        };
    }
}

fn valence(vertex: &Arc<Mutex<SDVertex>>) -> usize {
    let start_face = vertex.lock().unwrap().start_face.clone().unwrap();
    let self_id = vertex.lock().unwrap().id;

    if !vertex.lock().unwrap().boundary {
        let mut f = start_face.clone();

        let mut nf = 1;
        loop {
            let next_f = f.lock().unwrap().next_face(self_id).unwrap().clone();
            if Arc::ptr_eq(&next_f, &start_face) {
                break;
            }

            f = next_f;
            nf += 1;
        }
        return nf;
    }

    let mut f = start_face.clone();
    let mut nf = 1;

    loop {
        let next_f = f.lock().unwrap().next_face(self_id);

        f = match next_f {
            None => {
                break;
            }
            Some(content) => content.clone(),
        };

        nf += 1;
    }

    f = start_face.clone();
    loop {
        let prev_face = f.lock().unwrap().prev_face(self_id);
        f = match prev_face {
            None => {
                break;
            }
            Some(content) => content.clone(),
        };
        nf += 1;
    }

    return nf + 1;
}

fn one_ring(vertex: &Arc<Mutex<SDVertex>>, p: &mut Vec<Point3f>) {
    let start_face = vertex.lock().unwrap().start_face.clone().unwrap().clone();
    let vertex_id = vertex.lock().unwrap().id;

    if !vertex.lock().unwrap().boundary {
        let mut face = start_face.clone();
        let mut idx = 0;

        loop {
            p[idx] = face.lock().unwrap().next_vert(vertex_id).lock().unwrap().p;
            idx += 1;
            let next_face = face.lock().unwrap().next_face(vertex_id).unwrap();
            face = next_face;

            if Arc::ptr_eq(&face, &start_face) {
                break;
            }
        }
        return;
    }

    let mut face = start_face.clone();
    let mut idx = 0;
    loop {
        let f2 = match face.lock().unwrap().next_face(vertex_id) {
            None => {
                break;
            }
            Some(_face) => _face.clone(),
        };
        face = f2;
    }
    p[idx] = face.lock().unwrap().next_vert(vertex_id).lock().unwrap().p;
    idx += 1;

    loop {
        p[idx] = face.lock().unwrap().prev_vert(vertex_id).lock().unwrap().p;
        idx += 1;

        let prev_face = face.lock().unwrap().prev_face(vertex_id);
        face = match prev_face {
            None => {
                break;
            }
            Some(_face) => _face.clone(),
        };
    }
}

fn weight_boundary(vertex: &Arc<Mutex<SDVertex>>, beta: f64) -> Point3f {
    let valence = valence(vertex);
    let mut p_ring = vec![Point3f::new(0.0, 0.0, 0.0); valence];

    one_ring(vertex, &mut p_ring);

    let mut p = (1.0 - 2.0 * beta) * vertex.lock().unwrap().p;
    p += beta * p_ring[0];
    p += beta * p_ring[valence - 1];

    return p;
}

fn weight_one_ring(vertex: &Arc<Mutex<SDVertex>>, beta: f64) -> Point3f {
    let valence = valence(vertex);
    let mut p_ring = vec![Point3f::new(0.0, 0.0, 0.0); valence];

    one_ring(vertex, &mut p_ring);

    let mut p = (1.0 - (valence as f64) * beta) * vertex.lock().unwrap().p;
    for idx in 0..valence {
        p += beta * p_ring[idx];
    }

    return p;
}

fn beta(valence: usize) -> f64 {
    return if valence == 3 {
        3.0 / 16.0
    } else {
        3.0 / (8.0 * (valence as f64))
    };
}

fn loop_gamma(valence: usize) -> f64 {
    return 1.0 / ((valence as f64) + 3.0 / (8.0 * beta(valence)));
}

struct SDFace {
    pub v: [Option<Arc<Mutex<SDVertex>>>; 3],
    pub f: [Option<Arc<Mutex<SDFace>>>; 3],
    pub children: [Option<Arc<Mutex<SDFace>>>; 4],
}

impl Default for SDFace {
    fn default() -> Self {
        return SDFace {
            v: [None, None, None],
            f: [None, None, None],
            children: [None, None, None, None],
        };
    }
}

impl SDFace {
    pub fn set_f(&mut self, idx: usize, val: Option<Arc<Mutex<SDFace>>>) {
        self.f[idx] = val;
    }
}

fn next(idx: usize) -> usize {
    return (idx + 1) % 3;
}

fn prev(idx: usize) -> usize {
    return (idx + 2) % 3;
}

impl SDFace {
    fn vnum(&self, vertex_id: VertexID) -> usize {
        for idx in 0..3 {
            if self.v[idx].clone().unwrap().lock().unwrap().id == vertex_id {
                return idx;
            }
        }

        panic!("vnum(): logic error");
    }

    pub fn next_face(&self, _vertex_id: VertexID) -> Option<Arc<Mutex<SDFace>>> {
        return self.f[self.vnum(_vertex_id)].clone();
    }

    pub fn prev_face(&self, _vertex_id: VertexID) -> Option<Arc<Mutex<SDFace>>> {
        return self.f[prev(self.vnum(_vertex_id))].clone();
    }

    pub fn next_vert(&self, _vertex_id: VertexID) -> Arc<Mutex<SDVertex>> {
        match &self.v[next(self.vnum(_vertex_id))] {
            None => {
                panic!("next_vert(): logic error")
            }
            Some(vertex) => {
                return vertex.clone();
            }
        };
    }

    pub fn prev_vert(&self, _vertex_id: VertexID) -> Arc<Mutex<SDVertex>> {
        match &self.v[prev(self.vnum(_vertex_id))] {
            None => {
                panic!("prev_vert(): logic error")
            }
            Some(vertex) => {
                return vertex.clone();
            }
        };
    }

    pub fn other_vert(&self, v0: VertexID, v1: VertexID) -> Arc<Mutex<SDVertex>> {
        for optional_vertex in &self.v {
            let vertex = match optional_vertex {
                None => {
                    panic!("other_vert(): logic error");
                }
                Some(ptr) => ptr,
            };

            let vertex_id = vertex.lock().unwrap().id;
            if v0 != vertex_id && v1 != vertex_id {
                return vertex.clone();
            }
        }
        panic!("other_vert(): logic error");
    }
}

#[derive(Clone)]
struct SDEdge {
    pub v: [Arc<Mutex<SDVertex>>; 2],
    pub f: [Option<Arc<Mutex<SDFace>>>; 2],
    pub f0_edge_num: i32,
}

impl PartialEq<Self> for SDEdge {
    fn eq(&self, other: &Self) -> bool {
        for idx in 0..self.v.len() {
            if !Arc::ptr_eq(&(self.v[idx]), &(other.v[idx])) {
                return false;
            }
        }
        return true;
    }
}

impl Eq for SDEdge {}

impl Hash for SDEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in &self.v {
            Arc::as_ptr(v).hash(state);
        }
    }
}

impl SDEdge {
    pub fn new(v0: Arc<Mutex<SDVertex>>, v1: Arc<Mutex<SDVertex>>) -> SDEdge {
        return SDEdge {
            v: if Arc::as_ptr(&v0) < Arc::as_ptr(&v1) {
                [v0, v1]
            } else {
                [v1, v0]
            },
            f: [None, None],
            f0_edge_num: -1,
        };
    }
}

pub fn loop_subdivide(
    render_from_object: &Transform,
    n_levels: usize,
    vertex_indices: Vec<usize>,
    p: Vec<Point3f>,
) -> Vec<Arc<Triangle>> {
    let mut vertices: Vec<Arc<Mutex<SDVertex>>> = vec![];
    for i in 0..p.len() {
        vertices.push(Arc::new(Mutex::new(SDVertex::new(p[i]))));
    }

    let n_faces = vertex_indices.len() / 3;
    let mut faces = vec![];
    for _ in 0..n_faces {
        faces.push(Arc::new(Mutex::new(SDFace::default())));
    }

    // Set face to vertex pointers
    for i in 0..n_faces {
        let f = &mut faces[i];

        for j in 0..3 {
            let v = vertices[vertex_indices[i * 3 + j]].clone();
            f.lock().unwrap().v[j] = Some(v.clone());
            v.lock().unwrap().start_face = Some(f.clone());
        }
    }

    let mut edges: HashSet<SDEdge> = HashSet::new();
    for i in 0..n_faces {
        let f = &mut faces[i];

        for edge_num in 0..3 {
            let v0 = f.lock().unwrap().v[edge_num].clone().unwrap();
            let v1 = f.lock().unwrap().v[next(edge_num)].clone().unwrap();

            let e = &mut SDEdge::new(v0.clone(), v1.clone());
            if !edges.contains(&e) {
                e.f[0] = Some(f.clone());
                e.f0_edge_num = edge_num as i32;
                edges.insert(e.clone());
            } else {
                let found_edge = edges.get(&e).unwrap();

                found_edge.f[0]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .set_f(found_edge.f0_edge_num as usize, Some(f.clone()));

                f.lock().unwrap().set_f(edge_num, found_edge.f[0].clone());
                edges.remove(e);
            }
        }
    }

    // Finish vertex initialization
    for i in 0..p.len() {
        let v = &vertices[i];

        let start_face = v.lock().unwrap().start_face.clone().unwrap().clone();

        let mut f = Some(start_face.clone());
        v.lock().unwrap().boundary = false;
        let v_id = v.lock().unwrap().id;

        loop {
            f = f.unwrap().lock().unwrap().next_face(v_id);

            match f {
                None => {
                    break;
                }
                Some(ref _face) => {
                    if Arc::ptr_eq(_face, &start_face) {
                        break;
                    }
                }
            }
        }

        v.lock().unwrap().boundary = f.is_none();

        v.lock().unwrap().regular = {
            let boundary = v.lock().unwrap().boundary;
            let valence = valence(v);

            if !boundary && valence == 6 {
                true
            } else if boundary && valence == 4 {
                true
            } else {
                false
            }
        };
    }

    // Refine _LoopSubdiv_ into triangles
    let mut f = faces.clone();
    let mut v = vertices.clone();

    for _ in 0..n_levels {
        // Update _f_ and _v_ for next level of subdivision
        let mut new_vertices: Vec<Arc<Mutex<SDVertex>>> = vec![];
        for vertex in &mut v {
            let mut child = SDVertex::default();
            child.regular = vertex.lock().unwrap().regular;
            child.boundary = vertex.lock().unwrap().boundary;

            let child_ptr = Arc::new(Mutex::new(child));
            vertex.lock().unwrap().child = Some(child_ptr.clone());
            new_vertices.push(child_ptr.clone());
        }

        let mut new_faces: Vec<Arc<Mutex<SDFace>>> = vec![];
        for face in &mut f {
            for k in 0..4 {
                let _new_face = Arc::new(Mutex::new(SDFace::default()));
                face.lock().unwrap().children[k] = Some(_new_face.clone());
                new_faces.push(_new_face.clone());
            }
        }

        // Update vertex positions and create new edge vertices
        // Update vertex positions for even vertices
        for vertex in &mut v {
            let p = if !vertex.lock().unwrap().boundary {
                if vertex.lock().unwrap().regular {
                    weight_one_ring(vertex, 1.0 / 16.0)
                } else {
                    weight_one_ring(vertex, beta(valence(vertex)))
                }
            } else {
                // Apply boundary rule for even vertex
                weight_boundary(vertex, 1.0 / 8.0)
            };
            vertex
                .lock()
                .unwrap()
                .child
                .clone()
                .unwrap()
                .lock()
                .unwrap()
                .p = p;
        }

        // Compute new odd edge vertices
        let mut edge_verts: HashMap<SDEdge, Arc<Mutex<SDVertex>>> = HashMap::default();
        for face in &f {
            for k in 0..3 {
                let face_v_k = face.lock().unwrap().v[k].clone().unwrap();
                let face_v_next_k = face.lock().unwrap().v[next(k)].clone().unwrap();

                let edge = SDEdge::new(face_v_k, face_v_next_k);
                if !edge_verts.get(&edge).is_none() {
                    continue;
                }

                let vert = Arc::new(Mutex::new(SDVertex::default()));
                new_vertices.push(vert.clone());
                vert.lock().unwrap().regular = true;
                vert.lock().unwrap().boundary = face.lock().unwrap().f[k].is_none();
                vert.lock().unwrap().start_face = face.lock().unwrap().children[3].clone();

                vert.lock().unwrap().p = if vert.lock().unwrap().boundary {
                    0.5 * edge.v[0].lock().unwrap().p + 0.5 * edge.v[1].lock().unwrap().p
                } else {
                    let edge_v0_id = edge.v[0].lock().unwrap().id;
                    let edge_v1_id = edge.v[1].lock().unwrap().id;

                    let mut p = 3.0 / 8.0 * edge.v[0].lock().unwrap().p;

                    p += 3.0 / 8.0 * edge.v[1].lock().unwrap().p;

                    p += 1.0 / 8.0
                        * face
                            .lock()
                            .unwrap()
                            .other_vert(edge_v0_id, edge_v1_id)
                            .lock()
                            .unwrap()
                            .p;

                    p += 1.0 / 8.0
                        * face.lock().unwrap().f[k]
                            .clone()
                            .unwrap()
                            .lock()
                            .unwrap()
                            .other_vert(edge_v0_id, edge_v1_id)
                            .lock()
                            .unwrap()
                            .p;
                    p
                };
                edge_verts.insert(edge, vert.clone());
            }
        }

        // Update new mesh topology
        // Update even vertex face pointers
        for vertex in &v {
            let vertex_id = vertex.lock().unwrap().id;
            let start_face = vertex.lock().unwrap().start_face.clone().unwrap().clone();
            let vert_num = start_face.clone().lock().unwrap().vnum(vertex_id);

            vertex
                .lock()
                .unwrap()
                .child
                .clone()
                .unwrap()
                .lock()
                .unwrap()
                .start_face = start_face.lock().unwrap().children[vert_num].clone();
        }

        // Update face neighbor pointers
        for face in &f {
            for j in 0..3 {
                // Update children _f_ pointers for siblings

                let face_children_next_j = face.lock().unwrap().children[next(j)].clone();
                face.lock().unwrap().children[3]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .f[j] = face_children_next_j;

                let face_children_3 = face.lock().unwrap().children[3].clone();
                face.lock().unwrap().children[j]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .f[next(j)] = face_children_3;

                // Update children _f_ pointers for neighbor children
                let f2 = face.lock().unwrap().f[j].clone();
                let result = match f2.clone() {
                    None => None,
                    Some(_f2) => {
                        let id = face.lock().unwrap().v[j]
                            .clone()
                            .unwrap()
                            .lock()
                            .unwrap()
                            .id;
                        let f2_vnum = _f2.lock().unwrap().vnum(id);
                        _f2.lock().unwrap().children[f2_vnum].clone()
                    }
                };

                face.lock().unwrap().children[j]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .f[j] = result;

                let f3 = face.lock().unwrap().f[prev(j)].clone();
                let result = match f3.clone() {
                    None => None,
                    Some(_f3) => {
                        let id = face.lock().unwrap().v[j]
                            .clone()
                            .unwrap()
                            .lock()
                            .unwrap()
                            .id;
                        let f3_vnum = _f3.lock().unwrap().vnum(id);
                        _f3.lock().unwrap().children[f3_vnum].clone()
                    }
                };

                face.lock().unwrap().children[j]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .f[prev(j)] = result;
            }
        }

        // Update face vertex pointers
        for face in &f {
            for j in 0..3 {
                // Update child vertex pointer to new even vertex
                let face_vj_child = face.lock().unwrap().v[j]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .child
                    .clone();

                face.lock().unwrap().children[j]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .v[j] = face_vj_child;

                // Update child vertex pointer to new odd vertex
                let edge = {
                    let v0 = face.lock().unwrap().v[j].clone().unwrap().clone();
                    let v1 = face.lock().unwrap().v[next(j)].clone().unwrap().clone();
                    SDEdge::new(v0, v1)
                };
                let vert = edge_verts.get(&edge).clone().cloned();
                //.cloned() to turn Option<&Arc<...>> into Option<Arc<...>>
                face.lock().unwrap().children[j]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .v[next(j)] = vert.clone();

                face.lock().unwrap().children[next(j)]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .v[j] = vert.clone();

                face.lock().unwrap().children[3]
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .v[j] = vert.clone();
            }
        }

        // Prepare for next level of subdivision
        f.clear();
        for _face in &new_faces {
            f.push(_face.clone());
        }

        v.clear();
        for _new_vertex in &new_vertices {
            v.push(_new_vertex.clone());
        }
    }

    // Push vertices to limit surface
    let mut p_limit = vec![Point3f::new(f64::NAN, f64::NAN, f64::NAN); v.len()];
    for i in 0..v.len() {
        p_limit[i] = if v[i].lock().unwrap().boundary {
            weight_boundary(&v[i].clone(), 1.0 / 5.0)
        } else {
            weight_boundary(&v[i], loop_gamma(valence(&v[i].clone())))
        }
    }
    for i in 0..v.len() {
        v[i].lock().unwrap().p = p_limit[i];
    }

    // Create triangle mesh from subdivision mesh
    let triangle_num = f.len();
    let mut mesh_vertex_indicies = vec![0; triangle_num * 3];
    let mut used_verts: HashMap<VertexID, usize> = HashMap::default();
    for i in 0..v.len() {
        used_verts.insert(v[i].lock().unwrap().id, i);
    }

    let mut idx = 0;
    for i in 0..triangle_num {
        for j in 0..3 {
            //meshVertexIndices[idx] = usedVerts[f[i]->v[j]->vertex_id];
            let key = f[i].lock().unwrap().v[j]
                .clone()
                .unwrap()
                .lock()
                .unwrap()
                .id;

            mesh_vertex_indicies[idx] = *used_verts.get(&key).unwrap();
            idx += 1;
        }
    }

    // TODO: read and parse Normal3f

    let mesh = TriangleMesh::new(
        render_from_object,
        p_limit,
        mesh_vertex_indicies,
        vec![],
        vec![],
    );
    return mesh.create_triangles();
}
