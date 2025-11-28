/* eslint-disable */
import { transformSync } from '@babel/core'
import vueJsxVapor from '@vue-jsx-vapor/babel'
import { transform as rsTransform } from '@vue-jsx-vapor/compiler-rs'
import vueJsx from '@vue/babel-plugin-jsx'
import { Bench } from 'tinybench'
function vueJsxTransform(source) {
  transformSync(source, {
    plugins: [vueJsx],
    filename: 'index.jsx',
    sourceMaps: false,
    sourceFileName: 'index.jsx',
    babelrc: false,
    configFile: false,
  })
}

function vueJsxVaporTransform(source) {
  transformSync(source, {
    plugins: [vueJsxVapor],
    filename: 'index.jsx',
    sourceMaps: false,
    sourceFileName: 'index.jsx',
    babelrc: false,
    configFile: false,
  })
}

const bench = new Bench()

const source = `export default () => <>${`
  <Comp
    foo={foo}
    ref={foo}
    onClick={()=> alert(1)}
    v-show={true}
    v-model={foo}
    v-once
    v-slot={foo}
  >
    <div
      v-if={foo}
      v-for={({item}, index) in list}
      key={key}
    >
      {item}
    </div>
    <span v-else-if={bar}>
      bar
    </span>
    <Foo v-else>
      default
      <template v-slot:bar={{ bar }}>
        {bar}
      </template>
    </Foo>
  </Comp>`.repeat(12)}
</>`

console.time('vue-jsx                    + babel-parser  ')
vueJsxTransform(source)
console.timeEnd('vue-jsx                    + babel-parser  ')

console.time('@vue-jsx-vapor/compiler    + babel-parser  ')
vueJsxVaporTransform(source)
console.timeEnd('@vue-jsx-vapor/compiler    + babel-parser  ')

console.time('@vue-jsx-vapor/compiler-rs + oxc-parser    ')
rsTransform(source)
console.timeEnd('@vue-jsx-vapor/compiler-rs + oxc-parser    ')

bench.add('vue-jsx + babel-parser', () => {
  vueJsxTransform(source)
})

bench.add('compiler-js + babel-parser', () => {
  vueJsxVaporTransform(source)
})

bench.add('compiler-rs + oxc-parser', () => {
  rsTransform(source)
})

await bench.run()

console.table(bench.table())
