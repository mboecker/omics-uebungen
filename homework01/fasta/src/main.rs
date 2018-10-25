use rand;
use rand::Rng;
use rayon::prelude::*;

fn reverse_kmer(kmer: impl DoubleEndedIterator<Item=char>) -> impl Iterator<Item=char> {
    kmer.rev().map(reverse)
}

fn reverse(b: char) -> char {
    match b {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => unreachable!(),
    }
}

/// Return an array of other bases thatn the given one
fn invert_values(b: u8) -> [char; 3] {
    match b.into() {
        'A' => ['C', 'G', 'T'],
        'C' => ['A', 'G', 'T'],
        'G' => ['A', 'C', 'T'],
        'T' => ['A', 'C', 'G'],
        _ => unreachable!(),
    }
}

// Replace a base with a randomly selected one
fn invert(b: u8, mut rng: impl Rng) -> u8 {
    let v = invert_values(b);
    *rng.choose(&v).unwrap() as u8
}

#[test]
fn test_invert() {
    assert!(!invert_values('A' as u8).contains(&'A'));
    assert!(!invert_values('C' as u8).contains(&'C'));
    assert!(!invert_values('G' as u8).contains(&'G'));
    assert!(!invert_values('T' as u8).contains(&'T'));
}

fn main() {
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::io::BufRead;
    use std::iter::Iterator;

    // Read sequence from FASTA file from stdin
    let seq: String = {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let s = stdin
            .lines()
            .skip(1) // Skip first line with header
            .map(Result::unwrap)
            .take_while(|x| !x.starts_with('>')) // Read sequence until next header
            .fold(String::new(), |mut x, y| {
                x.push_str(&y);
                x
            });
        s
    };

    // Fold sequence into counts for each base
    let amounts = seq.as_bytes().iter().fold([0usize; 256], |mut x, b| {
        x[*b as usize] += 1;
        x
    });

    // Length of q-mer
    const Q: usize = 17;

    // Coverage
    const C: usize = 30;

    let bytes_seq: &[u8] = seq.as_ref();

    // Record the correct q-grams for later use
    let correct_qgrams: Vec<String> = bytes_seq
        .windows(Q)
        .map(|window| String::from_utf8_lossy(window).into())
        .collect();

    // Try different p values.
    let fr: BTreeMap<_, _> = (1..101)
        .into_par_iter()
        .map(|p| {
            // Contains the count for every q-mer
            let mut hm: HashMap<String, usize> = HashMap::new();

            let mut rng = rand::thread_rng();

            // Do C times...
            for i in (0..C).into_iter() {
                eprintln!("p = {}, i = {}/{}", p, i + 1, C);

                // Iterate over seq and flip every base with p = 0.01
                // This copies the sequence.
                let copy: Vec<u8> = bytes_seq
                    .iter()
                    .map(|b| {
                        if rng.gen_bool(0.01) {
                            invert(*b, &mut rng)
                        } else {
                            *b
                        }
                    })
                    .collect();

                // Slide window over our copy
                for window in copy.windows(Q).into_iter() {

                    // Construct String from the q-mer
                    let qmer = String::from_utf8_lossy(window);
                    let reversed_qmer: String = reverse_kmer(qmer.chars()).collect();

                    //let selected = true;  // no sampling
                    //let selected = rng.gen_bool(0.05); // sample 5% of q-mers
                    //let selected = rng.gen_bool(0.10);  // sample 10% of q-mers
                    //let selected = rng.gen_bool(0.12);  // sample 12% of q-mers
                    let selected = rng.gen_bool(p as f64 / 100f64);

                    if selected {
                        // Increase counter or init counter with 1
                        if hm.contains_key(qmer.as_ref()) {
                            let x = hm.get_mut(qmer.as_ref()).unwrap();
                            *x = *x + 1;
                        } else {
                            hm.insert(qmer.into(), 1);
                        }
                    }
                }
            }

            // Correct genomes, which were also recorded into hm
            let genome_qgrams_in_hm = correct_qgrams
                .iter()
                .map(|x| hm.contains_key(x))
                .map(|b| b as usize)
                .fold(0, |x, y| x + y);

            // Correct genomes, which were never recorded into hm
            let missing_qgrams = correct_qgrams
                .iter()
                .map(|x| !hm.contains_key(x))
                .map(|b| b as usize)
                .fold(0, |x, y| x + y);

            // Incorrect genomes, which were nevertheless recorded into hm
            let errornous_qgrams_in_hm = hm.len() - genome_qgrams_in_hm;

            // New sorted Map, which contains the histogram
            let mut inverse = BTreeMap::new();

            for (_key, value) in &hm {
                // Increase count for value or init it with 1
                if inverse.contains_key(&value) {
                    let x = inverse.get_mut(&value).unwrap();
                    *x = *x + 1;
                } else {
                    inverse.insert(value, 1);
                }
            }

            let fnr = 100f64 * missing_qgrams as f64 / correct_qgrams.len() as f64;
            let fpr = 100f64 * errornous_qgrams_in_hm as f64 / hm.len() as f64;

            (p, (fnr, fpr))
        })
        .collect();

    for (c, v) in amounts.iter().enumerate().filter(|(_, &v)| v > 0) {
        println!("{}: {}", char::from(c as u8), v);
    }

    println!(
        "GC content: {:.2}",
        (amounts['G' as usize] + amounts['C' as usize]) as f64 / seq.len() as f64
    );

    // println!(
    //     "Genomische Q-grams: {} ({:.2}%)",
    //     genome_qgrams_in_hm,
    //     (100f64 * genome_qgrams_in_hm as f64 / hm.len() as f64)
    // );
    // println!(
    //     "Fehlende korrekte Q-gramme (FNR): {} ({:.2}%)",
    //     missing_qgrams,
    //     100f64 * missing_qgrams as f64 / correct_qgrams.len() as f64
    // );
    // println!(
    //     "Fehlerhafte Q-grams: {} (FPR) ({:.2}%)",
    //     errornous_qgrams_in_hm,
    //     (100f64 * errornous_qgrams_in_hm as f64 / hm.len() as f64)
    // );

    // Output as CSV
    // for (key, value) in inverse {
    //     println!("{}, {}", key, value);
    // }

    // Output error rates
    for (p, (fnr, fpr)) in fr {
        println!("{}, {}, {}", p, fnr, fpr);
    }
}
