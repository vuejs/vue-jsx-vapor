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

const source = `export default () => <>${`
  <Comp
    foo={foo}
    ref={foo}
    modelValue={foo}
    onUpdate:modelValue={e => foo = e}
    onClick={()=> alert(1)}
  >
    { foo
      ? list.map(({item}, index) => <div key={index}>{item}</div>)
      : bar
        ? <span>{bar}</span>
        : <Foo>
            {{
              default: () => <div>default</div>,
              bar: ({bar}) => <>{bar}</>
            }}
          </Foo> }
  </Comp>`.repeat(10)}
</>`

vueJsxVaporTransform(source)
console.time('vue-jsx-vapor     + babel  ')
vueJsxVaporTransform(source)
console.timeEnd('vue-jsx-vapor     + babel  ')

vueJsxTransform(source)
console.time('vue-jsx           + babel  ')
vueJsxTransform(source)
console.timeEnd('vue-jsx           + babel  ')

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

const bench = new Bench()

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

await bench.run()
console.table(bench.table())
