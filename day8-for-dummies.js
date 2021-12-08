const fs = require('fs');

const file = fs.readFileSync('input-day8.txt', 'utf8')

const input = file
    .toString('utf8')
    .split(/\n/g)
    .map(e => e
        .split(' ').map(i => i.split('').sort().join('')).join(' ')
        .split(/ [|] /g)
    )

// Each line of the input is now an array consisting of two elements and sorted items, for example:
// [ "abcdf af abcdfg bcdfg abcdefg abcefg abcde acf bcdefg adfg", "adfg af acf af" ]

// Start of Task 1
let count = 0
input.map(i => i[1].split(" ").map(e => [2, 3, 4, 7].includes(e.length) && count++))
console.log("Task 1", count)
// End of Task 1

// Start of Task 2
let totalSum = 0
let arr = ['', '', '', '', '', '', '', '', '', '']
for (let line of input) {
    let firstPart = line[0].split(" ")
    // Set numbers we definitely know just by looking at lengths
    for (let i = 0; i < firstPart.length; i++) {
        let res
        if (firstPart[i].length === 2) res = 1
        else if (firstPart[i].length === 4) res = 4
        else if (firstPart[i].length === 3) res = 7
        else if (firstPart[i].length === 7) res = 8
        if (res) {
            arr[res] = firstPart[i]
            firstPart[i] = ""
        }
    }
    // Find the remaining numbers by what we can assume...
    // THREE has 5 segments (= is 5 characters long)
    // and shares them with SEVEN with 2 mistakes (= three also has two different segments)
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0
        if (firstPart[i].length === 5)
            firstPart[i].split('').map(ch => !arr[7].includes(ch) && mistakes++)
        if (mistakes === 2) {
            arr[3] = firstPart[i]
            firstPart[i] = ""
            break;
        }
    }
    // NINE has 6 segments and shares them with THREE with 1 mistake
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0
        if (firstPart[i].length === 6)
            firstPart[i].split('').map(ch => !arr[3].includes(ch) && mistakes++)
        if (mistakes === 1) {
            arr[9] = firstPart[i]
            firstPart[i] = ""
            break;
        }
    }
    // ZERO and SIX have 6 segments and share them with SEVEN and 3 and 4 mistakes respectively
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0
        if (firstPart[i].length === 6)
            firstPart[i].split('').map(ch => !arr[7].includes(ch) && mistakes++)
        if (mistakes === 3) {
            // Zero
            arr[0] = firstPart[i]
            firstPart[i] = ""
        } else if (mistakes === 4) {
            // Six
            arr[6] = firstPart[i]
            firstPart[i] = ""
        }
    }
    // TWO has 5 segments and shares then with NINE with 1 mistake
    for (let i = 0; i < firstPart.length; i++) {
        let mistakes = 0
        if (firstPart[i].length === 5)
            firstPart[i].split('').map(ch => !arr[9].includes(ch) && mistakes++)
        if (mistakes === 1) {
            // you found a two
            arr[2] = firstPart[i]
            firstPart[i] = ""
            break;
        }
    }
    // FIVE is left
    for (let i = 0; i < firstPart.length; i++) {
        if (firstPart[i].length === 5) {
            arr[5] = firstPart[i]
            firstPart[i] = ""
            break
        }
    }
    result = ""
    let secondPart = line[1].split(" ")
    for (let seq of secondPart) {
        seq = seq.split('').sort().join('')
        result += (arr.indexOf(seq))
    }
    totalSum += parseInt(result, 10)
}
console.log("Task 2", totalSum)
// End of Task 2
