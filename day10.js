const fs = require('fs');

const input = fs
    .readFileSync('day10-input.txt', 'utf8')
    .split(/\n/g)

const rules = { ")": 3, "]": 57, "}": 1197, ">": 25137, "(": 1, "[": 2, "{": 3, "<": 4 }

let score = 0
let scoresArray = []
for (let line of input) {
    let arr = []
    let isValid = true
    for (let char of line)
        if (['(', '[', '{', '<'].includes(char)) arr.push(char)
        else if (![1, 2].includes(char.charCodeAt(0) - arr.pop().charCodeAt(0))) {
            isValid = false
            score += rules[char]
            break
        }
    if (isValid) {
        let t2score = 0
        while (arr.length > 0) t2score = t2score * 5 + rules[arr.pop()]
        scoresArray.push(t2score)
    }
}

console.log("task 1", score)
console.log("task 2", scoresArray.sort((a, b) => { return a - b })[(scoresArray.length - 1) / 2])
