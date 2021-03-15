// Because WASM can not be loaded by the browser natively,
// a fetch must happen. This means when using WASM, you must
// do it asynchronously.
//
// We are bootstraping the entire 'main.ts' asynchronously to
// get around this.
import('./main').catch(() => alert('Unable to load.'));
