import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vuetify from '@vuetify/vite-plugin';

export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          isCustomElement: tag => {
            return /^x-/.test(tag)
          }
        }
      }
    }),
    vuetify({
      autoImport: true
    })
  ],
  define: {
    'process.env': {}
  },
  rollupOptions: {
    output: {
      inlineDynamicImports: true
    }
  },
});
