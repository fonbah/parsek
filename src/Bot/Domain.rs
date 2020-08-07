use rayon::prelude::*;

fn vec_trim (v1: &[String], v2: &[String]) -> (usize, usize) {
    let mut lft: usize = 0;
    let mut rgt: usize = 0;
    
    let v: Vec<_> = v1.iter().zip(v2.iter()).collect();
    
    for (a, b) in v.iter() {
        if a == b { lft += 1;} else { break; }
    }
    
    let vr: Vec<_> = v1.iter().rev().zip(v2.iter().rev()).collect();
    
    for (a, b) in vr.iter() {
        if a == b { rgt += 1;} else { break; }
    }
    
    (lft, rgt)
}

pub fn parse (pages_lines: Vec<Vec<String>>) -> Vec<String> {

    let tailed: Vec<(usize, usize, &Vec<String>)> = pages_lines.par_iter().map(|lines| {
        let (lft, rgt) = pages_lines.iter().fold((0, 0), |lr, item| {
            if lines == item { return lr; }
            let (l, r) = vec_trim(lines.as_slice(), item.as_slice());
            let (cl, cr) = lr;
            
            let nl = match l > cl {
                true => l,
                false => cl,
            };
            
            let nr = match r > cr {
                true => r,
                false => cr,
            };

            (nl, nr)
        });
        (lft, rgt, lines)
    }).collect();
    
    let results: Vec<String> = tailed.into_iter().map(move |(l, rgt, lines)| {
        let lft = match l < lines.len() - rgt {
            true => l,
            false => 0,
        };
        let result: Vec<String> = lines[lft..lines.len() - rgt].into_iter().map(move |line| {
            let line_splitted: Vec<&str> = line.trim().split_whitespace().collect();
            line_splitted.join(" ")
        }).collect();
        result.join(" ")
    }).collect();
    results
}
