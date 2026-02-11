import { defineComponent, ref } from 'vue'

export default defineComponent(() => {
  let id = 0
  const newTodo = ref('')
  const todos = ref([
    { id: id++, text: 'Learn HTML' },
    { id: id++, text: 'Learn JavaScript' },
    { id: id++, text: 'Learn Vue' },
  ])
  function addTodo() {
    todos.value.push({
      id: id++,
      text: newTodo.value,
    })
    newTodo.value = ''
  }
  function removeTodo(todo) {
    todos.value = todos.value.filter((t) => t !== todo)
  }
  return () => (
    <>
      <form onSubmit_prevent={addTodo}>
        <input
          value={newTodo.value}
          onInput={(e) => (newTodo.value = e.currentTarget.value)}
          required
          placeholder="new todo"
        />
        <button>Add Todo</button>
      </form>
      <ul>
        <li v-for={todo in todos.value} key={todo.id}>
          {todo.text}
          <button onClick={() => removeTodo(todo)}>X</button>
        </li>
      </ul>
    </>
  )
})
