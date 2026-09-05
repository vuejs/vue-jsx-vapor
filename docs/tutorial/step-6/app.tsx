import { defineComponent, ref } from 'vue'
import { For } from 'vue-jsx'

interface Todo {
  id: number
  text: string
}

export default defineComponent(() => {
  let id = 0
  const newTodo = ref('')
  const todos = ref<Todo[]>([
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
  function removeTodo(todo: Todo) {
    todos.value = todos.value.filter((t) => t !== todo)
  }
  return () => (
    <>
      <form onSubmit={(event) => {
        event.preventDefault()
        addTodo()
      }}>
        <input
          value={newTodo.value}
          onInput={(e) => (newTodo.value = e.currentTarget.value)}
          required
          placeholder="new todo"
        />
        <button>Add Todo</button>
      </form>
      <ul>
        {(() => {
          const todo = todos.value[0]
          return (
            <li key={todo.id}>
              {todo.text}
              <button onClick={() => removeTodo(todo)}>X</button>
            </li>
          )
        })()}
      </ul>
    </>
  )
})
