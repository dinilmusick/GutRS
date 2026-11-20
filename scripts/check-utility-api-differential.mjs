#!/usr/bin/env node
// Executes all portable exact-ID GutTS APIs from the pinned, unmodified JS build.
// Compares independent native Rust output; no fixture expectations are invented.
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {execFileSync} from 'node:child_process';

const root=process.cwd();
const fixture=JSON.parse(readFileSync(resolve(root,'testdata/upstream-golden/api-utility-cases.json'),'utf8'));
const current=execFileSync('git',['rev-parse','HEAD'],{
 cwd:resolve(root,'upstream'),encoding:'utf8'
}).trim();
if(current!==fixture.upstream)throw new Error('Upstream SHA changed unexpectedly');
const load=async(category)=>import(pathToFileURL(resolve(root,'upstream/dist',category,
  category==='json'?'json-concepts.js':
  category==='arrays'?'array-concepts.js':
  category==='numbers'?'number-concepts.js':
  category==='objects'?'object-concepts.js':
  category==='strings'?'string-concepts.js':'url-concepts.js')).href);
const bundles=Object.fromEntries(await Promise.all(
 ['arrays','json','numbers','objects','strings','urls'].map(async category=>[category,await load(category)])
));
const args={
 compact:i=>[i.values],unique:i=>[i.values],chunk:i=>[i.values,i.size],
 flatten:i=>[i.values],last:i=>[i.values],isJsonValue:i=>[i.value],
 parseJson:i=>[i.text],tryParseJson:i=>[i.text],stringifyJson:i=>[i.value],
 clamp:i=>[i.value,i.min,i.max],inRange:i=>[i.value,i.min,i.max],
 roundTo:i=>[i.value,i.decimals],isFiniteNumber:i=>[i.value],sum:i=>[i.values],
 hasOwn:i=>[i.value,i.key],pick:i=>[i.value,i.keys],omit:i=>[i.value,i.keys],
 isPlainObject:i=>[i.value],isBlank:i=>[i.value],isNonBlank:i=>[i.value],
 isEmail:i=>[i.value],startsWithAny:i=>[i.value,i.prefixes],
 endsWithAny:i=>[i.value,i.suffixes],capitalize:i=>[i.value],
 camelCase:i=>[i.value],kebabCase:i=>[i.value],snakeCase:i=>[i.value],
 truncate:i=>[i.value,i.maxLength,i.suffix],
 truncateWords:i=>[i.value,i.maxWords,i.suffix],isUrl:i=>[i.value],
 joinUrl:i=>[i.base,...i.parts],getQueryParam:i=>[i.url,i.key],
 setQueryParam:i=>[i.url,i.key,i.value]
};
const results=JSON.parse(readFileSync(resolve(root,'rust-utility-results.json'),'utf8'));
if(results.length!==fixture.cases.length)throw new Error('Rust utility test count mismatch');
let passed=0;
const mismatches=[];
for(let n=0;n<fixture.cases.length;n++){
 const {id,input}=fixture.cases[n];
 const [namespace,category,symbol]=id.split('.');
 if(namespace!=='gutts'||!bundles[category]||typeof bundles[category][symbol]!=='function'){
  mismatches.push('Unrecognized pinned GutTS API '+id);continue;
 }
 let jsValue;
 let jsError=null;
 try {jsValue=bundles[category][symbol](...args[symbol](input));}
 catch(error) {jsError=error;}
 const actual=results[n];
 if(actual.id!==id){
  mismatches.push('Cross-language case order mismatch '+id);continue;
 }
 if(jsError){
  if(typeof actual.error!=='string'||!actual.error.length){
   mismatches.push('Rust accepted rejected input '+id+' '+JSON.stringify(input)
     +' JS='+String(jsError)+' Rust='+JSON.stringify(actual));
  }else{passed++;}
  continue;
 }
 if(Object.hasOwn(actual,'error')){
  mismatches.push('Rust rejected upstream-valid input '+id+' '+JSON.stringify(input)
    +' Rust='+actual.error+' JS='+JSON.stringify(jsValue));
  continue;
 }
 // Compare the JSON-serializable value domain, not JavaScript heap
 // representation. Sparse array holes are serialized to null, while Rust
 // serde_json arrays cannot store holes. Undefined is likewise mapped to null.
 const expected=jsValue===undefined?null:JSON.parse(JSON.stringify(jsValue));
 if(!isDeepStrictEqual(actual.actual,expected)){
  mismatches.push('Cross-language mismatch '+id+' '+JSON.stringify(input)
    +' JS='+JSON.stringify(expected)+' Rust='+JSON.stringify(actual.actual));
  continue;
 }
 passed++;
}
console.log('Verified '+passed+'/'+fixture.cases.length+' portable GutTS API cases via real Rust and TypeScript');
if(mismatches.length){
 throw new Error(mismatches.slice(0,40).join('\n')+(mismatches.length>40
   ?'\n... '+(mismatches.length-40)+' more mismatches':''));
}
