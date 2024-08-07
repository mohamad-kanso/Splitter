import * as fs from 'fs';
import * as path from 'path';
var jp = require('jsonpath');
import cloneDeep from 'lodash.clonedeep';
import microtime from 'microtime';

function main() {
    try {
        const jsonPath = path.join('/home/ubuntu/splitter-wasm', 'payload.json');
        const jsonData = fs.readFileSync(jsonPath, 'utf8');
        const data = JSON.parse(jsonData);
    
        let query = '$.payload.logs';

        let start = microtime.now();
        for (let i = 0; i<1000; i++) {
            let result = splitter(cloneDeep(data),query);
        }
        let end = microtime.now();

        let execution = (end - start) / 1000;
        // console.log(result);
        console.log(`execution time: ` + execution + `ms`);
        console.log();
        console.log(`average execution time is: ` + execution/1000 + `ms`);

    } catch (error) {
        if (error instanceof Error) {
            console.error('An error occurred:', error.message);
        } else {
            console.error('An unknown error occurred');
        }
    }

}

export function splitter(jsonObject: any, query: string): any {
    // TODO ERROR HANDLING
    let paths = jp.paths(jsonObject, query);
    arrayPathChecker(jsonObject, paths[0])
    let splitedObject = [];
    let originalObjectPtr = jsonObject;

    for (const node of paths[0]) {
        if (node == '$')
            continue;
        originalObjectPtr = originalObjectPtr[node];
    }
    if (originalObjectPtr.length == 0) // if the array is emty return empty array 
        return [];
    if (originalObjectPtr.length == 1) // if the array is empty or contain only one element return the same object
        return [jsonObject];
    let arrayDeepClone = cloneDeep(originalObjectPtr)
    for (const item of arrayDeepClone) {
        originalObjectPtr.splice(0, originalObjectPtr.length);
        originalObjectPtr.push(item);
        let newObject = cloneDeep(jsonObject);
        splitedObject.push(newObject);
    }
    return splitedObject;
}

export function arrayPathChecker(jsonObject: any, path: any) {
    let currentObject = jsonObject
    for (const node of path) {
        if (node == '$')
            continue;
        currentObject = currentObject[node]
    }    
    if (Array.isArray(currentObject)) { // Check if the lastNode is an Array
        return true
    } else {
        throw new SplitterTypeError("No Array In the Path To Split!!")
    }
}

export class SplitterTypeError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'TypeError';
    }
}

main()