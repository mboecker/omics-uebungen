fn read_seq() -> String {
    use std::io::BufRead;

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
}

fn count_this_char(sequence: &str, this_char: char) -> usize {
    let result = sequence.chars().filter(|&x| x == this_char).count();
    result
}

fn delete_this_char(text: &[char], this_char: char) -> String {
    use std::iter::FromIterator;

    let mut result: String = String::from_iter(text);
    result.retain(|c| c != this_char);
    result
}

fn mutate_string(text: &[u8], p_err: f64) -> String {
    use rand::prelude::*;

    let mut rng = thread_rng();
    let acgt = ['A', 'C', 'G', 'T'];
    let result = text.iter().map(|nucleotid| {
        let mut new_nucleotid: u8 = nucleotid.clone();
            if rng.gen_bool(p_err) {
                let gwe = delete_this_char(&acgt, (*nucleotid).into());
                new_nucleotid = *rng.choose(gwe.as_bytes()).unwrap();
            }
            new_nucleotid
        });

    String::from_utf8(result.collect()).unwrap()
}

fn prepare_qgrams<'a>(sequence: &'a str, length: usize, p_err: f64) -> impl Iterator<Item=String> + 'a {
    let sequence: &[u8] = sequence.as_ref();
    sequence.windows(length).map(move |qgram| mutate_string(qgram, p_err))
}

fn main() {
    // Read sequence from FASTA file from stdin
    let seq: String = read_seq();
    // Aufg. 1:
    let char_count = seq.len();
    let a_count = count_this_char(&seq, 'A');
    let c_count = count_this_char(&seq, 'C');
    let g_count = count_this_char(&seq, 'G');
    let t_count = count_this_char(&seq, 'T');
    let garb_count = char_count - a_count - c_count - g_count - t_count;
    let c_g_sum = c_count + g_count;
    let c_g_ratio = c_g_sum as f64/ char_count as f64;
    println!("Gesamt: {}; CG-Ratio: {}; Clean? {}", char_count, c_g_ratio, garb_count == 0); 
    //-------------------------------------------
    // Aufg. 2:
    const C: usize = 30;
    for i in 0..C{

    }
}
