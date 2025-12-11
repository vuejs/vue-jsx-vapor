/* eslint-disable */
import { transformSync } from '@babel/core'
import vueJsxVapor from '@vue-jsx-vapor/babel'
import { transform as rsTransform } from '@vue-jsx-vapor/compiler-rs'
import vueJsx from '@vue/babel-plugin-jsx'
import { transformSync as oxcTransformSync } from 'oxc-transform'
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

vueJsxTransform(source)
console.time('vue-jsx           + babel  ')
vueJsxTransform(source)
console.timeEnd('vue-jsx           + babel  ')

vueJsxVaporTransform(source)
console.time('vue-jsx-vapor     + babel  ')
vueJsxVaporTransform(source)
console.timeEnd('vue-jsx-vapor     + babel  ')

rsTransform(source, { interop: true })
console.time('vue-jsx-vapor.rs  + oxc    ')
rsTransform(source)
console.timeEnd('vue-jsx-vapor.rs  + oxc    ')

rsTransform(source, { interop: true })
console.time('vue-jsx.rs        + oxc    ')
rsTransform(source, { interop: true })
console.timeEnd('vue-jsx.rs        + oxc    ')

oxcTransformSync('index.jsx', source)
console.time('react             + oxc    ')
oxcTransformSync('index.jsx', source)
console.timeEnd('react             + oxc    ')

bench.add('vue-jsx-vapor    + babel', () => {
  vueJsxVaporTransform(source)
})

bench.add('vue-jsx          + babel', () => {
  vueJsxTransform(source)
})

bench.add('vue-jsx-vapor.rs + oxc', () => {
  rsTransform(source)
})

bench.add('vue-jsx.rs       + oxc', () => {
  rsTransform(source, { interop: true })
})

bench.add('react            + oxc', () => {
  oxcTransformSync('index.jsx', source)
})

// await bench.run()

// console.table(bench.table())
