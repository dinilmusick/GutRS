#!/usr/bin/env node
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const cwd=process.cwd();
const cases=JSON.parse(readFileSync(resolve(cwd,'testdata/upstream-golden/ettin-head.json'),'utf8'));
const rust=JSON.parse(readFileSync(resolve(cwd,'rust-ettin-head.json'),'utf8'));
const {computeEttinHeadScores}=await import(pathToFileURL(resolve(cwd,'upstream/dist/models/runtimes/ettin-head.js')).href);
if(rust.length!==cases.cases.length)throw new Error("Ettin fixture count differs");
for(let i=0;i<cases.cases.length;i++){
 const c=cases.cases[i],h=c.head;
 const head={w1:new Float32Array(h.w1),lnWeight:new Float32Array(h.lnWeight),
   lnBias:new Float32Array(h.lnBias),w2:new Float32Array(h.w2),
   b2:new Float32Array(h.b2)};
 const expected=computeEttinHeadScores({dims:c.dims,data:new Float32Array(c.data)},head);
 const actual=rust[i];
 if(actual.length!==expected.length||actual.some((score,j)=>Math.abs(score-expected[j])>1e-7)){
   throw new Error('Ettin head numerical mismatch at fixture '+i+
    ': JS '+JSON.stringify(expected)+' Rust '+JSON.stringify(actual));
 }
}
console.log('Certified native Ettin GELU/LayerNorm/Dense/sigmoid against pinned JS: '+rust.length+' fixture batches');
