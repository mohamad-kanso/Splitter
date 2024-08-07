import * as wasm from "./pkg/splitter_wasm.js";
import microtime from "microtime";
import {promises as fs} from 'fs';

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

    // let start = microtime.now();
    // // for (let i = 0; i<1000; i++) {
    //     let result = wasm.splitter(json,query);
    // // }
    // let end = microtime.now();

    // let j_result = mapToObject(result);
    // let string  = JSON.stringify(j_result);
    // let execution = (end-start)/1000;
    // // console.log(result);
    // // console.log();
    // // console.log(j_result);
    let execution = wasm.test(json,query);
    console.log(`wasm execution time: ` + execution + `ms`);
    console.log();
    // console.log(`average execution time is: ` + execution/1000 + `ms`);
    // console.log(string);
}

main()