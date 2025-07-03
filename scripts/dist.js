const fs = require('fs');
const path = require('path');

const CWD = process.cwd();

const source = path.resolve(CWD, 'target/wasm32-unknown-unknown/release/swc_plugin_svg_component.wasm');
const destDir = path.resolve(CWD, 'dist');

if (!fs.existsSync(destDir)) {
    fs.mkdirSync(destDir);
}

const dest = path.join(destDir, 'swc_plugin_svg_component.wasm');

fs.copyFileSync(source, dest);

console.log(`Copiado: ${path.relative(CWD, source)} -> ${path.relative(CWD, dest)}`);
