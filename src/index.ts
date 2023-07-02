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
    forEach(line => console.log(line));
