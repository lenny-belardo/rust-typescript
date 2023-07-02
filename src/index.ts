function addItemToList() {
    const initialList = [1, 2, 3];

    const newList = initialList.map(item => item + 1);

    return newList;
}

console.log("addItemToList ", addItemToList());

import fs from "fs";

fs.readFile('/Users/raoulgheletus/Documents/practice/rust-typescript/lines.txt', 'utf8', (err: any, data: any) => {
    if (err) {
        console.error(err);

        return;
    }

    console.log(data);
});
