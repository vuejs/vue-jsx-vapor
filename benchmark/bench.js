/* eslint-disable */
import { transformSync } from '@babel/core'
import vueJsxVapor from '@vue-jsx-vapor/babel'
import { transform as rsTransform } from '@vue-jsx-vapor/compiler-rs'
import vueJsx from '@vue/babel-plugin-jsx'
import { writeFileSync } from 'node:fs'
import path from 'node:path'
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

function reactTransform(source) {
  transformSync(source, {
    plugins: [
      ['@babel/plugin-transform-react-jsx', { throwIfNamespace: false }],
    ],
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
    ref={el => { foo = el }}
    modelValue={model}
    onUpdate:modelValue={$event => { model = $event }}
    onClick={() => alert(1)}
  >
    { foo
      ? list.map(({item}, index) => {
          return <div key={index}>{item}</div>
        })
      : bar
        ? <span>{bar}</span>
        : <Foo>
            {{
              default: () => <>default</>,
              bar: ({bar}) => <>{bar}</>
            }}
          </Foo> }
  </Comp>`.repeat(10)}
</>`

const bench = new Bench()

bench.add('vue-jsx-vapor + babel', () => {
  vueJsxVaporTransform(source)
})

bench.add('vue-jsx + babel', () => {
  vueJsxTransform(source)
})

bench.add('react + babel', () => {
  reactTransform(source)
})

bench.add('vue-jsx-vapor + oxc', () => {
  rsTransform(source)
})

bench.add('vue-jsx + oxc', () => {
  rsTransform(source, { interop: true })
})

bench.add('react + oxc', () => {
  oxcTransformSync('index.jsx', source)
})

await bench.run()
console.table(bench.table())

function getGroupName(fullName) {
  let name = fullName.replace(/['"]/g, '').trim()
  name = name.replace(' + oxc', '').replace(' + babel', '')
  return name
}

const groups = {}

bench.tasks.forEach((t) => {
  const groupName = getGroupName(t.name)
  const ops = Math.round(t.result.throughput.mean || 0)

  if (!groups[groupName]) {
    groups[groupName] = { name: groupName, babel: 0, oxc: 0 }
  }

  if (t.name.includes('babel')) groups[groupName].babel = ops
  if (t.name.includes('oxc')) groups[groupName].oxc = ops
})

const resultList = Object.values(groups).map((item) => {
  let multiplierText = ''
  if (item.babel > 0 && item.oxc > 0) {
    const x = (item.oxc / item.babel).toFixed(1)
    multiplierText = `${x}x`
  }
  return {
    ...item,
    multiplierText,
  }
})

resultList.sort((a, b) => b.oxc - a.oxc)

const outputPath = path.resolve(
  import.meta.dirname,
  '../docs/public/bench-results.json',
)
writeFileSync(outputPath, JSON.stringify(resultList, null, 2))
console.log('Bench data saved to', outputPath)
