use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

fn utf8_reverse(input: &str) -> String {
    return input.graphemes(true).rev().collect();
}

fn utf8_join(input: &str) -> String {
    return input.graphemes(true).join(" ");
}

fn utf8_substr(input: &str, st: usize, en: usize) -> String {
    return input.graphemes(true).skip(st).take(en - st).collect();
}

fn utf8_distance(input: &str) -> usize {
    return input.graphemes(true).count();
}

fn utf8_to_vector(input: &str) -> Vec<String> {
    let mut v: Vec<String> = vec![];

    for i in input.graphemes(true).into_iter() {
        v.push(i.to_string());
    }
    return v;
}

pub fn generate_swaston(s: &str) -> String {
    if utf8_distance(s) < 2 {
        return s.to_string();
    }
    let l_spaced_word = utf8_join(&s);
    let r_spaced_word = utf8_reverse(&l_spaced_word);
    let l_word = utf8_reverse(&s);
    let r_word = s.to_string();
    let center = format!("{0}{1}\n",
                         l_spaced_word,
                         utf8_substr(&r_spaced_word, 1, utf8_distance(&r_spaced_word))
    );
    let tab_pre = " ".repeat(utf8_distance(&r_spaced_word) - 2);
    let tab_post = tab_pre.clone() + " ";
    let r_vec = utf8_to_vector(&r_word);
    let l_vec = utf8_to_vector(&l_word);
    let length = l_vec.len();

    let mut upper = String::new();
    let mut lower = String::new();

    for c in 0..length {
        if c == 0 { // case first row
            upper.push_str(&format!("{0}{1}{2}\n", l_vec[c], tab_pre, l_spaced_word));
        } else if c + 1 != length {
            upper.push_str(&format!("{0}{1}{2}{3}\n", l_vec[c], tab_pre, r_vec[c], tab_post));
        }
    }

    for c in 1..length { // lower side
        if c == length - 1 {  // case last row
            lower.push_str(&format!("{0}{1}{2}\n", r_spaced_word, tab_pre, r_vec[c]));
        } else {
            lower.push_str(&format!("{0}{1}{2}{3}\n", tab_post, l_vec[c], tab_pre, r_vec[c]));
        }
    }
    return format!("{0}{1}{2}", upper, center, lower);
}
