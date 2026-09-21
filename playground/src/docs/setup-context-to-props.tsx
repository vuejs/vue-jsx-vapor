// docs/typescript/component-types.md 的示例；由 `tsc -b` 做类型检查
import type { SetupContextToProps } from 'vue-jsx'

type ListProps<T> = { items: T[] } & SetupContextToProps<
  { change: [value: T] },
  { row?: (item: T) => any },
  { first: T }
>

const List = <T,>(props: ListProps<T>) => <ul>{props.items.length}</ul>

export default () => (
  <List
    items={[1, 2, 3]}
    onChange={(value) => value.toFixed()}
    ref={(exposed) => exposed?.first.toFixed()}
  >
    {{ row: (item) => <li>{item.toFixed()}</li> }}
  </List>
)
