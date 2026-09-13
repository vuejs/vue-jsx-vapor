// docs/typescript/component-types.md 的示例；由 `tsc -b` 做类型检查
import type { SlotsToProps } from 'vue-jsx'

type ListProps<T> = { items: T[] } & SlotsToProps<{ row?: (item: T) => any }>

const List = <T,>(props: ListProps<T>) => <ul>{props.items.length}</ul>

export default () => <List items={[{ id: 1 }]} v-slots={{ row: (item) => <li>{item.id}</li> }} />
