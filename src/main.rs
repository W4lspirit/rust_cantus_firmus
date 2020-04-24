use std::fs::File;
use sdt::io::prelude::*;
use std::io::{Read, BufReader};

const HARMONIC: Vec<i8> = Vec![0,2,4,5,7,9,11];
const MELODIC: Vec<i8> = Vec![0,1,2,3,4,5,7];



fn main() {
    let init_node = Node { pitch: 0, intensity: 0, holy_tree: Vec![] };
    let init_tree = Vec![n];
    println!("Hello, world!");
    let path = "partition";
    // will be read from the cmd line
    let partition = read_partition_v2(path);

    convert_partition_to_vec_i8(partition);
}

fn convert_partition_to_vec_i8(partition: String) -> Vec<i8> {
    let v: Vec<&str> = partition.split(',').collect();
    let v_u: Vec<i8> = v.into_iter()
        .map(|s| s.parse::<i8>()?).collect();

    return v_u;
}

fn read_partition_v1(path: &str) -> String {
    let mut partition = File::open(path)?;
    let mut contents = String::new();
    partition.read_to_string(&mut contents)?;
    return contents;
}

fn read_partition_v2(path: &str) -> String {
    let file = File::open(path)?;
    let mut buf_reader =
        BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    return contents;
}

struct Node {
    pitch: i8,
    intensity: i8,
    holy_tree: Vec<Node>,
}


fn possible_node(cantus: i8, last_pitch_from_cc: i8) -> Vec<Node> {
    rec_aux(cantus, last_pitch_from_cc, 0, Vec![])
}

fn rec_aux(c: i8, p: i8, i: i8, notes: Vec<Node>) -> Vec<Node> {
    return match i {
        3 => notes,
        k if HARMONIC.contains(&((i - c).abs())) && MELODIC.contains(&((i - p).abs())) => rec_aux(c, p, i - 1, append_and_merge(k, c, &notes)),
        _ => rec_aux(c, p, i - 1, notes)
    };
}

fn append_and_merge(k: i8, c: i8, notes: &Vec<Node>) -> Vec<Node> {
    let n = Node { pitch: k, intensity: c, holy_tree: Vec![] };
    return Vec![n].append(notes);
}

fn magical_aux(cantus: i8, holy_list: Vec<Node>) {
    // Todo convert Notes(c_prev,c_f,[]) :: tl   Vector may not help...
    return match holy_list {
        [] => Vec![]
    };
}