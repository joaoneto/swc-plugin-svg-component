const path = require("node:path");
const { transform } = require("@swc/core");

const defaultSwcConfig = {
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
                importSource: "react"
            }
        },
        loose: false,
        externalHelpers: false,
        experimental: {
            plugins: [
                [path.resolve(__dirname, "dist/swc_plugin_svg_component.wasm"), {}]
            ]
        }
    }
};

module.exports = function svgrLoader(contents) {
    const callback = this.async()

    const previousExport = (() => {
        if (contents.startsWith('export ')) return contents
        const exportMatches = contents.match(/^module.exports\s*=\s*(.*)/)
        return exportMatches ? `export default ${exportMatches[1]}` : null
    })()

    if (!previousExport) {
        transform(contents, defaultSwcConfig)
            .then((content) => callback(null, content.code))
    } else {
        this.fs.readFile(this.resourcePath, (err, result) => {
            if (err) {
                callback(err)
                return
            }
            transform(String(result), defaultSwcConfig)
                .then((content) => callback(null, content.code))
                .catch(callback)
        })
    }
}
