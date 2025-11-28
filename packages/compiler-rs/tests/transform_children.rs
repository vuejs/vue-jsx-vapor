use compiler_rs::transform::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform(
    "<div>
      {foo} {bar}
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn comments() {
  let code = transform("<>{/*foo*/}<div>{/*bar*/}</div></>", None).code;
  assert_snapshot!(code);
}

#[test]
fn fragment() {
  let code = transform("<>{foo}</>", None).code;
  assert_snapshot!(code);
}

#[test]
fn children_sibling_references() {
  let code = transform(
    "<div id={id}>
      <p>{ first }</p>
      123 { second } 456 {foo}
      <p>{ forth }</p>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn efficient_traversal() {
  let code = transform(
    "<div>
      <div>x</div>
      <div><span>{{ msg }}</span></div>
      <div><span>{{ msg }}</span></div>
      <div><span>{{ msg }}</span></div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn efficient_find() {
  let code = transform(
    "<div>
      <div>x</div>
      <div>x</div>
      <div>{ msg }</div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn anchor_insertion_in_middle() {
  let code = transform(
    "<div>
      <div></div>
      <div v-if={1}></div>
      <div></div>
    </div>",
    None,
  )
  .code;
  // ensure the insertion anchor is generated before the insertion statement
  assert_snapshot!(code);
}

#[test]
fn jsx_component_in_jsx_expression_container() {
  let code = transform(
    "<div>
      {<Comp />}
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn next_child_and_nthchild_should_be_above_the_set_insertion_state() {
  let code = transform(
    "<div>
      <div />
      <Comp />
      <div />
      <div v-if={true} />
      <div>
        <button disabled={foo} />
      </div>
    </div>",
    None,
  )
  .code;
  assert_snapshot!(code);
}
