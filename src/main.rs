
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use overflow::{
  digits,
  permutations,
};

use digits::Digits;



/*
 * Complete the 'solve' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING n as parameter.
 */
fn solve(n: u32)-> &'static str {
  overflow::permutations::permutations(Digits::new(n).collect::<Vec<_>>(),|digits| {
  });

  ""
}

fn main()-> Result<(),Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap()?.trim().parse::<usize>()?;

    for n in stdin_iterator.take(t).flatten().filter_map(|line| line.trim().parse::<u32>().ok()) {
      writeln!(fptr,"{}",solve(n)).unwrap();
    }


  Ok(())
}
























