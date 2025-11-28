use std::cell::RefCell;

use compiler_rs::{
  transform::{TransformOptions, transform},
  utils::error::ErrorCodes,
};
use insta::assert_snapshot;

#[test]
fn implicit_default_slot() {
  let code = transform("<Comp><div/></Comp>", None).code;
  assert_snapshot!(code);
}

#[test]
fn on_component_default_slot() {
  let code = transform("<Comp v-slot={scope}>{ scope.foo + bar }</Comp>", None).code;
  assert_snapshot!(code);

  assert!(code.contains("default: (scope) =>"));
  assert!(code.contains("scope.foo + bar"));
}

#[test]
fn on_component_named_slot() {
  let code = transform(
    "<Comp v-slot:named={({ foo })}>{{ foo }}{{ foo: foo }}</Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);

  assert!(code.contains("named: (_slotProps0) =>"));
  assert!(code.contains("{ foo: _slotProps0.foo }"));
}

#[test]
fn on_component_dynamically_named_slot() {
  let code = transform("<Comp v-slot:$named$={{ foo }}>{ foo + bar }</Comp>", None).code;
  assert_snapshot!(code);

  assert!(code.contains("fn: (_slotProps0) =>"));
  assert!(code.contains("_slotProps0.foo + bar"));
}

#[test]
fn named_slots_with_implicit_default_slot() {
  let code = transform(
    "<Comp>
      <template v-slot:one>foo</template>bar<span/>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn named_slots_with_comment() {
  let code = transform(
    "<Comp>
      {/* foo */}
      <template v-slot:one>foo</template>foo<span/>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn nested_slots_scoping() {
  let code = transform(
    "<Comp>
      <template v-slot:default={{ foo }}>
        <Inner v-slot={{ bar }}>
          { foo + bar + baz }
        </Inner>
        { foo + bar + baz }
      </template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_slots_name() {
  let code = transform(
    "<Comp>
      <template v-slot:$name$>{foo}</template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_slots_name_with_v_for() {
  let code = transform(
    "<Comp>
      <template v-for={item in list} v-slot:$item$={{ bar }}>{ bar }</template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn dynamic_slots_name_with_v_if_and_v_else_if() {
  let code = transform(
    "<Comp>
      <template v-if={condition} v-slot:condition>condition slot</template>
      <template v-else-if={anotherCondition} v-slot:condition={{ foo, bar }}>another condition</template>
      <template v-else-if={otherCondition} v-slot:condition>other condition</template>
      <template v-else v-slot:condition>else condition</template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn quote_slot_name() {
  let code = transform(
    "<Comp>
      <template v-slot:nav-bar-title-before></template>
    </Comp>",
    None,
  )
  .code;
  assert_snapshot!(code);
}

#[test]
fn nested_component_slot() {
  let code = transform("<A><B/></A>", None).code;
  assert_snapshot!(code);
}

#[test]
fn error_on_extraneous_children_with_named_default_slot() {
  let error = RefCell::new(None);
  transform(
    "<Comp>
      <template v-slot:default>foo</template>bar
    </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(
    *error.borrow(),
    Some(ErrorCodes::VSlotExtraneousDefaultSlotChildren)
  );
}

#[test]
fn error_on_duplicated_slot_names() {
  let error = RefCell::new(None);
  transform(
    "<Comp>
      <template v-slot:foo></template>
      <template v-slot:foo></template>
    </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotDuplicateSlotNames));
}

#[test]
fn error_on_invalid_mixed_slot_usage() {
  let error = RefCell::new(None);
  transform(
    "<Comp v-slot={foo}>
      <template v-slot:foo></template>
    </Comp>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMixedSlotUsage));
}

#[test]
fn error_on_v_slot_usage_on_plain_elements() {
  let error = RefCell::new(None);
  transform(
    "<div v-slot/>",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VSlotMisplaced));
}
