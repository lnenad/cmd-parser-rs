extern crate cmdparser;
use std::time::Instant;

use cmdparser::Parser;

fn main() {
    let start = Instant::now();
    let (arguments, flags) = Parser::new().merge_values(true).parse();
    let end = start.elapsed();


    println!("Result: \nArgumentList: {:?}  \nFlags:{:?} \nBenchmark: {:?}ns", arguments, flags, end.subsec_nanos());
}