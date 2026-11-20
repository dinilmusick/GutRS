#!/usr/bin/env node
/**
 * Differential certification: execute portable golden cases against the pinned
 * actual GutTS JavaScript build, not a mock or a hand-written Rust facsimile.
 * Failure means the Rust/source-derived expectation must be reconciled.
 */
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {isDeepStrictEqual} from 'node:util';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';

const root = process.cwd();
const upstream = resolve(root, 'upstream');
const fixtures = JSON.parse(readFileSync(resolve(root,'testdata/upstream-golden/portable.json'),'utf8'));
const actualCommit = execFileSync('git',['rev-parse','HEAD'],{cwd:upstream,encoding:'utf8'}).trim();
if(actualCommit !== fixtures.source.sha){
  throw new Error('Pinned GutTS revision mismatch: ' + actualCommit);
}
const load = async (path) => import(pathToFileURL(resolve(upstream,'dist',path)).href);
const [
  arrays,numbers,strings,urls,json,objects,identities,oauth
] = await Promise.all([
  load('arrays/array-concepts.js'),
  load('numbers/number-concepts.js'),
  load('strings/string-concepts.js'),
  load('urls/url-concepts.js'),
  load('json/json-concepts.js'),
  load('objects/object-concepts.js'),
  load('auth/identityIds.js'),
  load('auth/oauthClientContract.js')
]);

let count = 0;
for (const test of fixtures.cases) {
 const i=test.input;
 let actual;
 try {
  switch(test.symbol){
   case 'arrays.chunk': actual=arrays.chunk(i.values,i.size);break;
   case 'arrays.last': actual=arrays.last(i.values);break;
   case 'numbers.clamp': actual=numbers.clamp(i.value,i.min,i.max);break;
   case 'numbers.inRange': actual=numbers.inRange(i.value,i.min,i.max);break;
   case 'numbers.roundTo': actual=numbers.roundTo(i.value,i.decimals);break;
   case 'numbers.sum': actual=numbers.sum(i.values);break;
   case 'strings.isEmail': actual=strings.isEmail(i.value);break;
   case 'strings.capitalize': actual=strings.capitalize(i.value);break;
   case 'strings.camelCase': actual=strings.camelCase(i.value);break;
   case 'strings.kebabCase': actual=strings.kebabCase(i.value);break;
   case 'strings.snakeCase': actual=strings.snakeCase(i.value);break;
   case 'strings.isBlank': actual=strings.isBlank(i.value);break;
   case 'strings.startsWithAny': actual=strings.startsWithAny(i.value,i.prefixes);break;
   case 'strings.endsWithAny': actual=strings.endsWithAny(i.value,i.suffixes);break;
   case 'strings.truncateWords': actual=strings.truncateWords(i.value,i.maxWords,i.suffix);break;
   case 'urls.joinUrl': actual=urls.joinUrl(i.base,...i.parts);break;
   case 'urls.getQueryParam': actual=urls.getQueryParam(i.url,i.key);break;
   case 'urls.setQueryParam': actual=urls.setQueryParam(i.url,i.key,i.value);break;
   case 'json.parseJson': actual=json.parseJson(i.text);break;
   case 'json.tryParseJson': actual=json.tryParseJson(i.text);break;
   case 'json.stringifyJson': actual=json.stringifyJson(i.value);break;
   case 'objects.hasOwn': actual=objects.hasOwn(i.value,i.key);break;
   case 'objects.pick': actual=objects.pick(i.value,i.keys);break;
   case 'auth.parseIdentityId': actual=identities.parseIdentityId(i.kind,i.id);break;
   case 'auth.parseIdentityRef': actual=identities.parseIdentityRef(i.ref);break;
   case 'auth.validateOAuthRedirectUri':{
     const result=oauth.validateOAuthRedirectUri(i.url,i.platform);
     actual=result.ok?result.value:{error:result.reason};
     break;
   }
   default:throw new Error('Fixture has no upstream executable for '+test.symbol);
  }
 }catch(error){
  if(test.symbol==='arrays.chunk' && error instanceof RangeError)actual={error:'InvalidSize'};
  else if(test.symbol==='numbers.roundTo' && error instanceof RangeError)actual={error:'NegativePrecision'};
  else throw error;
 }
 // The JSON fixture wire format cannot represent JS undefined. For the
 // tryParseJson miss only, convert undefined to null *in the fixture protocol*.
 // This is not a claim that upstream returns null.
 if(test.symbol==='json.tryParseJson' && actual===undefined)actual=null;
 if(!isDeepStrictEqual(actual,test.expected)){
  throw new Error('GutTS differential mismatch '+test.symbol
    +' input='+JSON.stringify(i)
    +' expected='+JSON.stringify(test.expected)
    +' actual='+JSON.stringify(actual));
 }
 count++;
}
if(count!==fixtures.cases.length || count===0)throw new Error('Incomplete golden execution');
console.log('Executed '+count+' upstream golden cases on pinned GutTS '+actualCommit);

const [upstreamApis,upstreamModels,upstreamFod] = await Promise.all([
  load('api/api-registry.js'),
  load('models/model-registry.js'),
  load('api/fod-node-provider.js')
]);
const pinnedApiBlueprints = JSON.parse(readFileSync(
  resolve(root,'testdata/upstream-golden/api-blueprints.json'),'utf8'));
const actualBlueprints = upstreamApis.guttsApiDefinitions.map(({inputSchema, ...blueprint}) => blueprint);
if (!isDeepStrictEqual(pinnedApiBlueprints, actualBlueprints)) {
  throw new Error('Pinned API blueprint snapshot differs from actual GutTS exports');
}
const rustRegistry = JSON.parse(readFileSync(resolve(root,'rust-registry.json'),'utf8'));
const comparisons = [
  ['API registry',rustRegistry.apis,upstreamApis.guttsApiDefinitions],
  ['FOD node registry',rustRegistry.fod,upstreamFod.guttsFodNodeDefinitions],
  ['model metadata',rustRegistry.models,upstreamModels.guttsModelDefinitions]
];
for(const [label, rust, typescript] of comparisons){
 if(!isDeepStrictEqual(rust, typescript)){
   const index=rust.findIndex((item,i)=>!isDeepStrictEqual(item,typescript[i]));
   const at=index<0?0:index;
   throw new Error('GutTS differential mismatch: '+label+' index='+at
     +' expected='+JSON.stringify(typescript[at])
     +' actual='+JSON.stringify(rust[at]));
 }
 console.log('Certified '+label+': '+rust.length+' pinned exports');
}
