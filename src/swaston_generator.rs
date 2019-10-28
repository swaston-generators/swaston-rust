use unicode_segmentation::UnicodeSegmentation;
use itertools::Itertools;


fn reverse(input: &str) -> String {
    return input.graphemes(true).rev().collect();
}

fn join(input: &str) -> String {
    return input.graphemes(true).join(" ");
}

fn substr(input: &str, st: usize, en: usize) -> String {
    return input.graphemes(true).skip(st).take(en - st).collect();
}

fn len(input: &str) -> usize {
    return input.graphemes(true).count();
}

fn string_to_vec(input: &str) -> Vec<String> {
    let mut v: Vec<String> = vec![];

    for i in input.graphemes(true).into_iter() {
        v.push(i.to_string());
    }
    return v;
}

pub fn generate_swaston(s: &str) -> String {
    if len(s) < 2 {
        return s.to_string();
    }
    let (l_spaced_word,
        r_spaced_word,
        l_word) =
        (
            join(s),
            &reverse(&join(s)),
            &reverse(s),
        );

    let center = format!("{0}{1}\n", l_spaced_word, substr(r_spaced_word, 1, len(r_spaced_word)));
    let t = " ".repeat(len(&r_spaced_word) - 2);
    let r_vec = string_to_vec(s);
    let l_vec = string_to_vec(l_word);
    let size: usize = l_vec.len();

    let mut upper: String = "".to_string();
    let mut lower: String = "".to_string();

    for c in 0..size {
        if c == 0 {
            upper.push_str(&format!("{0}{1}{2}\n", l_vec[c], t, l_spaced_word));
        } else if c + 1 != size {
            upper.push_str(&format!("{0}{1}{2}{3}\n", l_vec[c], t, r_vec[c], t));
        }
    }

    for c in 0..size { // lower side
        if c == 0 { // case first row (center)
            continue;
        } else if c + 1 != size {
            lower.push_str(&format!("{0}{1}{2}{3}{4}\n", t, ' ', l_vec[c], t, r_vec[c]));
        } else { // case last row
            lower.push_str(&format!("{0}{1}{2}", r_spaced_word, t, r_vec[c]));
        }
    }
    return format!("{0}{1}{2}", upper, center, lower);
}
