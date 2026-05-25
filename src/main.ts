import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import App from "./App.vue";
import "./styles.css";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "patient-list",
      component: () => import("./views/PatientList.vue"),
    },
    {
      path: "/register",
      name: "patient-register",
      component: () => import("./views/PatientRegistration.vue"),
    },
    {
      path: "/patients/:tsid",
      name: "patient-detail",
      component: () => import("./views/PatientDetail.vue"),
    },
    {
      path: "/patients/:tsid/view",
      name: "patient-card-view",
      component: () => import("./views/PatientCardView.vue"),
    },
    {
      path: "/records/new",
      name: "medical-record-create",
      component: () => import("./views/MedicalRecordCreate.vue"),
    },
  ],
});

createApp(App).use(router).mount("#app");