const input = ["forward 8", "forward 9", "forward 9", "down 3", "forward 9", "down 1", "down 7", "down 7", ... ]
               
// Task 1
               
let horizontal = 0
let depth = 0 

for (let item of input) { 
  let command = item.split(" ") 
  let units = parseInt(command[1])
  switch(command[0]) {
    case "forward":
      horizontal += units
      break;
    case "up": 
      depth -= units
      break;
    case "down": 
      depth += units
      break;
    }
}

console.log(horizontal*depth)               
               
// Task 2

horizontal = 0
depth = 0 
let aim = 0

for (let item of input) { 
  let command = item.split(" ") 
  let units = parseInt(command[1])
  switch(command[0]) {
    case "forward":
      horizontal += units
      depth += aim * units
      break;
    case "up": 
      aim -= units
      break;
    case "down": 
      aim += units
      break;
    }
}

console.log(horizontal*depth)               
              
