import init, { compile } from '../pkg/python2javascript.js';

await init();
const code = await document.querySelector('script[type="python"]').innerText;
console.log(compile(code));