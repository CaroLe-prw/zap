// eslint.config.mjs
// @ts-check

import js from "@eslint/js";
import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";
import vue from "eslint-plugin-vue";
import vueParser from "vue-eslint-parser";
import prettierConfig from "eslint-config-prettier";
import globals from "globals";

export default defineConfig(
  // 0) 忽略目录（按你项目调整）
  {
    ignores: [
      "dist/**",
      "node_modules/**",
      "src-tauri/**",
      "target/**",
      ".output/**",
      ".vite/**",
    ],
  },

  // 1) JS 推荐规则
  js.configs.recommended,

  // 1.1) 全局变量（解决 process/no-undef 等）
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },

  // 2) TS 推荐规则
  tseslint.configs.recommended,

  // 3) Vue 推荐规则（flat configs 是数组，需要 spread 合并）
  ...vue.configs["flat/recommended"],

  // 4) 让 .vue 的 <script lang="ts"> 用 TS parser
  {
    files: ["**/*.vue"],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        extraFileExtensions: [".vue"],
        ecmaVersion: "latest",
        sourceType: "module",
      },
    },
  },

  // 5) 你的“实用规则”（尽量不和 Prettier 打架）
  {
    rules: {
      // 通用质量
      eqeqeq: ["error", "always"],
      curly: ["error", "all"],
      "no-var": "error",
      "prefer-const": "error",

      // 开发时允许 warn/error；log 给 warn
      "no-console": ["warn", { allow: ["warn", "error"] }],
      "no-debugger": process.env.NODE_ENV === "production" ? "error" : "warn",

      // TS：忽略 _xxx 未使用
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
          ignoreRestSiblings: true,
        },
      ],
      // any 给 warn（不阻塞开发）
      "@typescript-eslint/no-explicit-any": "warn",

      // 鼓励 type import
      "@typescript-eslint/consistent-type-imports": [
        "warn",
        { prefer: "type-imports", fixStyle: "separate-type-imports" },
      ],

      // Vue：更适合个人/桌面 app
      "vue/multi-word-component-names": "off",
      "vue/no-unused-components": "warn",
      "vue/no-unused-vars": "warn",
      "vue/no-v-html": "warn",
      "vue/require-default-prop": "off",
    },
  },

  // 6) 放宽 .d.ts（比如 vite-env.d.ts 这种声明文件）
  {
    files: ["**/*.d.ts"],
    rules: {
      "@typescript-eslint/no-empty-object-type": "off",
      "@typescript-eslint/no-explicit-any": "off",
    },
  },

  // 7) 放宽配置文件（你也可以删掉这段，选择把 == 改成 ===）
  {
    files: ["vite.config.ts", "**/*.config.*"],
    rules: {
      eqeqeq: "off",
    },
  },

  // 8) 必须最后：关闭和 Prettier 冲突的 ESLint 规则
  prettierConfig,
);
