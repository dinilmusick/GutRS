#!/usr/bin/env node
// Independent pinned GutTS semantic-input resolution comparison.
// Only deterministic/empty/aborted requests; inference must never be mocked.
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {execFileSync} from 'node:child_process';

const root=process.cwd();
const fixture=JSON.parse(readFileSync(resolve(root,'testdata/upstream-golden/semantic-resolutions.json'),'utf8'));
const actualUpstream=execFileSync('git',['rev-parse','HEAD'],{cwd:resolve(root,'upstream'),encoding:'utf8'}).trim();
if(actualUpstream!==fixture.upstream)throw new Error('Incorrect pinned GutTS commit');
const semantic=await import(pathToFileURL(resolve(root,'upstream/dist/semantic-input/index.js')).href);
const rust=JSON.parse(readFileSync(resolve(root,'rust-semantic-results.json'),'utf8'));
if(rust.length!==fixture.cases.length)throw new Error('Incorrect native semantic case count');
for (let i=0;i<fixture.cases.length;i++){
 const test=fixture.cases[i];
 const request={...test.request};
 if(test.aborted)request.signal=AbortSignal.abort();
 const expected=JSON.parse(JSON.stringify(await semantic.resolveSemanticInput(request)));
 if(!isDeepStrictEqual(rust[i],expected)){
   throw new Error('Semantic contract mismatch #'+i+
    '\nJS='+JSON.stringify(expected)+'\nRust='+JSON.stringify(rust[i]));
 }
}
console.log('Verified '+fixture.cases.length+' pinned GutTS semantic resolution and provenance cases');
