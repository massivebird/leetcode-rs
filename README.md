<p align="center">
  <img src="https://leetcard.jacoblin.cool/massivebird?theme=dark&font=JetBrains%20Mono&ext=heatmap" />
</p>

> Banner generated with [leetcode.jacoblin.cool](https://leetcard.jacoblin.cool/)

# massivebird's Rusty leetcode solutions

Here are my Leetcode solutions, all in Rust 🦀

The `main` branch exclusively contains working solutions.

I'm working through the [Top Interview 150](https://leetcode.com/studyplan/top-interview-150/), it's fun and covers a lot of topics. Keep those linked lists and trees fresh.

## FAQ

__Q: Rust is cool! How can I start learning?__

__A:__ Start your Rust journey [right here](https://rust-book.cs.brown.edu/ch00-00-introduction.html)!

__Q: Can you explain \<code segment>?__

__A:__ See above.

__Q: Why the weird function signatures?__

__A:__ TLDR: it's LeetCode's fault, and I would change them if I could.

LeetCode's Rust support is rough around the edges. You can't change much about the provided function signatures; I think all you're allowed is adding/removing `mut` keywords. For example, in most cases, a provided function argument typed `Vec<i32>` should technically be redefined as a slice `&[i32]`, but LeetCode disallows that type of modification.

__Q: What is `#[allow(clippy::needless_pass_by_value)]`?__

__A:__ TLDR: See above.

This is an [attribute](https://doc.rust-lang.org/rust-by-example/attribute.html) that instructs the code linter (clippy) to quit bothering me about inefficient parameter typing. I employ this for LeetCode function signatures that I'm not allowed to change. Click [here](https://rust-lang.github.io/rust-clippy/master/index.html#needless_pass_by_value) for more info on the lint.

__Q: I found a curse word in the source code. Can you remove it?__

__A:__ That's my bad. I'll remove it right away

## Favorite challenges

+ 0136: Single Number
+ 0200: Number of Islands
