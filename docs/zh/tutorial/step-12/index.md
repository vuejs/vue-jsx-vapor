---
aside: false
prev: false
next: false
---

# 恭喜完成!
  
<script setup>
import appCode from '~/tutorial/step-12/app.tsx?raw'
import { getDefaultFiles } from '~/tutorial/template'
import { ref } from 'vue'

const files = ref(getDefaultFiles())
const apps  = {
  app: { 'src/App.tsx': appCode },
}
</script>

<jsx-repl :files :apps prev="/zh/tutorial/step-11/">

你已经完成了本教程！

到目前为止，你应该对使用 Vue JSX Vapor 的体验有了很好的了解。然而，我们快速地涵盖了很多内容并略过了一些细节，所以一定要继续学习！作为下一步，你可以：

- 通过下载 [vitesse-jsx-vapor](https://github.com/zhiyuanzmj/vitesse-jsx-vapor) 模板在你的机器上搭建一个真正的 Vue JSX Vapor 项目。

- 通过下载 [vue-jsx-vapor-ssr](https://github.com/zhiyuanzmj/vue-jsx-vapor-ssr) 模板在你的机器上搭建一个真正的 SSR 项目。

</jsx-repl>