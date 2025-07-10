const { transform } = require("@swc/core");
const swcConfig = require("./swcrc");

module.exports = function svgComponentLoader(contents) {
    const callback = this.async()

    const previousExport = (() => {
        if (contents.startsWith('export ')) return contents
        const exportMatches = contents.match(/^module.exports\s*=\s*(.*)/)
        return exportMatches ? `export default ${exportMatches[1]}` : null
    })()

    if (!previousExport) {
        transform(contents, swcConfig)
            .then((content) => callback(null, content.code))
    } else {
        this.fs.readFile(this.resourcePath, (err, result) => {
            if (err) {
                callback(err)
                return
            }
            transform(String(result), swcConfig)
                .then((content) => callback(null, content.code))
                .catch(callback)
        })
    }
}
