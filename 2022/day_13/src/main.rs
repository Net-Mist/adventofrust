use serde_json::Value;
use serde_json::Value::{Array, Number};
use std::cmp::Ordering;

fn data() -> Vec<(Value, Value)> {
    let a = include_str!("../i.txt").split("\n\n").map(|b| {
        b.split_once('\n')
            .map(|(a, b)| {
                (
                    serde_json::from_str(a).unwrap(),
                    serde_json::from_str(b).unwrap(),
                )
            })
            .unwrap()
    });
    a.collect()
}

fn compare_list(vl: &Vec<Value>, vr: &Vec<Value>) -> Option<bool> {
    let s1 = vl.len();
    let s2 = vr.len();
    let s_min = s1.min(s2);

    let mut keep = None;

    for i in 0..s_min {
        match (&vl[i], &vr[i]) {
            (Number(ln), Number(rn)) => {
                match ln.as_f64().unwrap().total_cmp(&rn.as_f64().unwrap()) {
                    Ordering::Greater => return Some(false),
                    Ordering::Less => return Some(true),
                    Ordering::Equal => {}
                }
            }
            (Array(vl), Array(vr)) => keep = compare_list(vl, vr),
            (Array(vl), Number(rn)) => {
                let vr = vec![Value::Number(rn.clone())];
                keep = compare_list(vl, &vr)
            }
            (Number(ln), Array(vr)) => {
                let vl = vec![Value::Number(ln.clone())];
                keep = compare_list(&vl, vr)
            }
            _ => unreachable!(),
        }

        if keep.is_some() {
            return keep;
        }
    }
    if s1 == s2 {
        None
    } else {
        Some(s1 <= s2)
    }
}

fn part1() {
    let d = data();

    let mut s = 0;
    for (i, (l, r)) in d.into_iter().enumerate() {
        let (vl, vr) = if let (Array(vl), Array(vr)) = (l, r) {
            (vl, vr)
        } else {
            unreachable!()
        };

        if let Some(false) = compare_list(&vl, &vr) {
            println!("don't add {}", i + 1);
        } else {
            println!("add {}", i + 1);
            s += i + 1
        }
    }

    println!("{}", s)
}

fn compare(a: &&Value, b: &&Value) -> Ordering {
    let (vl, vr) = if let (Array(vl), Array(vr)) = (a, b) {
        (vl, vr)
    } else {
        unreachable!()
    };

    match compare_list(vl, vr) {
        None => Ordering::Equal,
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    }
}

fn part2() {
    let d = data();
    let mut d2 = Vec::new();
    d.iter().for_each(|(a, b)| {
        d2.push(a);
        d2.push(b);
    });

    let dec1 = Value::Array(vec![Value::Array(vec![Value::Number(
        serde_json::Number::from_f64(2.0).unwrap(),
    )])]);
    let dec2 = Value::Array(vec![Value::Array(vec![Value::Number(
        serde_json::Number::from_f64(6.0).unwrap(),
    )])]);

    d2.push(&dec1);
    d2.push(&dec2);
    d2.sort_by(compare);

    let a = d2.iter().position(|v| **v == dec1).unwrap();
    let b = d2.iter().position(|v| **v == dec2).unwrap();
    println!("{}", (a + 1) * (b + 1));
}

fn main() {
    part1();
    part2();
}
