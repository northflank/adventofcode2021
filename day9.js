const fs = require('fs');

const file = fs.readFileSync('day9-input.txt', 'utf8')

const input = file
    .toString('utf8')
    .split(/\n/g)
    .map(e => e
        .split('')
        .map(Number)
    )

// surround the array with 9s for easier processing 
input.map(e => e.push(9))
input.map(e => e.unshift(9))
input.unshift(Array(input[0].length).fill(9))
input.push(Array(input[0].length).fill(9))

// Part 1
let segments = [] // for part 2
let result = []
for (let x = 1; x < input.length - 1; x++) {
    for (let y = 1; y < input[x].length - 1; y++) {
        let val = input[x][y]
        if (val < input[x - 1][y] && val < input[x + 1][y] && val < input[x][y - 1] && val < input[x][y + 1]) {
            result.push(val + 1)
            segments.push([x, y]) // for part 2
        }
    }
}

let p1res = result.reduce((partial_sum, a) => partial_sum + a, 0);
console.log("Part1", p1res)
// End of Part 1

// Part 2
function findNeighbors(x, y, prevArr) {
    let posX = [x - 1, x + 1, x, x]
    let posY = [y, y, y - 1, y + 1]
    for (let i = 0; i < posX.length; i++) {
        if (input[posX[i]][posY[i]] > input[x][y] && input[posX[i]][posY[i]] !== 9) {
            prevArr.push([posX[i], posY[i]])
            findNeighbors(posX[i], posY[i], prevArr)
        }
    }
    return prevArr
}


let allBasins = []
for (let coordinates of segments) {
    let basin = []
    basin = findNeighbors(coordinates[0], coordinates[1], []) // get coordinates of neighbors that meet the criteria
    basin.push([coordinates[0], coordinates[1]]) // add the coordinates of the lowest point
    // remove array of array duplicates:
    // https://stackoverflow.com/questions/44014799/javascript-how-to-remove-duplicate-arrays-inside-array-of-arrays
    basin = basin.filter((t = {}, a => !(t[a] = a in t)));
    allBasins.push(basin)
}
allBasins.sort((a, b) => b.length - a.length); // sort by lenghts of inside arrays
let p2res = 1
for (let i = 0; i < 3; i++) p2res = p2res * allBasins[i].length // multiply first three
console.log("Part 2", p2res)

// End of Part 2
