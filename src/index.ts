function addItemToList() {
    const initialList = [1, 2, 3];

    const newList = initialList.map(item => item + 1);

    return newList;
}

console.log("addItemToList ", addItemToList());

import fs from "fs";

fs.readFileSync("lines.txt").
    toString().
    split("\n").
    filter((_, index) => index % 2 === 0).
    filter((_, index) => index > 1 && index < 4).
    forEach(line => console.log(line));

enum Color {
    Red = "red",
    Blue = "blue",
    Green = "green"
}

function printColor(color: Color) {
    switch (color) {
        case Color.Red:
            console.log("red");
            break;

        case Color.Blue:
            console.log("blue");
            break;

        case Color.Green:
            console.log("green");
            break;
    }
}

printColor(Color.Red);

type Custom = {
    age: number,
    name: string,
}

type Item = number | string | Custom;

function append(items: Item[]) {
    items.push("Hello Fem!");

    return items;
}

const myArray: Item[] = ["something"];
console.log(myArray);
append(myArray);
console.log(myArray);

const numbers: number[] = [1, 2, 3];
console.log(numbers);
append(numbers);
console.log(numbers);

function multiplyNumber(number: number | undefined): number | undefined {
    // return number === undefined ? undefined : number * 5;
    if (number === undefined) {
        return undefined;
    }

    return number * 5;
}

console.log("multiplyNumber", multiplyNumber(undefined));
console.log("multiplyNumber", multiplyNumber(3));

function practice(nums: number[], index: number): number {
    // let multiplication = index * 5;

    // if (index < nums.length) {
    //     multiplication = nums[index] * 5;
    // }

    // return multiplication;

    return (nums[index] ?? index) * 5;
}

console.log("practice", practice([1, 2, 3], 5));
console.log("practice", practice([1, 2, 3], 1));

// Error handling
function printNumbersFromTxtFile() {
    const fileName = process.argv[2];

    if (!fileName) {
        return null;
    } else {
        return fs.readFileSync(fileName).
            toString().
            split("\n").
            forEach(line => {
                // if (Number(line)) {
                //     console.log(line);
                // } else {
                //     console.log("Line not a number");
                // }
                const print = parseInt(line);

                if (isNaN(print)) {
                    console.log("Line not a number");
                } else {
                    console.log(print);
                }
            });
    }
}

printNumbersFromTxtFile();

interface Area {
    area(): number;
}

class Rectangle implements Area {
    constructor(
        public x: number,
        public y: number,
        public width: number,
        public height: number
    ){}

    area(): number {
        return this.width * this.height;
    }

    toString() {
        return `Rectangle(${this.x}, ${this.y}): ${this.width}x${this.height}`;
    }
}

class Circle {
    constructor(
        public x: number,
        public y: number,
        public radius: number
    ){}

    area(): number {
        return this.radius * this.radius * Math.PI;
    }
}

const circle = new Circle(0, 0, 10);
const rect = new Rectangle(0, 0, 10, 10);

console.log(rect, circle);


