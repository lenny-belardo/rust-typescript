function addItemToList() {
    const initialList = [1, 2, 3];

    const newList = initialList.map(item => item + 1);

    return newList;
}

console.log("addItemToList ", addItemToList());