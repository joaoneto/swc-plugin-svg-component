const fs = require("node:fs/promises");
const path = require("node:path");
const { transform } = require("@swc/core");
const swcConfig = require("./swcrc");

/**
 * @returns {import("vite").Plugin}
 */
module.exports = function svgComponentLoader() {
  return {
    name: "swc-plugin-svg-component",
    enforce: "pre",
    async transform(_, id) {
      if (!id.endsWith(".svg")) return;

      const svgContent = await fs.readFile(id, "utf-8");

      const result = await transform(svgContent, {
        ...swcConfig,
        filename: path.basename(id)
      });

      return {
        code: result.code,
        map: null
      };
    }
  };
}
