use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
enum HashItem {
    Uv__([f32; 2]),
    Vert([f32; 3]),
    Quat([f32; 4]),
}

impl PartialEq for HashItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HashItem::Uv__(a), HashItem::Uv__(b)) => a == b,
            (HashItem::Vert(a), HashItem::Vert(b)) => a == b,
            (HashItem::Quat(a), HashItem::Quat(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for HashItem {}

impl Hash for HashItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HashItem::Uv__(arr) => {
                arr.iter().for_each(|&x| {
                    x.to_bits().hash(state);
                });
            }
            HashItem::Vert(arr) => {
                arr.iter().for_each(|&x| {
                    x.to_bits().hash(state);
                });
            }
            HashItem::Quat(arr) => {
                arr.iter().for_each(|&x| {
                    x.to_bits().hash(state);
                });
            }
        }
    }
}

// Your existing implementation
#[derive(Debug)]
struct HashCollections {
    data: Vec<f32>,
    map: HashMap<HashItem, usize>,
}

#[derive(Debug)]
enum IndexType {
    Uv__(usize),
    Vert(usize),
    Quat(usize),
}

impl HashCollections {
    fn new() -> Self {
        let mut h = HashCollections {
            data: Vec::new(),
            map: HashMap::new(),
        };

        // 3 uvs, 2 vers, 1 quat
        // assuming there's one unit cube in there somewhere,
        // these will be made useful
        let starter = [-0.5, 0.5, 0.5, -0.5];
        h.data.extend_from_slice(&starter);

        h.map.insert(HashItem::Quat(starter), 0);
        h.map
            .insert(HashItem::Vert([starter[0], starter[1], starter[2]]), 0);
        h.map
            .insert(HashItem::Vert([starter[1], starter[2], starter[3]]), 1);
        h.map.insert(HashItem::Uv__([starter[0], starter[1]]), 0);
        h.map.insert(HashItem::Uv__([starter[1], starter[2]]), 1);
        h.map.insert(HashItem::Uv__([starter[2], starter[3]]), 2);

        eprintln!("data: {:?}", h.data);
        eprintln!("_map: {:?}", h.map);

        h
    }

    fn add_sequence(&mut self, sequence: HashItem) -> IndexType {
        if let Some(&index) = self.map.get(&sequence) {
            return match sequence {
                HashItem::Uv__(_) => IndexType::Uv__(index),
                HashItem::Vert(_) => IndexType::Vert(index),
                HashItem::Quat(_) => IndexType::Quat(index),
            };
        }

        let len = self.data.len();
        let mut payload = self.data[(len - 3)..len].to_vec();

        // it's not in there, lets append and return the slice
        let iterations = match sequence {
            HashItem::Uv__(arr) => {
                let plen = payload.len();
                // [-1a, 1b]
                let one_new = [payload[plen - 1], arr[1]];
                let iter = match &arr {
                    _ if arr == one_new => 1,
                    _ => 2,
                };
                self.data.extend_from_slice(&arr[arr.len()-iter..]);
                payload.extend_from_slice(&arr[arr.len()-iter..]);
                iter
            }
            HashItem::Vert(arr) => {
                let plen = payload.len();
                // [-1a, 1b, 2b]
                let two_new = [payload[plen - 1], arr[1], arr[2]];
                // [-2a, -1a, 2b]
                let one_new = [payload[plen - 2], payload[plen - 1], arr[2]];
                let iter = match &arr {
                    _ if arr == one_new => 1,
                    _ if arr == two_new => 2,
                    _ => 3,
                };
                self.data.extend_from_slice(&arr[arr.len()-iter..]);
                payload.extend_from_slice(&arr[arr.len()-iter..]);
                iter
            }
            HashItem::Quat(arr) => {
                let plen = payload.len();
                // [-1a, 1b, 2b, 3b]
                let thr_new = [payload[plen - 1], arr[1], arr[2], arr[3]];
                // [-2a, -1a, 2b, 3b]
                let two_new = [payload[plen - 2], payload[plen - 1], arr[2], arr[3]];
                // [-3a, -2a, -1a, 3b]
                let one_new = [payload[plen - 3], payload[plen - 2], payload[plen - 1], arr[3]];
                let iter = match &arr {
                    _ if arr == one_new => 1,
                    _ if arr == two_new => 2,
                    _ if arr == thr_new => 3,
                    _ => 4,
                };
                self.data.extend_from_slice(&arr[arr.len()-iter..]);
                payload.extend_from_slice(&arr[arr.len()-iter..]);
                iter
            }
        };

        let plen = payload.len();
        for i in 0..iterations {
            let tmp_quat = HashItem::Quat([
                payload[plen - i - 4],
                payload[plen - i - 3],
                payload[plen - i - 2],
                payload[plen - i - 1],
            ]);
            if self.map.get(&tmp_quat).is_none() {
                self.map.insert(tmp_quat, self.data.len() - i - 4);
            }
            let tmp_vert = HashItem::Vert([
                payload[plen - i - 3],
                payload[plen - i - 2],
                payload[plen - i - 1],
            ]);
            if self.map.get(&tmp_vert).is_none() {
                self.map.insert(tmp_vert, self.data.len() - i - 3);
            }
            let tmp_uv__ = HashItem::Uv__([payload[plen - i - 2], payload[plen - i - 1]]);
            if self.map.get(&tmp_uv__).is_none() {
                self.map.insert(tmp_uv__, self.data.len() - i - 2);
            }
        }

        match sequence {
            HashItem::Uv__(_) => IndexType::Uv__(len),
            HashItem::Vert(_) => IndexType::Vert(len),
            HashItem::Quat(_) => IndexType::Quat(len),
        }
    }

    fn print_data(&self) {
        eprintln!("data[{}]: {:?}", self.data.len(), self.data);
    }

    fn print_map(&self) {
        eprintln!("_map: {:?}", self.map);
    }
}

fn main() {
    let mut collections = HashCollections::new();

    let mut indices = vec![];
    let quat = [1.0, 2.0, 3.0, 4.0];
    let vert = [1.0, 2.0, 3.0];
    let uv = [2.0, 3.0];

    /*
    indices.push(collections.add_sequence(HashItem::Quat(quat)));
    indices.push(collections.add_sequence(HashItem::Vert(vert)));
    indices.push(collections.add_sequence(HashItem::Uv__(uv)));

    indices.push(collections.add_sequence(HashItem::Quat(quat)));
    indices.push(collections.add_sequence(HashItem::Vert([0., 0., 1.])));
    indices.push(collections.add_sequence(HashItem::Uv__([0., 1.])));

    indices.push(collections.add_sequence(HashItem::Quat([0.,0.,1.,2.])));
    indices.push(collections.add_sequence(HashItem::Quat([3.,4.,4.,4.])));
    indices.push(collections.add_sequence(HashItem::Vert([4., 4., 4.])));
    indices.push(collections.add_sequence(HashItem::Uv__([4., 4.])));
    */

    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,-0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([-0.5,-0.5,0.5])));
    indices.push(collections.add_sequence(HashItem::Vert([0.5,-0.5,0.5])));

    /*
    use rand::prelude::*;

    let mut rng = rand::thread_rng();
    for _ in 0..300_000 {
        let hash_type: u8 = rng.gen_range(0..3);
        match hash_type {
            0 => {
                let f1: f32 = rng.gen();
                let f2: f32 = rng.gen();
                indices.push(collections.add_sequence(HashItem::Uv__([f1 as f32, f2 as f32])));
            },
            1 => {
                let f1: f32 = rng.gen();
                let f2: f32 = rng.gen();
                let f3: f32 = rng.gen();
                indices.push(collections.add_sequence(HashItem::Vert([f1 as f32, f2 as f32, f3 as f32])));
            },
            2 => {
                let f1: f32 = rng.gen();
                let f2: f32 = rng.gen();
                let f3: f32 = rng.gen();
                let f4: f32 = rng.gen();
                indices.push(collections.add_sequence(HashItem::Quat([f1 as f32, f2 as f32, f3 as f32, f4 as f32])));
            },
            _ => panic!("bad hash type")
        }
    }
    */

    eprintln!("===================");
    collections.print_data();
    eprintln!("indices[{}]: {:?}", indices.len(), indices);
}
