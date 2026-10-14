import { ref } from 'vue'
import { defineComponent, defineVaporComponent } from 'vue-jsx'

type IsExact<A, B> =
  (<T>() => T extends A ? 1 : 2) extends <T>() => T extends B ? 1 : 2 ? true : false

/**
 * Doc guard for `docs/typescript/component-types.md`.
 *
 * Covers the default configuration (`macros: false`): `defineComponent` and
 * `defineVaporComponent` take slots, exposed members and events from the setup
 * context types, generics included - no `SlotsType` wrapper, no `emits` option,
 * no helper props. The macro-enabled path is documented separately because the
 * props rewrite drops the annotations these inferences read.
 */
const Panel = defineComponent(
  <T,>(
    props: { items: T[] },
    ctx: {
      emit: (e: 'change', v: T) => void
      slots: { default?: (item: T) => any; footer?: () => any }
      expose: (exposed?: { reset: () => void }) => void
    },
  ) => {
    ctx.expose({ reset: () => {} })
    ctx.emit('change', props.items[0]!)
    return () => <section>{props.items.length}</section>
  },
)

export function VSlots() {
  const r = ref<{ reset: () => void } | null>(null)
  return (
    <Panel
      items={[{ id: 1 }]}
      ref={r}
      onChange={(v) => {
        const emitPayloadIsGeneric: IsExact<typeof v, { id: number }> = true
      }}
      v-slots={{
        default: (item) => {
          const vSlotsParamIsGeneric: IsExact<typeof item, { id: number }> = true
          return <p>{item.id}</p>
        },
        footer: () => <p />,
      }}
    />
  )
}

export function Children() {
  return (
    <Panel
      items={[{ name: 'a' }]}
      onChange={(v) => {
        const emitPayloadIsGeneric: IsExact<typeof v, { name: string }> = true
      }}
    >
      {(item) => {
        const childrenParamIsGeneric: IsExact<typeof item, { name: string }> = true
        return <p>{item.name}</p>
      }}
    </Panel>
  )
}

export function RefTarget() {
  return (
    <Panel
      items={[{ id: 1 }]}
      ref={(e) => {
        const exposedMembersComeFromExpose: IsExact<typeof e, { reset: () => void } | null> = true
      }}
    />
  )
}

const VaporPanel = defineVaporComponent(
  <T,>(
    props: { items: T[] },
    ctx: {
      emit: (e: 'change', v: T) => void
      slots: { default?: (item: T) => any; footer?: () => any }
      expose: (exposed?: { reset: () => void }) => void
    },
  ) => {
    ctx.expose({ reset: () => {} })
    ctx.emit('change', props.items[0]!)
    return <section>{props.items.length}</section>
  },
)

// Multiple events: describe them in the emits option, not as an overloaded
// `emit` type - an intersection of two emit signatures loses the event props.
const Multi = defineComponent(
  (props: { n: number }) => {
    return () => <div>{props.n}</div>
  },
  {
    emits: {
      change: (v: number) => true,
      clear: () => true,
    },
  },
)

export function MultiEvent() {
  return (
    <Multi
      n={1}
      onChange={(v) => {
        const emitsOptionArgsAreTyped: IsExact<typeof v, number> = true
      }}
      onClear={() => {}}
    />
  )
}

export function VaporUsage() {
  return (
    <VaporPanel
      items={[{ id: 1 }]}
      onChange={(v) => {
        const emitPayloadIsGeneric: IsExact<typeof v, { id: number }> = true
      }}
      ref={(e) => {
        const exposedMembersComeFromExpose: IsExact<typeof e, { reset: () => void } | null> = true
      }}
    >
      {(item) => {
        const childrenParamIsGeneric: IsExact<typeof item, { id: number }> = true
        return <p>{item.id}</p>
      }}
    </VaporPanel>
  )
}
