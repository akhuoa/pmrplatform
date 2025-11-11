# PMR App UI - Vue Frontend

Vue 3 + TypeScript frontend for the Physiome Model Repository platform.

## Tech Stack

- **Vue 3** + **TypeScript** + **Vite**
- **Vue Router** - Routing
- **Pinia** - State management
- **Axios** - HTTP client for API calls

## Project Structure

```
src/
├── api/              # API client modules (auth, workspace, exposure)
├── components/       # Reusable Vue components
├── views/            # Page components
├── router/           # Vue Router configuration
├── stores/           # Pinia stores (auth, etc.)
├── types/            # TypeScript types (matching Rust models)
├── App.vue           # Root component
└── main.ts           # Entry point
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Vue (Official)](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

## Recommended Browser Setup

- Chromium-based browsers (Chrome, Edge, Brave, etc.):
  - [Vue.js devtools](https://chromewebstore.google.com/detail/vuejs-devtools/nhdogjmejiglipccpnnnanhbledajbpd)
  - [Turn on Custom Object Formatter in Chrome DevTools](http://bit.ly/object-formatters)
- Firefox:
  - [Vue.js devtools](https://addons.mozilla.org/en-US/firefox/addon/vue-js-devtools/)
  - [Turn on Custom Object Formatter in Firefox DevTools](https://fxdx.dev/firefox-devtools-custom-object-formatters/)

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) to make the TypeScript language service aware of `.vue` types.

## Customize configuration

See [Vite Configuration Reference](https://vite.dev/config/).

## Project Setup

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

Dev server runs at `http://localhost:5173` and proxies `/api/*` to Rust backend at `http://localhost:9380`.

### Type-Check, Compile and Minify for Production

```sh
npm run build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
npm run test:unit
```

### Lint with [ESLint](https://eslint.org/)

```sh
npm run lint
```
