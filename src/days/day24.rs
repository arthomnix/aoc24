use std::collections::{HashMap, HashSet};
use Op::*;
use Value::*;
use WireRole::*;

fn id_of(s: &str) -> u32 {
    let mut c = s.chars();
    ((c.next().unwrap() as u32) << 16) | ((c.next().unwrap() as u32) << 8) | (c.next().unwrap() as u32)
}

fn id_of_io(n: u32, c: char) -> u32 {
    let d1 = n / 10;
    let d2 = n % 10;
    ((c as u32) << 16) | ((d1 + b'0' as u32) << 8) | (d2 + b'0' as u32)
}

fn id_of_output(n: u32) -> u32 {
    id_of_io(n, 'z')
}

fn id_of_in_x(n: u32) -> u32 {
    id_of_io(n, 'x')
}

fn id_of_in_y(n: u32) -> u32 {
    id_of_io(n, 'y')
}

fn from_id(id: u32) -> String {
    let mut s = String::with_capacity(3);
    s.push((id >> 16) as u8 as char);
    s.push(((id >> 8) & 0xFF) as u8 as char);
    s.push((id & 0xFF) as u8 as char);
    s
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            And => a && b,
            Or => a || b,
            Xor => a ^ b,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Value {
    Literal(bool),
    Operator(u32, Op, u32),
}

fn parse(input: &str) -> HashMap<u32, Value> {
    let mut map = HashMap::new();

    let (v, g) = input.split_once("\n\n").unwrap();
    for l in v.lines() {
        let (id, b) = l.split_once(": ").unwrap();
        let id = id_of(id);
        let v = match b.trim() {
            "0" => false,
            "1" => true,
            _ => panic!("not a bit"),
        };
        map.insert(id, Literal(v));
    }

    for l in g.lines() {
        let mut i = l.split_whitespace();
        let id1 = id_of(i.next().unwrap());
        let gate = match i.next().unwrap() {
            "AND" => And,
            "OR" => Or,
            "XOR" => Xor,
            _ => panic!("invalid gate"),
        };
        let id2 = id_of(i.next().unwrap());
        let id3 = id_of(i.nth(1).unwrap());
        map.insert(id3, Operator(id1.min(id2), gate, id1.max(id2)));
    }

    map
}

fn get(id: u32, map: &mut HashMap<u32, Value>) -> bool {
    let value = map[&id];
    match value {
        Literal(v) => v,
        Operator(l, op, r) => {
            let lv = get(l, map);
            let rv = get(r, map);
            let res = op.apply(lv, rv);
            map.insert(id, Literal(res));
            res
        }
    }
}

pub(crate) fn part1(input: String) {
    let mut map = parse(&input);
    let mut res = 0;
    let mut i = 0;
    while map.contains_key(&id_of_output(i)) {
        res |= (get(id_of_output(i), &mut map) as u64) << i;
        i += 1;
    }
    println!("{res}");
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum WireRole {
    InX,
    InY,
    OutZ,
    Cout,
    XorXY,
    AndXY,
    AndXorXYC,
    Unknown,
}

fn get_roles(map: &HashMap<u32, Value>) -> (HashMap<u32, (WireRole, u32)>, HashMap<(WireRole, u32), u32>, HashSet<u32>) {
    let reversed = map
        .iter()
        .map(|(&a, &b)| (b, a))
        .collect::<HashMap<_, _>>();

    let mut roles = HashMap::new();
    let mut reversed_roles = HashMap::new();
    let mut swaps = HashSet::new();

    let mut n_bits = 0;
    while map.contains_key(&id_of_output(n_bits)) {
        n_bits += 1;
    }

    for i in 0..=n_bits {
        roles.insert(id_of_in_x(i), (InX, i));
        roles.insert(id_of_in_y(i), (InY, i));
        roles.insert(id_of_output(i), (OutZ, i));
        reversed_roles.insert((InX, i), id_of_in_x(i));
        reversed_roles.insert((InY, i), id_of_in_y(i));
        reversed_roles.insert((OutZ, i), id_of_output(i));

        // First output - half adder
        if i == 0 {
            // find Cout of half adder
            if let Some(&cout) = reversed.get(&Operator(id_of_in_x(i), And, id_of_in_y(i))) {
                roles.insert(cout, (Cout, i));
                reversed_roles.insert((Cout, i), cout);
            }
            continue;
        }

        // Full adder
        // Xn AND Yn
        let and_x_y = reversed.get(&Operator(id_of_in_x(i), And, id_of_in_y(i)));
        if let Some(&val) = and_x_y {
            roles.insert(val, (AndXY, i));
            reversed_roles.insert((AndXY, i), val);
        }
        // Xn XOR Yn
        let xor_x_y = reversed.get(&Operator(id_of_in_x(i), Xor, id_of_in_y(i)));
        if let Some(&val) = xor_x_y {
            roles.insert(val, (XorXY, i));
            reversed_roles.insert((XorXY, i), val);
        }

        // Carry out from previous stage
        let cin = reversed_roles.get(&(Cout, i - 1))
            .map(|x| *x)
            .or_else(|| {
                if let Some(Operator(l, Xor, r)) = map.get(&id_of_output(i)) {
                    if xor_x_y == Some(l) {
                        Some(*r)
                    } else {
                        Some(*l)
                    }
                } else {
                    None
                }
            });


        if let Some(cin) = cin {
            if let Some(&val) = xor_x_y {
                let left = cin.min(val);
                let right = cin.max(val);
                let and_xor_x_y_c = reversed.get(&Operator(left, And, right));
                if let Some(&val2) = and_xor_x_y_c {
                    roles.insert(val2, (AndXorXYC, i));
                    reversed_roles.insert((AndXorXYC, i), val2);

                    if let Some(&val3) = and_x_y {
                        let left = val2.min(val3);
                        let right = val2.max(val3);
                        if let Some(&val4) = reversed.get(&Operator(left, Or, right)) {
                            roles.insert(val4, (Cout, i));
                            reversed_roles.insert((Cout, i), val4);
                        }
                    }
                }

                // check output matches
                let output = reversed.get(&Operator(left, Xor, right));
                if let Some(&o) = output {
                    if o != id_of_output(i) {
                        swaps.insert(o);
                        swaps.insert(id_of_output(i));
                    }
                }
            }
        }
    }

    for &v in map.keys() {
        if !roles.contains_key(&v) {
            roles.insert(v, (Unknown, 0));
        }
    }

    (roles, reversed_roles, swaps)
}

fn find_base_unknowns(map: &HashMap<u32, Value>, roles: &HashMap<u32, (WireRole, u32)>, start: u32) -> HashSet<u32> {
    match map[&start] {
        Literal(_) => HashSet::from([start]),
        Operator(l, o, r) => {
            let (rl, _) = roles[&l];
            let (rr, _) = roles[&r];
            if rl != Unknown && rr != Unknown {
                HashSet::from([start])
            } else {
                let mut set = HashSet::new();
                if rl == Unknown {
                    set.extend(find_base_unknowns(map, roles, l));
                }
                if rr == Unknown {
                    set.extend(find_base_unknowns(map, roles, r));
                }
                set
            }
        }
    }
}

pub(crate) fn part2(input: String) {
    let map = parse(&input);
    let (roles, reversed_roles, mut swaps) = get_roles(&map);

    let mut s = HashSet::new();
    for unknown in roles.iter().filter_map(|(&k, v)| if v.0 == Unknown { Some(k) } else { None }) {
        s.extend(find_base_unknowns(&map, &roles, unknown));
    }

    for swap in &swaps {
        s.remove(swap);
    }

    for v in s {
        if let Operator(l, o, r) = map[&v] {
            if roles[&l].0 == AndXY {
                let level = roles[&l].1;
                swaps.insert(l);
                swaps.insert(reversed_roles[&(XorXY, level)]);
            } else if roles[&r].0 == AndXY {
                let level = roles[&r].1;
                swaps.insert(r);
                swaps.insert(reversed_roles[&(XorXY, level)]);
            }
        }
    }

    assert_eq!(swaps.len(), 8);
    let mut v = swaps.into_iter().map(|x| from_id(x)).collect::<Vec<_>>();
    v.sort_unstable();
    println!("{}", v.join(","));
}
