use rand::Rng;



fn main() {
let mut rng =  rand::thread_rng();
let l1 = rng.gen_bool(0.5);
let l2 = rng.gen_bool(0.5);
let l3 = rng.gen_bool(0.5);
let l4 = rng.gen_bool(0.5);
let l5 = rng.gen_bool(0.5);
let l6 = rng.gen_bool(0.5);
let l7 = rng.gen_bool(0.5);
convert_to_line(l7);
convert_to_line(l6);
convert_to_line(l5);
convert_to_line(l4);
convert_to_line(l3);
convert_to_line(l2);
convert_to_line(l1);
}
fn convert_to_line(var: bool) {
     if var == false {
         println!("\t--  --");
     } else {
         println!("\t------");
     }
}
