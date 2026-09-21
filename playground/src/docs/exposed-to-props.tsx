// docs/typescript/component-types.md 的示例；由 `tsc -b` 做类型检查
import type { ExposedToProps } from 'vue-jsx'

type CardProps<T> = { value: T } & ExposedToProps<{ reset: () => void }>

const Card = <T,>(props: CardProps<T>) => <div>{props.value}</div>

export default () => <Card value={1} ref={(exposed) => exposed?.reset()} />
