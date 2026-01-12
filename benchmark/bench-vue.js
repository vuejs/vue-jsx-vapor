/* eslint-disable */
import { transform as rsTransform } from '@vue-jsx-vapor/compiler-rs'
import { Bench } from 'tinybench'
import { compileTemplate } from 'vue/compiler-sfc'

function vueTransform(source, options = {}) {
  compileTemplate({
    source,
    id: 'foo.vue',
    ...options,
  })
}

const source = `export default () => <>${`
  <Comp
    foo={foo}
    ref={el => { foo = el }}
    v-model={model}
    onClick={() => alert(1)}
  >
    <div v-if={foo} v-for={({item}, index) in list} key={index}>
      {item}
    </div>
    <span v-else-if={bar}>{bar}</span>
    <Foo v-else>
      default
      <template v-slot:bar={{ bar }}>
        {bar}
      </template>
    </Foo>
  </Comp>`.repeat(10)}
</>`

const vueSource = `<template>${`
  <Comp
    :foo="foo"
    :ref="foo"
    v-model="foo"
    onClick="() => alert(1)"
  >
    <div v-if="foo" v-for="({item}, index) in list" :key="index">
      {{ item }}
    </div>
    <span v-else-if="bar">{{bar}}</span>
    <Foo v-else>
      default
      <template #bar="{ bar }">
        {{ bar }}
      </template>
    </Foo>
  </Comp>`.repeat(10)}
</template>
`

const bench = new Bench()

bench.add('vue-vapor', () => {
  vueTransform(vueSource, { vapor: true })
})

bench.add('vue', () => {
  vueTransform(vueSource)
})

bench.add('vue-jsx-vapor.rs + oxc', () => {
  rsTransform(source)
})

bench.add('vue-jsx.rs       + oxc', () => {
  rsTransform(source, { interop: true })
})

await bench.run()
console.table(bench.table())
