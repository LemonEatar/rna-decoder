fn main() {
    decode("gccgacgaa");
}

fn decode(dna: &str) {
    const STEP: u32 = 3;
    let dna_vec: Vec<char> = dna.chars().collect();
    let mut amino_vec: Vec<String> = Vec::new();
    if dna.len() % 3 != 0
        || !dna
            .chars()
            .all(|c| c == 'a' || c == 'g' || c == 'c' || c == 'u')
    {
        panic!("Not Valid Dna")
    }
    for index in (0..dna_vec.len()).step_by(STEP as usize) {
        let mut amino_chars: Vec<char> = Vec::new();
        amino_chars.push(dna_vec[index]);
        amino_chars.push(dna_vec[index + 1]);
        amino_chars.push(dna_vec[index + 2]);
        amino_vec.push(match_amino(amino_chars));
    }
    if &amino_vec[0] != "met" {
        println!("First amnio sequenc should always be met")
    } else {
        println!("{:?}", amino_vec)
    }
}

fn match_amino(amino: Vec<char>) -> String {
    let x = match amino.as_slice() {
        ['g', 'c', _val] => "ala",
        ['g', 'a', val] if *val == 'u' || *val == 'c' => "asp",
        ['g', 'a', val] if *val == 'a' || *val == 'g' => "glu",
        ['g', 'g', _val] => "gly",
        ['u', 'u', val] if *val == 'u' || *val == 'c' => "phe",
        ['u', 'u', val] if *val == 'a' || *val == 'g' => "leu",
        ['u', 'c', _val] => "ser",
        ['u', 'a', val] if *val == 'u' || *val == 'c' => "tyr",
        ['a', 'u', 'g'] => "met",
        _ => "leck",
    };
    return x.to_string();
}
