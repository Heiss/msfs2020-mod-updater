import { createRouter, createWebHistory } from "vue-router";

import HomePage from './pages/home/home-page.vue';


export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      title: "Home",
      icon: "mdi-view-dashboard",
      component: HomePage,
    },
    {
      path: "/modules",
      title: "Modules",
      icon: "mdi-download",
      component: () => import('./pages/modules/index.vue')
    },
    {
      path: "/repositories",
      title: "Repositories",
      icon: "mdi-download",
      component: () => import('./pages/repositories/index.vue')
    },
    {
      path: '/about',
      title: "About",
      icon: "mdi-forum",
      component: () => import('./pages/about/about-us-page.vue' /* webpackChunkName: "about-us-page" */),
    },
  ]
});
