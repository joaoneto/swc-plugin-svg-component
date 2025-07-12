const path = require("node:path");

const swcConfig = {
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
    },
};

function mergeConfig(optionsConfig = {}, filename) {
    const resolvedPlugin = [path.resolve(__dirname, "dist/swc_plugin_svg_component.wasm"), {}];

    return {
        ...swcConfig,
        ...optionsConfig,
        module: {
            ...swcConfig.module,
            ...optionsConfig.module,
            type: optionsConfig.module?.type || swcConfig.module.type,
        },
        jsc: {
            ...swcConfig.jsc,
            ...optionsConfig.jsc,
            transform: {
                ...swcConfig.jsc.transform,
                ...optionsConfig.jsc?.transform,
                react: {
                    ...swcConfig.jsc.transform.react,
                    ...optionsConfig.jsc?.transform?.react,
                },
            },
            experimental: {
                ...swcConfig.jsc.experimental,
                ...optionsConfig.jsc?.experimental,
                plugins: [
                    ...(optionsConfig.jsc?.experimental?.plugins || []),
                    resolvedPlugin,
                ],
            },
            minify: {
                ...swcConfig.jsc.minify,
                ...optionsConfig.jsc?.minify,
            },
        },
        filename: path.basename(filename),
    };
}

module.exports.swcConfig = swcConfig;
module.exports.mergeConfig = mergeConfig;
