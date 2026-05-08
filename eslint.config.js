import eslint from "@eslint/js";
import tseslint from "@typescript-eslint/eslint-plugin";
import tsparser from "@typescript-eslint/parser";
import sveltePlugin from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";
import globals from "globals";

export default [
  // Archivos a ignorar
  {
    ignores: ["node_modules/**", "dist/**", "build/**", ".svelte-kit/**"],
  },

  // Base JS
  eslint.configs.recommended,

  // Archivos TypeScript
  {
    files: ["**/*.ts"],
    languageOptions: {
      parser: tsparser,
      parserOptions: {
        project: "./tsconfig.app.json",
        extraFileExtensions: [".svelte"],
      },
      globals: {
        ...globals.browser,
        ...globals.es2023,
      },
    },
    plugins: {
      "@typescript-eslint": tseslint,
    },
    rules: {
      // Evita falsos positivos del core en TS
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
      // Cualquier any explícito es error
      "@typescript-eslint/no-explicit-any": "error",
      // Fuerza anotar el retorno de funciones
      "@typescript-eslint/explicit-function-return-type": "warn",
      // No asignaciones inseguras
      "@typescript-eslint/no-unsafe-assignment": "error",
      "@typescript-eslint/no-unsafe-member-access": "error",
      "@typescript-eslint/no-unsafe-call": "error",
      "@typescript-eslint/no-unsafe-return": "error",
      // No usar variables antes de definirlas
      "@typescript-eslint/no-use-before-define": "error",
      // Fuerza import type cuando es solo un tipo
      "@typescript-eslint/consistent-type-imports": "error",
    },
  },

  // Archivos Svelte
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parser: svelteParser,
      parserOptions: {
        parser: tsparser,
        project: "./tsconfig.app.json",
        extraFileExtensions: [".svelte"],
      },
      globals: {
        ...globals.browser,
        ...globals.es2023,
      },
    },
    plugins: {
      "@typescript-eslint": tseslint,
      svelte: sveltePlugin,
    },
    rules: {
      // Reglas de Svelte
      ...sveltePlugin.configs.recommended.rules,
      // Evita falsos positivos del core en TS dentro de .svelte
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],
      // Las mismas reglas de TypeScript para archivos .svelte
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/explicit-function-return-type": "warn",
      "@typescript-eslint/no-unsafe-assignment": "error",
      "@typescript-eslint/no-unsafe-member-access": "error",
      "@typescript-eslint/no-unsafe-call": "error",
      "@typescript-eslint/no-unsafe-return": "error",
      "@typescript-eslint/consistent-type-imports": "error",
    },
  },
];
