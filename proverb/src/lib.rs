pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return "".to_string();
    }
    let mut proverb = String::new();
    for (i, item) in list.iter().enumerate() {
        if i == 0 {
            continue;
        };
        proverb = format!(
            "{}{}\n",
            proverb,
            format!("For want of a {} the {} was lost.", list[i - 1], item),
        )
    }
    format!("{}And all for the want of a {}.", proverb, list[0])
}
