use rand::Rng;

fn main() {
let mut rng =  rand::thread_rng();
let heptagram = rng.gen_range(0..128);
let true_h = heptagram + 1;
let h_binary = format!("{:07b}", heptagram);
//let h_string = h_binary.to_string();
let h_bools: Vec<bool> = h_binary.chars().map(|c| c == '1').collect();
h_bools.iter().for_each(|&x| convert_to_line(x));
println!("Heptagram: {true_h}");
}
fn convert_to_line(var: bool) {
     if var == false {
         println!("\t--  --");
     } else {
         println!("\t------");
     }
}

