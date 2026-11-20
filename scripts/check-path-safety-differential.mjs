#!/usr/bin/env node
// Independent executable parity verification of the pinned GutTS path-safety exports.
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {execFileSync} from 'node:child_process';
import {isDeepStrictEqual} from 'node:util';

const root=process.cwd();
const fixture=JSON.parse(readFileSync(resolve(root,'testdata/upstream-golden/path-safety-cases.json'),'utf8'));
const current=execFileSync('git',['rev-parse','HEAD'],{cwd:resolve(root,'upstream'),encoding:'utf8'}).trim();
if(current!==fixture.upstream)throw new Error('Upstream source is not at pinned commit');
const source=await import(pathToFileURL(resolve(root,'upstream/dist/urls/path-safety.js')).href);
const rust=JSON.parse(readFileSync(resolve(root,'rust-path-safety-results.json'),'utf8'));
if(rust.length!==fixture.cases.length)throw new Error('Rust path case count mismatch');
for(let i=0;i<fixture.cases.length;i++){
 const {kind,path,origin}=fixture.cases[i];
 let expected;
 try{
   let value;
   if(kind==='normalize')value=source.normalizeStoragePath(path);
   else if(kind==='encode')value=source.encodeSafePathSegments(path);
   else if(kind==='resolve')value=source.resolveTrustedApiUrl(origin,path).href;
   else throw new Error('Unrecognized operation '+kind);
   expected={ok:true,value};
 }catch(e){
   expected={ok:false};
 }
 if(!isDeepStrictEqual(rust[i],expected)){
   throw new Error('Pinned GutTS path safety mismatch #'+i+' kind='+kind+
     '\ninput='+JSON.stringify(fixture.cases[i])+
     '\nexpected='+JSON.stringify(expected)+'\nRust='+JSON.stringify(rust[i]));
 }
}
console.log('Verified '+fixture.cases.length+' pinned path and trusted-origin URL cases');
