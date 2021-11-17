# Microsoft Flight Simulator 2020 - Mod Updater

This is an application to make it easy to download or update mods for msfs2020.

## Hoster

| Hoster                               | Download | Updates | Login needed? |
| ------------------------------------ | -------- | ------- | ------------- |
| Static link (check for hash change)  | ❌        | ❌       | no            |
| [github.com](https://github.com)     | ❌        | ❌       | no            |
| [flightsim.to](https://flightsim.to) | ❌        | ❌       | yes           |

✔️ = Done ⌛ = Work in Progress ❌ = Not implemented

# Contribution

## Requirements

Because MSFS2020 was developed by Microsoft and is playable only on windows, this application is only built for windows. So you need the following tools for windows. Install the tools and execute the commands in an admin powershell.

### [nvm-windows](https://github.com/coreybutler/nvm-windows)

```powershell
nvm install lts
nvm use lts
npm install -g yarn
```

### [Rustup](https://rustup.rs)

Install also the [c++ buildtools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (should be top left of the icons)

## Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn tauri:serve
```

### Compiles and minifies for production
```
yarn tauri:build
```

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
