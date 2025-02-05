pub fn are_almost_equal(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }

    let s1_vec: Vec<char> = s1.chars().collect();
    let s2_vec: Vec<char> = s2.chars().collect();

    let mut diff: u8 = 0;
    let mut diff_index: Vec<usize> = Vec::new();

    for (idx, (val_s1, val_s2)) in s1_vec.iter().zip(s2_vec.iter()).enumerate() {
        if val_s1 != val_s2 {
            if diff >= 2 {
                return false;
            }
            diff += 1;

            diff_index.push(idx);
        }
    }
    match diff {
        2 => {
            let mut s2_mut = s2_vec.clone();
            s2_mut.swap(diff_index[0], diff_index[1]);
            s1_vec == s2_mut
        }
        _ => false,
    }
}
