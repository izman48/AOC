use std::fs::read_to_string;

fn main() {
  let filename = "input/inputAOCDay2.txt"; 
  println!("In file {}", filename);

  let thing = read_to_string(filename).unwrap();
  let lines = thing.lines().collect::<Vec<_>>();

  let answer = day2(lines);
  println!("{}", answer);
  
}

fn day2(lines: Vec<&str>) -> i64 {
  let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;

    for line in lines {

            let parts: Vec<&str> = line.split(' ').collect();
            let val: i64 = parts[1].parse().unwrap();
            if parts[0] == "forward" {
              x = val + x;
              y = y + (aim * val);
            }
            if parts[0] == "up" {
              aim = aim - val;
            }
            if parts[0] == "down" {
              aim = aim + val;
            
        }
    }
    let answer: i64 = x * y;
    return answer;
    
}