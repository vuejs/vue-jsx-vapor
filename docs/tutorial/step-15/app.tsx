import { defineComponent, defineCustomElement, ref } from 'vue'

const UserCardElement = defineCustomElement(
  (props: { name: string }) => {
    const following = ref(false)
    return () => (
      <article>
        <slot name="avatar">
          <span class="avatar">?</span>
        </slot>
        <div class="content">
          <strong>{props.name}</strong>
          <button onClick={() => (following.value = !following.value)}>
            {following.value ? 'Following' : 'Follow'}
          </button>
        </div>
      </article>
    )
  },
  {
    props: {
      name: { type: String, required: true },
    },
    styles: [
      `:host { display: block; max-width: 320px; font-family: system-ui; }`,
      `article { display: flex; gap: 12px; align-items: center; border: 1px solid #d8dee4; border-radius: 8px; padding: 16px; }`,
      `.content { display: grid; gap: 8px; flex: 1; }`,
      `button { width: fit-content; border: 0; border-radius: 6px; padding: 6px 10px; color: white; background: #42b883; cursor: pointer; }`,
      `::slotted([slot="avatar"]), .avatar { display: grid; place-items: center; width: 44px; height: 44px; border-radius: 50%; color: white; background: #35495e; }`,
    ],
  },
)

export default defineComponent(() => {
  // Register UserCardElement as <tutorial-user-card> here.

  return () => (
    <tutorial-user-card name="Ada Lovelace">
      <span slot="avatar">AL</span>
    </tutorial-user-card>
  )
})
