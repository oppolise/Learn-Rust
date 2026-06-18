fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}

fn main() {
    let m_map1 = "thai thepza";
    let m_map2 = "thai notthepza";

    let chosen_map = longest_map(m_map1, m_map2);
    println!("{}", chosen_map);
}
