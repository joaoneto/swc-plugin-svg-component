/// <reference types="node" />

import type { Plugin as VitePlugin } from "vite";
import type { Config as SwcConfig, TransformOutput } from "@swc/core";

export interface SvgComponentLoaderOptions extends SwcConfig {}

/**
 * Cria o plugin Vite para transformar arquivos SVG em componentes React.
 * @param options Configuração SWC extra
 * @returns Plugin Vite
 */
export declare function svgComponentLoader(options?: SvgComponentLoaderOptions): VitePlugin;
export default svgComponentLoader;