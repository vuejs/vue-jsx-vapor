import { defineComponent, ref } from 'vue'

export const Comp = () => {
  const color = ref('red')
  defineStyle(`
    .foo {
      color: ${color.value};
    }
  `)
  return <div class="foo">foo</div>
}

export default defineComponent(() => {
  const color = ref('red')
  const styles = defineStyle.scss(`
    .bar {
      color: ${color.value};
      .bar-baz {
        background: red;
      }
    }
  `)
  const { default: Default, ...slots } = defineSlots()
  return () => (
    <>
      <div class={styles.bar}>foo</div>
      <div class="bar">
        <span class={styles['bar-baz']}>bar</span>
      </div>

      <Default />
      <slots.title />
    </>
  )
})

defineStyle.scss(`
  .bar {
    color: red;
  }
`)
