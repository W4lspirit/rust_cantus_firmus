use std::fs::File;
use sdt::io::prelude::*;
use std::io::{Read, BufReader};
use std::ops::Not;
use rand::prelude::*;

const HARMONIC: Vec<i8> = vec![0, 2, 4, 5, 7, 9, 11];
const MELODIC: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 7];

const I_EMPTY: Vec<i8> = vec![];
const EMPTY_HOLY_TREE: Vec<Note> = vec![];

struct Note {
    cantus_firmus: i8,
    contre_chant: i8,
    holy_tree: Vec<Note>,
}

fn main() {
    let init_note = note(0, 0, EMPTY_HOLY_TREE);
    let init_tree = vec![init_note];
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


fn possible_note(c_f: i8, l_c_f: i8) -> Vec<Note> {
    rec_aux(c_f, l_c_f, 11, EMPTY_HOLY_TREE)
}

fn rec_aux(c_f: i8, l_c_f: i8, count: i8, notes: Vec<Note>) -> Vec<Note> {
    return match count {
        3 => notes,
        k if HARMONIC.contains(&((count - c_f).abs())) && MELODIC.contains(&((count - l_c_f).abs())) => rec_aux(c_f, l_c_f, count - 1, append_and_merge(k, c_f, notes)),
        _ => rec_aux(c_f, l_c_f, count - 1, notes)
    };
}

fn append_and_merge(k: i8, c: i8, notes: Vec<Note>) -> Vec<Note> {
    let n = note(k, c, EMPTY_HOLY_TREE);
    return concat_head_and_tail(n, notes);
}

fn magical_aux(cantus: i8, holy_list: Vec<Note>) -> Vec<Note> {
    if holy_list.is_empty() {
        return holy_list;
    }
    // Todo convert Notes(c_prev,c_f,[]) :: tl   Vector may not help...
    return match holy_list.split_at(1) {
        ([Note { cantus_firmus: c_prev, contre_chant: c_f, holy_tree: [] }], b) => {
            let head = note(*c_prev, *c_f, possible_note(cantus, *c_prev));
            concat_head_and_tail(head, magical_aux(cantus, b.into_vec()))
        }
        ([Note { cantus_firmus: c_prev, contre_chant: c_f, holy_tree: l }], b) => {
            let head = note(*c_prev, *c_f, magical_aux(cantus, l.into_vec()));
            concat_head_and_tail(head, magical_aux(cantus, b.into_vec()))
        }
    };
}

fn ajoute(c_f: i8, note: Note) -> Note {
    return match note {
        Note { cantus_firmus: 0, contre_chant: 0, holy_tree: [] } =>
            note(0, c_f, possible_note(c_f, 0)),

        Note { cantus_firmus: cantus_firmus, contre_chant: contre_chant, holy_tree } =>
            note(cantus_firmus, contre_chant, magical_aux(c_f, holy_tree))
    };
}

fn construire_arbre_complet(l: Vec<i8>) -> Vec<Note> {
    return c_a_c_aux(l, note(0, 0, EMPTY_HOLY_TREE));
}

fn c_a_c_aux(l: Vec<i8>, h_tree: Note) -> Vec<Note> {
    return if l.is_empty() {
        vec![h_tree]
    } else {
        let ([n], q) = l.split_at(1);
        aux(q.to_vec(), ajoute(*n, h_tree))
    };
}

fn profondeur_noeud(n: Note) -> i32 {
    return match n {
        Note { cantus_firmus: p, contre_chant: i, holy_tree: [] } => 1,
        Note { cantus_firmus: cantus_firmus, contre_chant: contre_chant, holy_tree } => {
            return profondeur_holy_tree(holy_tree);
        }
    };
}

fn profondeur_holy_tree(h_tree: Vec<Note>) -> i32 {
    return h_tree.into_iter().into_iter()
        .map(|t| profondeur_noeud(t))
        .max()
        .map_or(0, |max| 1 + max);
}

fn holy_purge(profondeur_max: i32, profondeur: i32, h_tree: Vec<Note>, first_time: bool) -> Vec<Note> {
    if h_tree.is_empty() {
        return EMPTY_HOLY_TREE;
    }

    match h_tree.split_at(1) {
        ([Note { cantus_firmus: c_prev, contre_chant: c_f, holy_tree: [] }], tail) => {
            if profondeur < profondeur_max {
                tail.to_vec()
            } else {
                concat_head_and_tail(note(*c_prev, *c_f, EMPTY_HOLY_TREE), tail.to_vec())
            }
        }
        ([Note { cantus_firmus: cantus_firmus, contre_chant: contre_chant, holy_tree }], tail) => {
            if first_time {
                let vec = holy_purge(profondeur_max, profondeur + 1, holy_tree.to_vec(), true);
                let head = note(*cantus_firmus, *contre_chant, vec);
                let h_tree = concat_head_and_tail(head, tail.to_vec());
                holy_purge(profondeur_max, profondeur, h_tree, false)
            } else {
                let tail = holy_purge(profondeur_max, profondeur, tail.to_vec(), true);
                concat_head_and_tail(head, tail)
            }
        }
    }
}

fn parcours_arbre(holy_tree: Vec<Note>) -> Vec<i8> {
    p_a_aux(holy_tree, I_EMPTY)
}

fn p_a_aux(holy_tree: Vec<Note>, res: Vec<i8>) -> Vec<i8> {
    let mut rng = rand::thread_rng(); //Not sure about this code
    let size = holy_tree.len();
    let nth = rng.gen_range(0, size);

    return holy_tree.get(nth)
        .map(|nth_note| {
            return match nth_note {
                Note { cantus_firmus: c_f, contre_chant: c_c, holy_tree: [] } => {
                    let mut rev_res: Vec<i8> = vec![*c_f];
                    rev_res.extend(res.iter());
                    rev_res.reverse();
                    rev_res
                }

                Note { cantus_firmus, contre_chant, holy_tree } => {
                    let mut n_res: Vec<i8> = vec![*c_f];
                    n_res.extend(res.iter());
                    p_a_aux(holy_tree.to_vec(), n_res)
                }
            };
        }).unwrap_or(I_EMPTY);
}
//


//------------------------------------- Utils
fn note(cantus_firmus: i8, contre_chant: i8, holy_tree: Vec<Note>) -> Note {
    return Note { cantus_firmus, contre_chant, holy_tree };
}

fn concat_head_and_tail(head: Note, tail: Vec<Note>) -> Vec<Note> {
    let mut v = vec![head];
    v.extend(tail.iter());
    return v;
}

