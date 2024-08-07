"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.SplitterTypeError = void 0;
exports.splitter = splitter;
exports.arrayPathChecker = arrayPathChecker;
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
var jp = require('jsonpath');
const lodash_clonedeep_1 = __importDefault(require("lodash.clonedeep"));
const microtime_1 = __importDefault(require("microtime"));
function main() {
    try {
        const jsonPath = path.join('/home/ubuntu/splitter-wasm', 'payload.json');
        const jsonData = fs.readFileSync(jsonPath, 'utf8');
        const data = JSON.parse(jsonData);
        let query = '$.payload.logs';
        let start = microtime_1.default.now();
        for (let i = 0; i < 1000; i++) {
            let result = splitter((0, lodash_clonedeep_1.default)(data), query);
        }
        let end = microtime_1.default.now();
        let execution = (end - start) / 1000;
        // console.log(result);
        console.log(`execution time: ` + execution + `ms`);
        console.log();
        console.log(`average execution time is: ` + execution / 1000 + `ms`);
    }
    catch (error) {
        if (error instanceof Error) {
            console.error('An error occurred:', error.message);
        }
        else {
            console.error('An unknown error occurred');
        }
    }
}
function splitter(jsonObject, query) {
    // TODO ERROR HANDLING
    let paths = jp.paths(jsonObject, query);
    arrayPathChecker(jsonObject, paths[0]);
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
    let arrayDeepClone = (0, lodash_clonedeep_1.default)(originalObjectPtr);
    for (const item of arrayDeepClone) {
        originalObjectPtr.splice(0, originalObjectPtr.length);
        originalObjectPtr.push(item);
        let newObject = (0, lodash_clonedeep_1.default)(jsonObject);
        splitedObject.push(newObject);
    }
    return splitedObject;
}
function arrayPathChecker(jsonObject, path) {
    let currentObject = jsonObject;
    for (const node of path) {
        if (node == '$')
            continue;
        currentObject = currentObject[node];
    }
    if (Array.isArray(currentObject)) { // Check if the lastNode is an Array
        return true;
    }
    else {
        throw new SplitterTypeError("No Array In the Path To Split!!");
    }
}
class SplitterTypeError extends Error {
    constructor(message) {
        super(message);
        this.name = 'TypeError';
    }
}
exports.SplitterTypeError = SplitterTypeError;
main();
