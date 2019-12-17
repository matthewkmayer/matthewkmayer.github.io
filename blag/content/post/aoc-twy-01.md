+++
date = "2019-12-16T00:28:53-07:00"
draft = false
title = "Advent of Code 2019, the wrong way (part one)"
+++

Per [my tweet](https://twitter.com/Motoblag/status/1203557633648553984): doing Advent of Code 2019 the wrong way. 😁

Today we're easing into #serverless.

## WHAT are we doing?

[Advent of Code](https://adventofcode.com/2019/about)!

`Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like.`

It's an excuse to play around with smaller coding problems and enjoy some recreational coding. In this series of blog posts, we're going to go about solving the problems the wrong way: as much serverless as possible.

## Yup, no servers

Instead of solving the puzzles on my computer, I'm going to use serverless as much as I can. This theme will certainly create hilariously overbuilt and overly complicated results, but that's going to be part of the fun this year.

## Day one, part one

There are two parts to day one: reading the problem input and making a function for calculating fuel load: ` Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.`

Let's ease into serverless by starting on [the Rust playground](https://play.rust-lang.org/).

I like test-first development, especially with these puzzles. Let's do that first:

```rust
fn main() {
    println!("yo");
}

// take its mass, divide by three, round down, and subtract 2
fn fuel_required(mass: i64) -> i64 {
    return mass;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }
}
```

The test cases in the problem description are converted to a test here. The `fuel_required` function is stubbed out, and our tests fail:

```bash
failures:

---- tests::test_fuel stdout ----
thread 'tests::test_fuel' panicked at 'assertion failed: `(left == right)`
  left: `12`,
 right: `2`', src/lib.rs:17:9
 ```

 So far so good. Adding the implementation for `fuel_required` we get:

 ```rust
 // take its mass, divide by three, round down, and subtract 2
fn fuel_required(mass: i64) -> i64 {
    let mass_as_float = mass as f64;
    let m1 = (mass_as_float / 3.0).floor();
    return (m1 - 2.0) as i64;
}
```

Results:

```bash
running 1 test
test tests::test_fuel ... ok
```

If we were running locally, we could put our hundred lines of input into a file and read that. Being ~~~serverless let's take a different approach and use a [raw string literal](https://doc.rust-lang.org/rust-by-example/std/str.html):

```rust
// parts removed for brevity
const DAYS_INPUT: &str = r#"
130762
108691
131618
138163
"#;
```

Now we can get the strings out, splitting by whitespace:

```rust
let string_entries: Vec<&str> = DAYS_INPUT.split_whitespace().collect();
println!("string entries: {:?}", string_entries);
```

That works great, but we need integers for our fuel calculations:

```rust
let int_entries: Vec<i64> = string_entries.iter().map(|x| x.parse::<i64>().unwrap()).collect();
println!("int entries: {:?}", int_entries);  
```

And per the day's instructions, we need to run each entry through the calculation function and get the total:

```rust
let total_fuel: i64 = int_entries.iter().map(|x| fuel_required(*x)).sum();
println!("fuel required: {:?}", total_fuel);
```

That's it! 🎉 See the entire solution on the Rust Playground: [https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=df26e4c4a8d8b7e2fa8c829ebf8b98e3](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=df26e4c4a8d8b7e2fa8c829ebf8b98e3).

Day one, part one is complete. Soon we'll get into (ab)using AWS for solving these problems.
