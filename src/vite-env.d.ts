/// <reference types="vite/client" />
interface Window {
  API_URL: string;
}
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}
