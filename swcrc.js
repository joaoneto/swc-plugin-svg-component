const path = require("node:path");

module.exports = {
    module: {
        type: "es6"
    },
    jsc: {
        parser: {
            syntax: "ecmascript",
            jsx: true,
        },
        transform: {
            react: {
                runtime: "automatic",
                importSource: "react",
            },
        },
        loose: false,
        externalHelpers: false,
        experimental: {
            plugins: [
                [path.resolve(__dirname, "dist/swc_plugin_svg_component.wasm"), {}],
            ],
        },
        minify: {
            compress: false,
            mangle: false,
        },
    },
};
