const fs = require("node:fs/promises");
const { transform } = require("@swc/core");
const { mergeConfig } = require("./swcrc");

module.exports = function svgComponentLoader(options = {}) {
  return {
    name: "swc-plugin-svg-component",
    enforce: "pre",
    async transform(_, id) {
      if (!id.endsWith(".svg")) return;

      const svgContent = await fs.readFile(id, "utf-8");

      const result = await transform(svgContent, mergeConfig(options, id));

      return {
        code: result.code,
        map: null
      };
    }
  };
}
