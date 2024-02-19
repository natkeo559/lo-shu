use std::collections::{BTreeSet, HashMap};

use lo_shu::{prelude::*, CheckScalar, Cycles, Permutation, O4};

fn main() {
    let names = ["a", "b"];

    let r = Permutation::<O4>::identity().rotate_90();
    let s = Permutation::<O4>::identity().reflect_x();

    let isometry = BTreeSet::from([
        ("e", Permutation::identity()),
        ("r", r),
        ("r.pow(2)", r.pow(2)),
        ("r.pow(3)", r.pow(3)),
        ("s", s),
        ("sr", s * r),
        ("sr.pow(2)", s * r.pow(2)),
        ("sr.pow(3)", s * r.pow(3)),
    ]);

    let input = [
        vec![
            vec![2, 4, 16, 11, 8, 5, 12],
            vec![3, 13, 6, 14, 7],
            vec![9, 15, 10],
        ],
        vec![
            vec![2, 8, 10],
            vec![3, 14, 9, 13, 16, 6, 15],
            vec![4, 11, 12, 7, 5],
        ],
    ];

    let mut input_ms_isometry_set = BTreeSet::new();

    let input_ms =
        names
            .into_iter()
            .zip(input.map(|i| {
                minimize_permutation_isometry(&Cycles::<O4>::from_vecs(i).into_permutation())
            }))
            .collect::<BTreeSet<_>>();

    for i in input_ms.iter() {
        for j in i.1.generate_d() {
            input_ms_isometry_set.insert(j);
        }
    }

    let mut actions = vec![];
    for &(i_name, i_perm) in input_ms.iter() {
        for &(j_name, j_perm) in input_ms.iter() {
            let mut name = "(".to_string();
            name.push_str(i_name);
            name.push_str(".inv()");
            name.push_str(j_name);
            name.push_str(")");

            let factor = i_perm.inv() * j_perm;
            actions.push((name, factor))
        }
    }

    let mut results = vec![];
    let mut result_ms_perms_set = BTreeSet::new();
    for &(i_name, i_perm) in input_ms.iter() {
        for (j_name, j_perm) in actions.iter() {
            for &(s_name, s_perm) in isometry.iter() {
                let mut name = "(".to_string();
                name.push_str(i_name);
                name.push_str(s_name);
                name.push_str(&j_name);
                name.push_str(")");
                let perm = (i_perm * s_perm) * *j_perm;

                if perm.check_s().is_some() {
                    result_ms_perms_set.insert(perm);
                }
                results.push((name, perm));
            }
        }
    }

    let results_expr_table = results
        .iter()
        .map(|(s, p)| (format!("{}", p.cyclic_notation()), s))
        .collect::<HashMap<String, &String>>();

    let mut unique_results = vec![];
    let unique_result_ms_perms_set = reduce_isometry(&result_ms_perms_set);
    for (name, perm) in results.iter() {
        for up in unique_result_ms_perms_set.iter() {
            if perm == up {
                unique_results.push((name, perm));
            }
        }
    }

    let mut new_ms_set = BTreeSet::new();

    let banner = "+---------------------------+---------------------------------------------------------+----------+--------+--------+\n\
                        | EXPRESSION                | PERMUTATION (VALUE)                                     | IS MAGIC | IS NEW | PARITY |\n\
                        +---------------------------+---------------------------------------------------------+----------+--------+--------+";

    println!("{banner}");

    for set in [input_ms, isometry] {
        for (name, perm) in set.iter() {
            let is_magic = perm.check_s().is_some();
            let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
            let p = format!("{}", perm.cyclic_notation());
            let parity = format!("{}", perm.sign());
            println!(
                "| {:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
                name, p, is_magic, is_new, parity
            );
        }
    }
    let ainv = vec![
            vec![2, 4, 16, 11, 8, 5, 12],
            vec![3, 13, 6, 14, 7],
            vec![9, 15, 10],
        ];
    let perm = Cycles::<O4>::from_vecs(ainv).into_permutation().inv();
    let p = format!("{}", perm.cyclic_notation());
    let is_magic = perm.check_s().is_some();
    let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
    let parity = format!("{}", perm.sign());
    println!(
        "| {:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
        "a.inv()", p, is_magic, is_new, parity
    );

    println!("+---------------------------+---------------------------------------------------------+----------+--------+--------+");
    for (name, perm) in actions.iter() {
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
        let p = format!("{}", perm.cyclic_notation());
        let parity = format!("{}", perm.sign());
        println!(
            "|>{:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
            name, p, is_magic, is_new, parity
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+--------+");
    for (name, perm) in results.iter() {
        let p = format!("{}", perm.cyclic_notation());
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
        if is_new {
            new_ms_set.insert((name.clone(), perm));
        }
        let parity = format!("{}", perm.sign());
        println!(
            "| {:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
            name, p, is_magic, is_new, parity
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+--------+");
    for (name, perm) in new_ms_set.iter() {
        let p = format!("{}", perm.cyclic_notation());
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
        let parity = format!("{}", perm.sign());
        println!(
            "| {:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
            name, p, is_magic, is_new, parity
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+--------+");

    let mut new_ms_isometry_set = BTreeSet::new();
    for (name, perm) in new_ms_set.iter() {
        let p = format!("{}", perm.cyclic_notation());
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;

        if !new_ms_isometry_set.contains(*perm) {
            for i in perm.generate_d() {
                new_ms_isometry_set.insert(i);
            }
            let parity = format!("{}", perm.sign());
            println!(
                "| {:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
                name, p, is_magic, is_new, parity
            )
        }
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+--------+");
    for perm in unique_result_ms_perms_set {
        let min_perm = minimize_permutation_isometry(&perm);
        let p = format!("{}", min_perm.cyclic_notation());
        let is_magic = min_perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&min_perm) && is_magic;
        let name = results_expr_table.get(&p).unwrap();
        let parity = format!("{}", min_perm.sign());
        println!(
            "| {:<25} | {:<55} | {:<8} | {:<6} | {:<6} |",
            name, p, is_magic, is_new, parity
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+--------+");
}
