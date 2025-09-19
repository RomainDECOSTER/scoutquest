import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  sourcemap: true,
  clean: true,
  splitting: false,
  minify: false,
  external: ['axios', 'ws'],
  outDir: 'dist',
  target: 'es2020',
  tsconfig: './tsconfig.json',
});
