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
for(let n=0;n<fixture.cases.length;n++){
 const {id,input}=fixture.cases[n];
 const [namespace,category,symbol]=id.split('.');
 if(namespace!=='gutts'||!bundles[category]||typeof bundles[category][symbol]!=='function'){
  throw new Error('Unrecognized pinned GutTS API '+id);
 }
 let jsValue;
 let jsError=null;
 try {
  jsValue=bundles[category][symbol](...args[symbol](input));
 } catch(error) {
  jsError=error;
 }
 const actual=results[n];
 if(actual.id!==id){
  throw new Error('Cross-language case order mismatch '+id);
 }
 if(jsError){
  if(typeof actual.error!=='string'||!actual.error.length){
   throw new Error('Rust did not reject invalid input for '+id+
     '\nJS error='+String(jsError)+'\nRust='+JSON.stringify(actual));
  }
  passed++;
  continue;
 }
 if(Object.hasOwn(actual,'error')){
  throw new Error('Rust rejected upstream-valid input for '+id+
    '\nRust error='+actual.error+'\nJS='+JSON.stringify(jsValue));
 }
 // The Rust JSON boundary has null rather than the JS-only undefined value.
 const expected=jsValue===undefined?null:jsValue;
 if(!isDeepStrictEqual(actual.actual,expected)){
  throw new Error('Cross-language mismatch '+id+
    '\nJS='+JSON.stringify(expected)+'\nRust='+JSON.stringify(actual.actual));
 }
 passed++;
}
console.log('Verified all '+passed+' portable GutTS API IDs via real Rust and TypeScript');
