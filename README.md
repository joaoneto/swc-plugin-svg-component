# swc-plugin-svg-component

Plugin SWC minimalista para transformar arquivos SVG em componentes React de forma rápida e integrada ao pipeline de build.  
Desenvolvido pensando em fluxos modernos com SWC e Next.js, inclusive com Turbopack.

Este projeto é uma alternativa de propósito restrito ao SVGR, que é muito mais completo e amplamente testado. O objetivo aqui é ter um plugin enxuto, escrito em Rust e exportado como WebAssembly, que transforma SVGs em componentes React válidos.  

Ele cuida apenas do essencial: substitui atributos problemáticos como `class` por `className`, converte `style` em objeto JavaScript e normaliza nomes de atributos com hífen para camelCase, por exemplo `stroke-width` vira `strokeWidth`.

A ideia é oferecer um fluxo de transformação direto no compilador, sem etapas manuais ou scripts adicionais além do build.

> [!WARNING] 
> Este plugin é experimental e pode não ser estável para produção. Seu funcionamento depende de partes internas do SWC e de integrações específicas com o Next.js. Mudanças frequentes nesses projetos podem quebrar o plugin sem aviso. Use por sua conta e risco, especialmente em ambiente de produção. Para projetos que exigem compatibilidade garantida e suporte a todos os edge cases de SVG no React, recomenda-se utilizar o [SVGR](https://react-svgr.com/).

> [!NOTE]
> Este repositório está em fase inicial e serve como experimento para testar o poder do SWC com transformações via plugin em Rust. Ele é útil como laboratório para estudar fluxos de build modernos, criação de plugins WASM e integração com pipelines como o Turbopack.

## Instalação

Adicione o pacote ao seu projeto:

```bash
npm install swc-plugin-svg-component
````

---

## Como usar

### Build com .swcrc
No seu `.swcrc` ou configuração SWC customizada, adicione este plugin:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["swc-plugin-svg-component", {}]
      ]
    }
  }
}
```

Para compilar, rode o swc cli incluindo a extensão .svg:

```bash
swc src -d dist --extensions .ts,.js,.tsx,.jsx,.svg
```

### Build com Next.js 15+
No seu `next.cofig.ts`, adicione o loader para turbopack:

```typescript
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  turbopack: {
    rules: {
      '*.svg': {
        loaders: [
          {
            loader: "swc-plugin-svg-component/turbopack",
            options: {
              icon: true,
            },
          },
        ],
        as: '*.js',
      },
    },
  },
};

export default nextConfig;
```

Inicie o script dev do Next.js:
```
npm run dev
```

**Exemplo de uso:**
Importe um SVG como componente React:

```jsx
import MyIcon from './icon.svg';

export default function Page() {
  return <MyIcon width={24} height={24} />;
}
```

---

## Desenvolvimento

### Build do plugin

Para compilar (com cargo) o plugin e gerar o `/dist`, utilize o npm script:

```bash
npm run build
```

