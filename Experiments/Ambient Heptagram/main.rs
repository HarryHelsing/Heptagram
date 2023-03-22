use rand::Rng;
use std::process::Command;

fn main() {
let mut rng =  rand::thread_rng();

let mut l1 = rng.gen_bool(0.5);
let mut l2 = rng.gen_bool(0.5);
let mut l3 = rng.gen_bool(0.5);
let mut l4 = rng.gen_bool(0.5);
let mut l5 = rng.gen_bool(0.5);
let mut l6 = rng.gen_bool(0.5);
let mut l7 = rng.gen_bool(0.5);
let mut switch_line = 1;

loop
{
print!("\x1B[2J");
convert_to_line(l1);
convert_to_line(l2);
convert_to_line(l3);
convert_to_line(l4);
convert_to_line(l5);
convert_to_line(l6);
convert_to_line(l7);
switch_line = rng.gen_range(1..=7);
match switch_line {
	1 => {l1 = !l1}
	2 => {l2 = !l2}
	3 => {l3 = !l3}
	4 => {l4 = !l4}
	5 => {l5 = !l5}
	6 => {l6 = !l6}
	7 => {l7 = !l7}
	_ => (),
}
let mut child = Command::new("sleep").arg("1.00").spawn().unwrap();
let _result = child.wait().unwrap();
}
}

fn convert_to_line(var: bool) {
     if var == false {
         println!("\t--  --\t\t------");
     } else {
         println!("\t------\t\t--  --");
     }   
 }
