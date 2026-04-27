pub fn equals_unordered<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut matched = vec![false; b.len()];

    for item_a in a {
        let mut found_match = false;
        for (i, item_b) in b.iter().enumerate() {
            if !matched[i] && item_a == item_b {
                matched[i] = true;
                found_match = true;
                break;
            }
        }
        if !found_match {
            return false;
        }
    }

    true
}
