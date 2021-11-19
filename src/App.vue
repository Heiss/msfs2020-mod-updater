<template>
  <v-app id="inspire">
    <v-navigation-drawer permanent expand-on-hover>
      <v-list-item>
        <v-list-item-content>
          <v-list-item-title class="text-h6">MSFS 2020</v-list-item-title>
          <v-list-item-subtitle>Mod Updater</v-list-item-subtitle>
        </v-list-item-content>
        <v-spacer></v-spacer>
        <v-btn small icon>
          <v-icon large v-if="notifications">mdi-bell-badge</v-icon>
          <v-icon large v-else>mdi-bell</v-icon>
        </v-btn>
      </v-list-item>

      <v-divider></v-divider>

      <v-list dense nav>
        <v-list-item
          v-for="item in this.$router.options.routes"
          :key="item.title"
          :to="item.path"
          link
        >
          <v-list-item-icon>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-icon>

          <v-list-item-title>{{ item.title }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "App",
  data: () => ({
    drawer: null,
    notifications: false,
  }),
  mounted() {
    // Invoke the command
    invoke("my_custom_command", { invokeMessage: "Hello!" }).then((message) =>
      console.log(message)
    );
  },
  methods: {},
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
