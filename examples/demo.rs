use std::collections::BTreeSet;

use lo_shu::{reduce_isometry, CheckScalar, Cycles, Permutation, O4};

fn main() {
    let names = [
        // "a",
        "b", "c",
    ];

    let r = Permutation::<O4>::identity().rotate_90();
    let s = Permutation::<O4>::identity().reflect_x();

    let isometry = [
        ("e", Permutation::identity()),
        ("r", r),
        ("r.pow(2)", r.pow(2)),
        ("r.pow(3)", r.pow(3)),
        ("s", s),
        ("sr", s * r),
        ("sr.pow(2)", s * r.pow(2)),
        ("sr.pow(3)", s * r.pow(3)),
    ];

    let input = [
        // vec![
        //     vec![2, 5, 14, 10, 8, 3, 12],
        //     vec![4, 16, 13],
        //     vec![6, 11, 9, 15, 7],
        // ],
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

    let input_ms = names
        .into_iter()
        .zip(input.map(|i| Cycles::<O4>::from_vecs(i).into_permutation()))
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

    let banner = "+---------------------------+---------------------------------------------------------+----------+--------+\n\
                        | EXPRESSION                | PERMUTATION (VALUE)                                     | IS MAGIC | IS NEW |\n\
                        +---------------------------+---------------------------------------------------------+----------+--------+";

    println!("{banner}");
    for (name, perm) in input_ms.iter() {
        let is_magic = perm.check_s().is_some();
        let p = format!("{}", perm.cyclic_notation());
        println!(
            "| {: <25} | {: <55} | {: <8} | {: <6} |",
            name, p, is_magic, ""
        )
    }
    for (name, perm) in isometry.iter() {
        let is_magic = perm.check_s().is_some();
        let p = format!("{}", perm.cyclic_notation());
        println!(
            "| {: <25} | {: <55} | {: <8} | {: <6} |",
            name, p, is_magic, ""
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+");
    for (name, perm) in actions {
        let is_magic = perm.check_s().is_some();
        let p = format!("{}", perm.cyclic_notation());
        println!(
            "| {: <25} | {: <55} | {: <8} | {: <6} |",
            name, p, is_magic, ""
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+");
    for (name, perm) in results {
        let p = format!("{}", perm.cyclic_notation());
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
        if is_new {
            new_ms_set.insert((name.clone(), perm));
        }
        println!(
            "| {: <25} | {: <55} | {: <8} | {: <6} |",
            name, p, is_magic, is_new
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+");
    for (name, perm) in new_ms_set.iter() {
        let p = format!("{}", perm.cyclic_notation());
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;
        println!(
            "| {: <25} | {: <55} | {: <8} | {: <6} |",
            name, p, is_magic, is_new
        )
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+");

    let mut new_ms_isometry_set = BTreeSet::new();
    for (name, perm) in new_ms_set.iter() {
        let p = format!("{}", perm.cyclic_notation());
        let is_magic = perm.check_s().is_some();
        let is_new = !input_ms_isometry_set.contains(&perm) && is_magic;

        if !new_ms_isometry_set.contains(perm) {
            for i in perm.generate_d() {
                new_ms_isometry_set.insert(i);
            }
            println!(
                "| {: <25} | {: <55} | {: <8} | {: <6} |",
                name, p, is_magic, is_new
            )
        }
    }
    println!("+---------------------------+---------------------------------------------------------+----------+--------+");
    println!("Output:");
    for i in unique_result_ms_perms_set {
        println!("{}", i.cyclic_notation())
    }
}
