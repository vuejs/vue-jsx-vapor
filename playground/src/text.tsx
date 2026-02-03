import { defineVaporComponent, ref, shallowRef } from 'vue'

export default () => {
  const foo = shallowRef('foo')
  const bar = shallowRef('foo')

  const Comp = defineVaporComponent(() => <div>Comp</div>)

  return [
    <div>
      <button
        onClick={() => (
          (foo.value = <div>div</div>),
          (bar.value = <div>div</div>)
        )}
      >
        to div
      </button>
      <button
        onClick={() => ((foo.value = [1, 2, 3]), (bar.value = [1, 2, 3]))}
      >
        to fragment
      </button>
      <button
        onClick={() => (
          (foo.value = <Comp></Comp>),
          (bar.value = <Comp></Comp>)
        )}
      >
        to component
      </button>
      <button
        onClick={() => (
          (foo.value = (
            <>
              {[1, 2, 3].map((i) => (
                <div>{i}</div>
              ))}
            </>
          )),
          (bar.value = (
            <>
              {[1, 2, 3].map((i) => (
                <div>{i}</div>
              ))}
            </>
          ))
        )}
      >
        to array
      </button>
      <button onClick={() => ((foo.value = 'foo'), (bar.value = 'foo'))}>
        to text
      </button>
      <button onClick={() => ((foo.value = 'bar'), (bar.value = 'bar'))}>
        to text2
      </button>
      <div>
        <div>1</div>
        {foo.value}
        <div>2</div>
      </div>
      ------
    </div>,
    <div>3</div>,
    <>{bar.value}</>,
    <div>4</div>,
  ]
}
