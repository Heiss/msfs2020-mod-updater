<template>
  <img alt="Vue logo" src="./assets/logo.png" />
  <HelloWorld msg="Welcome to Your Vue.js App" />
</template>

<script>
import HelloWorld from "./components/HelloWorld.vue";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "App",
  components: {
    HelloWorld,
  },
  mounted() {
    // With the Tauri API npm package:
    // With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
    //const invoke = window.__TAURI__.invoke;

    // Invoke the command
    invoke("my_custom_command", { invokeMessage: "Hello!" }).then((message) =>
      console.log(message)
    );
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
