---
aside: false
prev: false
next: false
---

# Yoou Did it!
  
<script setup>
import appCode from './app.tsx?raw'
import { getDefaultFiles } from '../template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
}
</script>

<jsx-repl :files :apps prev="/tutorial/step-11">

You have finished the tutorial!

At this point, you should have a good idea of what it's like to work with Vue JSX Vapor. However, we covered a lot of things really fast and glossed over the details, so definitely keep learning! As a next step, you can:

- Set up a real Vue JSX Vapor project on your machine by downloading the [vitesse-jsx-vapor](https://github.com/zhiyuanzmj/vitesse-jsx-vapor) template.

- Set up a real SSR project on your machine by downloading the [vue-jsx-vapor-ssr](https://github.com/zhiyuanzmj/vue-jsx-vapor-ssr) template.

</jsx-repl>
