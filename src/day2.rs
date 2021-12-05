pub fn calculate(lines: Vec<&str>) -> i64 {
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