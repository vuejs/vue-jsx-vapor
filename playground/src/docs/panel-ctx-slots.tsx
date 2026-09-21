// docs/typescript/component-types.md 的示例；由 `tsc -b` 做类型检查
const Panel = (props: { step: number }, { slots }: { slots: { default?: (n: number) => any } }) => (
  <div>{slots.default?.(props.step)}</div>
)

export default () => <Panel step={1}>{(n) => <span>{n.toFixed()}</span>}</Panel>
