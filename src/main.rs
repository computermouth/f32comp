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

impl HashCollections {
    fn new() -> Self {
        let mut h = HashCollections {
            data: Vec::new(),
            map: HashMap::new(),
        };

        // 3 uvs, 2 vers, 1 quat
        let starter = [1.,1.,0.,0.];
        h.data.extend_from_slice(&starter);

        h.map.insert(HashItem::Quat(starter), 0);
        h.map.insert(HashItem::Vert([starter[0],starter[1],starter[2]]), 0);
        h.map.insert(HashItem::Vert([starter[1],starter[2],starter[3]]), 1);
        h.map.insert(HashItem::Uv__([starter[0],starter[1]]), 0);
        h.map.insert(HashItem::Uv__([starter[1],starter[2]]), 1);
        h.map.insert(HashItem::Uv__([starter[2],starter[3]]), 2);

        eprintln!("data: {:?}", h.data);
        eprintln!("_map: {:?}", h.map);

        h
    }

    fn add_sequence(&mut self, sequence: HashItem) -> usize {
        if let Some(&index) = self.map.get(&sequence) {
            return index;
        }

        let len = self.data.len();
        let mut payload = self.data[(len - 3)..len].to_vec();


        // it's not in there, lets append and return the slice
        let iterations = match sequence {
            HashItem::Uv__(arr) => {
                self.data.extend_from_slice(&arr);
                payload.extend_from_slice(&arr);
                2
            },
            HashItem::Vert(arr) => {
                self.data.extend_from_slice(&arr);
                payload.extend_from_slice(&arr);
                3
            },
            HashItem::Quat(arr) => {
                self.data.extend_from_slice(&arr);
                payload.extend_from_slice(&arr);
                4
            },
        };

        let plen = payload.len();
        for i in 0..iterations {
            let tmp_quat = HashItem::Quat([payload[plen - i - 4],payload[plen - i - 3],payload[plen - i - 2],payload[plen - i - 1]]);
            if let None = self.map.get(&tmp_quat) {
                self.map.insert(tmp_quat, self.data.len() - 4);
            }
            let tmp_vert = HashItem::Vert([payload[plen - i - 3],payload[plen - i - 2],payload[plen - i - 1]]);
            if let None = self.map.get(&tmp_vert) {
                self.map.insert(tmp_vert, self.data.len() - 3);
            }
            let tmp_uv__ = HashItem::Uv__([payload[plen - i - 2],payload[plen - i - 1]]);
            if let None = self.map.get(&tmp_uv__) {
                self.map.insert(tmp_uv__, self.data.len() - 2);
            }

            self.print_collection();
        }

        len
    }

    fn print_collection(&self) {
        eprintln!("data: {:?}", self.data);
        eprintln!("_map: {:?}", self.map);
    }

    fn add_vert(&mut self, vert: [f32; 3]) -> usize {
        self.add_sequence(HashItem::Vert(vert))
    }

    fn add_uv(&mut self, uv: [f32; 2]) -> usize {
        self.add_sequence(HashItem::Uv__(uv))
    }

    fn add_quat(&mut self, quat: [f32; 4]) -> usize {
        self.add_sequence(HashItem::Quat(quat))
    }

    fn get_vert(&self, index: usize) -> Option<[f32; 3]> {
        if index + 3 <= self.data.len() {
            Some([self.data[index], self.data[index + 1], self.data[index + 2]])
        } else {
            None
        }
    }

    fn get_uv(&self, index: usize) -> Option<[f32; 2]> {
        if index + 2 <= self.data.len() {
            Some([self.data[index], self.data[index + 1]])
        } else {
            None
        }
    }

    fn get_quat(&self, index: usize) -> Option<[f32; 4]> {
        if index + 4 <= self.data.len() {
            Some([
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ])
        } else {
            None
        }
    }
}

fn main() {
    let mut collections = HashCollections::new();

    let quat = [1.0, 2.0, 3.0, 4.0];
    let vert = [1.0, 2.0, 3.0];
    let uv = [2.0, 3.0];

    let quat_index = collections.add_quat(quat);
    let vert_index = collections.add_vert(vert);
    let uv_index = collections.add_uv(uv);

    println!("Quat index: {}", quat_index);
    println!("Vert index: {}", vert_index);
    println!("UV index: {}", uv_index);

    let quat_index = collections.add_quat(quat);
    let vert_index = collections.add_vert([0.,0.,1.]);
    let uv_index = collections.add_uv([0.,1.]);

    println!("Quat index: {}", quat_index);
    println!("Vert index: {}", vert_index);
    println!("UV__ index: {}", uv_index);


    // println!("Quat at index {}: {:?}", quat_index, collections.get_quat(quat_index));
    // println!("Quat at index {}: {:?}", quat_index, collections.get_quat(4));
    // println!("Vert at index {}: {:?}", vert_index, collections.get_vert(5));
    for i in 0..7 {
        println!("UV at index {}: {:?}", i, collections.get_uv(i));
    }

    collections.print_collection();
}
