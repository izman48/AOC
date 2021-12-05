use std::fs::read_to_string;
use std::convert::TryInto;
mod day2;

fn main() {
  let filename = "input/inputAOCDay3.txt"; 
  println!("In file {}", filename);

  let thing = read_to_string(filename).unwrap();
  let lines = thing.lines().collect::<Vec<_>>();

  let answer = calculate2(lines);
  println!("{}", answer);
  
}

fn calculate(lines: Vec<&str>) -> i64 {
   let vec_size = lines.len();
   let mut gamma: String = "".to_owned();
   let mut epsilon: String = "".to_owned();

   for i in 0..lines[0].len() {
    let mut count0 = 0;
    let mut count1 = 0;

    for j in 0..vec_size {
        let line = lines[j];
        let c = line.chars().nth(i).unwrap();
        if c == '0' {
          count0 = count0 + 1;
        } else  {
          count1 = count1 + 1;
        }
    }
    if count0 > count1 {
      gamma.push_str("0");
      epsilon.push_str("1");
    } else {
      gamma.push_str("1");
      epsilon.push_str("0");
    }
    
   }
   let intgamma = isize::from_str_radix(&gamma, 2).unwrap();
   let inteps = isize::from_str_radix(&epsilon, 2).unwrap();

   let ans = intgamma * inteps;
   return ans.try_into().unwrap();
}

fn calculate2(lines: Vec<&str>) -> i64 {
   let vec_size = lines.len();
   
   let mut oxygen = lines.clone().to_owned();
   let mut co2 = lines.clone().to_owned();

   for i in 0..lines[0].len() {
    let mut count0 = 0;
    let mut count1 = 0;

    for j in 0..vec_size {
        let line = lines[j];
        let c = line.chars().nth(i).unwrap();
        if c == '0' {
          count0 = count0 + 1;
        } else  {
          count1 = count1 + 1;
        }
    }
    if count0 == count1 {
      break;
    }
    if count0 > count1 {
      for j in 0..vec_size {
        let line = lines[j];
        let c = line.chars().nth(i).unwrap();
        if c == '0' {
          co2.remove(j);
        } else  {
          oxygen.remove(j);
        }
     }
    } else {
      for j in 0..vec_size {
        let line = lines[j];
        let c = line.chars().nth(i).unwrap();
        if c == '0' && oxygen.contains(&line) {
          oxygen.remove(j);
        } else  {
          co2.remove(j);
        }
     }
    }
    
   }

   let o = oxygen[0];
   let c = co2[0];
   let intgamma = isize::from_str_radix(&o, 2).unwrap();
   let inteps = isize::from_str_radix(&c, 2).unwrap();

   let ans = intgamma * inteps;
   return ans.try_into().unwrap();
}
