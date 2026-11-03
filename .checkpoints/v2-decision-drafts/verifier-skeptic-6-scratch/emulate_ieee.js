// skeptic-6: std's div_euclid steps (q = trunc(a / b); if a % b < 0 then q -/+ 1) under IEEE 754 semantics in Node.
// Math.fround(a / b) on two f32 values is the correctly rounded f32 quotient (53 >= 2*24 + 2); JS % is the exact fmod.
// An emulation of the algorithm, not Rust compiled to Wasm.
const f = Math.fround;
function de(a, b, R) { const q = Math.trunc(R(a / b)); const r = a % b; return r < 0 ? (b > 0 ? q - 1 : q + 1) : q; }
const a32 = f(3 + 2 ** -22), b32 = f(1 + 2 ** -23);
console.log('f32 emu: exact?', a32 === 3 + 2 ** -22, b32 === 1 + 2 ** -23, 'div_euclid =', de(a32, b32, f), 'rem =', a32 % b32);
const a64 = 3 + 2 ** -51, b64 = 1 + 2 ** -52;
console.log('f64 emu: div_euclid =', de(a64, b64, x => x), 'rem =', a64 % b64);
