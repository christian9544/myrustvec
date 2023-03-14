fn scalar(lhs: &Vec<i32>, rhs: &Vec<i32>) -> i32 {
    let mut summe: i32 = 0;
    for i in lhs.iter() {
        for j in rhs.iter() {
            summe += i*j;
        }
    }
    return summe;
}

#[allow(dead_code)]
fn epsilonijk(i: usize, j: usize, k: usize) -> i32 {
    let input = [i, j, k];
    return match input {
        [0,1,2] => 1,
        [2,0,1] => 1,
        [1,2,0] => 1,
        [2,1,0] => - 1,
        [0,2,1] => - 1,
        [1,0,2] => - 1,
        _ => 0,
    };
}

fn permutations_sym (mut input: Vec<usize>) -> Vec<Vec<usize>> {
    let mut erg = vec![input.clone()];
    for _ in 0..input.len()-1 {
        if let Some(last) = input.pop() {
            input.insert(0, last);
            erg.push(input.clone());
        }
    }
    return erg;
}

fn permutations_asym(mut input: Vec<usize>) -> Vec<Vec<usize>> {
    input.reverse();
    return permutations_sym(input);
}

#[allow(dead_code)]
fn eijk(i: usize, j: usize, k: usize) -> i32 {
    let p_sym: Vec<Vec<usize>> = permutations_sym(vec![0,1,2]);
    let p_asym: Vec<Vec<usize>> = permutations_asym(vec![0,1,2]);
    let input: Vec<usize> = vec![i, j, k];
    if p_sym.contains(&input) {
        return 1;
    }
    if p_asym.contains(&input) {
        return -1;
    }
    return 0;
}

fn vprodukt(lhs: Vec<i32>, rhs: Vec<i32>) -> Vec<i32> {
    let mut prod = Vec::new();
    for k in 0..=2 {
        let mut kvalue = 0;
        for (i,iv) in lhs.iter().enumerate() {
            for (j,jv) in rhs.iter().enumerate() {
                // kvalue += epsilonijk(i,j,k) * iv * jv;
                kvalue += eijk(i,j,k) * iv * jv;
            }
        }
        prod.push(kvalue);
    }
    return prod;
}

fn main() {
    println!("mein Vec");
    
    let v1 = vec![1,2,3];
    let v2 = vec![3,2,1];

    for (k,v) in v2.iter().enumerate(){
        println!("k = {}, v = {}", k, v);
    }
    println!("{:?}", v1);
    let erg = scalar(&v1,&v2);
    // println!("{:?}", v1);
    println!("{:?}", erg);
    
    let prod = vprodukt(v1, v2);

    println!("{:?}", prod);

    let v3 = vec![0,1,2];
    let funny = permutations_sym(v3.clone());
    println!("{:?}", funny);
    let funny_asym = permutations_asym(v3);
    println!("{:?}", funny_asym);

}
