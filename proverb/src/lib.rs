pub fn get_proverb_line(a: &str, b: &str) -> String {
    format!("For want of a {} the {} was lost.", a, b)
}

pub fn build_proverb(list: &[&str]) -> String {
    // let mut outs = vec![];
    // if list.len() >= 2 {
    //     for i in 0..list.len() - 1 {
    //         outs.push(get_proverb_line(list[i], list[i + 1]));
    //     }
    // }
    // if !list.is_empty() {
    //     outs.push(format!("And all for the want of a {}.", list[0]));
    // }
    // outs.join("\n")

    // 模範解答
    // https://exercism.io/tracks/rust/exercises/proverb/solutions/b8351b82c9d74924aa7c555fe43a8384
    if list.is_empty() {
        return String::new();
    }

    // windows: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.windows
    //   Returns an iterator over all contiguous windows of length size. 
    //   The windows overlap. If the slice is shorter than size, the iterator returns no values.
    // chain: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain
    //   Takes two iterators and creates a new iterator over both in sequence.
    //   chain() will return a new iterator which will first iterate over values from the first iterator
    //   and then over values from the second iterator.
    //   In other words, it links two iterators together, in a chain. 🔗
    //   std::iter::once is commonly used to adapt a single value into a chain of other kinds of iteration.
    //   once: https://doc.rust-lang.org/std/iter/fn.once.html
    //     Creates an iterator that yields an element exactly once.
    //     This is commonly used to adapt a single value into a chain of other kinds of iteration. 
    //     Maybe you have an iterator that covers almost everything, but you need an extra special case. 
    //     Maybe you have a function which works on iterators, but you only need to process one value.
    list.windows(2)
        .map(|win| format!("For want of a {0} the {1} was lost.", win[0], win[1])) // returns std::iter::Map
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect::<Vec<_>>()
        .join("\n")
}
