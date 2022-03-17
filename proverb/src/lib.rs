pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut res = String::new();
    let mut last = String::new();
    list.into_iter().fold("", |prev, curr| {
        if prev.len() == 0 {
            last.push_str(format!("And all for the want of a {}.", curr).as_str());
        } else {
            res += &format!("For want of a {} the {} was lost.\n", prev, curr);
        }
        return curr;
    });
    res.push_str(last.as_str());
    res
}
