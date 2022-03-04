import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import ViteRsw from 'vite-plugin-rsw';

// https://vitejs.dev/config/
import { defineConfig } from 'vite';
import ViteRsw from 'vite-plugin-rsw';

export default defineConfig({
  plugins: [
    ViteRsw({
      crates: [
        '@rsw/hey', // npm org
        'rsw-test', // npm package
        // https://github.com/lencx/vite-plugin-rsw/issues/8#issuecomment-820281861
        // outDir: use `path.resolve` or relative path.
        // https://github.com/lencx/vite-plugin-rsw#plugin-options
        'wasm-test', // custom package name
        { name: '@rsw/hello', outDir: 'custom/path' },
      ],
    }),
  ],
});
