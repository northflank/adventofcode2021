const input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

function getMedian(arr) {
  return arr.slice().sort((a, b) => a - b)[Math.floor(arr.length / 2)];
}

function totalFuel(input, position) {
  let totalFuel = 0;
  input.forEach((i) => (totalFuel += Math.abs(position - i)));
  return totalFuel;
}

console.log('Task 1:', totalFuel(input, getMedian(input)));
