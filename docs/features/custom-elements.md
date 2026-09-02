# Custom Elements

Vue JSX can both consume existing Custom Elements and define new ones. The same JSX syntax works with Virtual DOM and Vapor Mode.

## Using a Custom Element

Lowercase tag names containing a hyphen are recognized as Custom Elements automatically. No `isCustomElement` compiler option is required.

```tsx
export default () => (
  <user-card name="Ada Lovelace">
    <span slot="avatar">AL</span>
  </user-card>
)
```

The standard `slot` attribute can be used to provide content to a named native slot.

> [!TIP]
> Custom Element names must contain a hyphen. This is also how Vue JSX distinguishes `<user-card>` from native HTML elements and Vue components such as `<UserCard>`.

## Defining a Custom Element

Use Vue's `defineCustomElement` for Virtual DOM output or `defineVaporCustomElement` for Vapor output.

::: code-group

```tsx [Virtual DOM]
import { defineCustomElement } from 'vue'

const UserCard = defineCustomElement(
  (props: { name: string }) => () => (
    <article>
      <slot name="avatar" />
      <strong>{props.name}</strong>
    </article>
  ),
  {
    props: {
      name: { type: String, required: true },
    },
  },
)
```

```tsx [Vapor Mode]
import { defineVaporCustomElement } from 'vue'

const UserCard = defineVaporCustomElement(
  (props: { name: string }) => (
    <article>
      <slot name="avatar" />
      <strong>{props.name}</strong>
    </article>
  ),
  {
    props: {
      name: { type: String, required: true },
    },
  },
)
```

:::

Register the constructor with the browser before using its tag:

```ts
if (!customElements.get('user-card')) {
  customElements.define('user-card', UserCard)
}
```

The guard is useful during development because the Custom Elements registry does not allow the same name to be defined twice. Register browser-only elements from a client entry when using SSR.

## Props and Attributes

A TypeScript annotation describes props to the type checker. The runtime `props` option tells Vue which HTML attributes to observe and enables conversions such as `Number` and `Boolean`.

```tsx
const ProgressRing = defineVaporCustomElement(
  (props: { value: number; compact: boolean }) => (
    <output>{props.compact ? props.value : `${props.value}%`}</output>
  ),
  {
    props: {
      value: Number,
      compact: Boolean,
    },
  },
)
```

```html
<progress-ring value="75" compact></progress-ring>
```

## Events

Events emitted by a Vue Custom Element are dispatched as native `CustomEvent` objects. Emitted arguments are available through `event.detail`.

```tsx
const UserCard = defineVaporCustomElement(
  (props: { name: string }, { emit }) => (
    <button onClick={() => emit('select', props.name)}>{props.name}</button>
  ),
  {
    props: { name: String },
    emits: ['select'],
  },
)

export default () => (
  <user-card
    name="Ada"
    onSelect={(event: CustomEvent<[string]>) => {
      console.info(event.detail[0])
    }}
  />
)
```

## Shadow DOM and Styles

Vue Custom Elements use Shadow DOM by default. Pass CSS through `styles`, or set `shadowRoot: false` when the element must render into the light DOM.

```tsx
const UserCard = defineVaporCustomElement(render, {
  styles: [
    `:host { display: block; }`,
    `article { border: 1px solid #d8dee4; padding: 12px; }`,
  ],
  shadowRoot: true,
})
```

## Adding Strict JSX Types

Unknown Custom Element names are accepted so third-party elements work without setup. A library or application can augment `JSX.IntrinsicElements` to validate a specific element's props and events:

```ts
declare module 'vue-jsx' {
  namespace JSX {
    interface IntrinsicElements {
      'user-card': {
        name: string
        compact?: boolean
        onSelect?: (event: CustomEvent<[string]>) => void
      }
    }
  }
}
```

After augmentation, `<user-card>` receives completion and type checking while other Custom Elements remain available.
