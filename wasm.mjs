import * as wasm from "./pkg/splitter_wasm.js";
import microtime from "microtime";
import {promises as fs} from 'fs';
import { SplitterNode } from "./pkg/splitter_wasm.js";
import cloneDeep from 'lodash.clonedeep';

function mapToObject(map) {
    if (map instanceof Map) {
        const obj = {};
        for (const [key, value] of map) {
            obj[key] = mapToObject(value);
        }
        return obj;
    } else if (Array.isArray(map)) {
        return map.map(item => mapToObject(item));
    } else {
        return map;
    }
}

async function main() {
    let data = await fs.readFile('./payload.json');
    let json = JSON.parse(data);

    let query = ('$.payload.logs');
    let node = new SplitterNode(query);
    let start = microtime.now();
    for (let i = 0; i<1000; i++) {
        let result = node.execute(json);
    }
    let end = microtime.now();
    let execution = (end-start)/1000;
    // let j_result = mapToObject(result);
    // let string  = JSON.stringify(j_result);
    // // console.log(result);
    // // console.log(j_result);
    // console.log(string);
    console.log(`wasm execution time: ` + execution + `ms`);
    console.log();
}

main()