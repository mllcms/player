import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    { path: "/", component: () => import("@/views/list.vue") },
    { path: "/video/:name", name: "video", component: () => import("@/views/video.vue") },
  ],
});

export default router;
