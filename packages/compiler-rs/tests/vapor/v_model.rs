use std::cell::RefCell;

use common::{error::ErrorCodes, options::TransformOptions};
use compiler_rs::transform;
use insta::assert_snapshot;

#[test]
fn basic() {
  let code = transform("<input v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn modifiers_number() {
  let code = transform("<input v-model_number={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => model, (_value) => model = _value, { number: true });
  	return _n0;
  })();
  "#);
}

#[test]
fn modifiers_trim() {
  let code = transform("<input v-model_trim={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => model, (_value) => model = _value, { trim: true });
  	return _n0;
  })();
  "#);
}

#[test]
fn modifiers_lazy() {
  let code = transform("<input v-model_lazy={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => model, (_value) => model = _value, { lazy: true });
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_input_text() {
  let code = transform("<input type=\"text\" v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input type=text>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_input_radio() {
  let code = transform("<input type=\"radio\" v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyRadioModel as _applyRadioModel, template as _template } from "vue";
  const _t0 = _template("<input type=radio>", true);
  (() => {
  	const _n0 = _t0();
  	_applyRadioModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_input_checkbox() {
  let code = transform("<input type=\"checkbox\" v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyCheckboxModel as _applyCheckboxModel, template as _template } from "vue";
  const _t0 = _template("<input type=checkbox>", true);
  (() => {
  	const _n0 = _t0();
  	_applyCheckboxModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_select() {
  let code = transform("<select v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applySelectModel as _applySelectModel, template as _template } from "vue";
  const _t0 = _template("<select>", true);
  (() => {
  	const _n0 = _t0();
  	_applySelectModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_textarea() {
  let code = transform("<textarea v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<textarea>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => model, (_value) => model = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_input_dynamic_type() {
  let code = transform("<input type={foo} v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyDynamicModel as _applyDynamicModel, renderEffect as _renderEffect, setProp as _setProp, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyDynamicModel(_n0, () => model, (_value) => model = _value);
  	_renderEffect(() => _setProp(_n0, "type", foo));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_dynamic_props() {
  let code = transform("<input {...obj} v-model={model} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyDynamicModel as _applyDynamicModel, renderEffect as _renderEffect, setDynamicProps as _setDynamicProps, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyDynamicModel(_n0, () => model, (_value) => model = _value);
  	_renderEffect(() => _setDynamicProps(_n0, [obj]));
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_member_expression() {
  let code = transform("<input v-model={setupRef.child} />", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input>", true);
  (() => {
  	const _n0 = _t0();
  	_applyTextModel(_n0, () => setupRef.child, (_value) => setupRef.child = _value);
  	return _n0;
  })();
  "#);
}

#[test]
fn should_support_member_expression_with_inline() {
  let code = transform("<><input v-model={setupRef.child} /><input v-model={setupLet.child} /><input v-model={setupMaybeRef.child} /></>", None).code;
  assert_snapshot!(code, @r#"
  import { applyTextModel as _applyTextModel, template as _template } from "vue";
  const _t0 = _template("<input>");
  (() => {
  	const _n0 = _t0();
  	const _n1 = _t0();
  	const _n2 = _t0();
  	_applyTextModel(_n0, () => setupRef.child, (_value) => setupRef.child = _value);
  	_applyTextModel(_n1, () => setupLet.child, (_value) => setupLet.child = _value);
  	_applyTextModel(_n2, () => setupMaybeRef.child, (_value) => setupMaybeRef.child = _value);
  	return [
  		_n0,
  		_n1,
  		_n2
  	];
  })();
  "#);
}

#[test]
fn errors_invalid_element() {
  let error = RefCell::new(None);
  transform(
    "<span v-model={model} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnInvalidElement));
}

#[test]
fn errors_plain_elements_with_argument() {
  let error = RefCell::new(None);
  transform(
    "<input v-model:value={model} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelArgOnElement));
}

#[test]
fn errors_allow_usage_on_custom_element() {
  let error = RefCell::new(None);
  transform(
    "<my-input v-model={model} />",
    Some(TransformOptions {
      is_custom_element: Box::new(|tag| tag.starts_with("my-")),
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), None);
}

#[test]
fn errors_if_used_file_input_element() {
  let error = RefCell::new(None);
  transform(
    "<input type=\"file\" v-model={test} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelOnFileInputElement));
}

#[test]
fn errors_on_dynamic_value_binding_alongside_v_model() {
  let error = RefCell::new(None);
  transform(
    "<input v-model={test} value={test} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelUnnecessaryValue));
}

#[test]
fn errors_should_not_error_on_static_value_binding_alongside_v_model() {
  let error = RefCell::new(None);
  transform(
    "<input v-model={test} value=\"test\" />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), None);
}

#[test]
fn errors_empty_expression() {
  let error = RefCell::new(None);
  transform(
    "<span v-model=\"\" />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelMalformedExpression));
}

#[test]
fn errors_mal_formed_expression() {
  let error = RefCell::new(None);
  transform(
    "<span v-model={a + b} />",
    Some(TransformOptions {
      on_error: Box::new(|e, _| {
        *error.borrow_mut() = Some(e);
      }),
      ..Default::default()
    }),
  );
  assert_eq!(*error.borrow(), Some(ErrorCodes::VModelMalformedExpression));
}

#[test]
fn component() {
  let code = transform("<Comp v-model={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, {
  		modelValue: () => foo,
  		"onUpdate:modelValue": () => (_value) => foo = _value
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_with_arguments() {
  let code = transform("<Comp v-model:bar={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, {
  		bar: () => foo,
  		"onUpdate:bar": () => (_value) => foo = _value
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_with_dynamic_arguments() {
  let code = transform("<Comp v-model:$arg$={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, { $: [() => ({
  		[arg]: foo,
  		["onUpdate:" + arg]: () => (_value) => foo = _value
  	})] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_with_dynamic_arguments_with_v_for() {
  let code = transform("<Comp v-for={{arg} in list} v-model:$arg$={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  import { createFor as _createFor } from "vue";
  (() => {
  	const _n0 = _createFor(() => list, (_for_item0) => {
  		const _n2 = _createComponent(Comp, { $: [() => ({
  			[_for_item0.value.arg]: foo,
  			["onUpdate:" + _for_item0.value.arg]: () => (_value) => foo = _value
  		})] });
  		return _n2;
  	}, void 0, 2);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_should_generate_model_value_modifiers() {
  let code = transform("<Comp v-model_trim_bar-baz={foo} />", None).code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, {
  		modelValue: () => foo,
  		"onUpdate:modelValue": () => (_value) => foo = _value,
  		modelValueModifiers: () => ({
  			trim: true,
  			"bar-baz": true
  		})
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_with_arguments_should_generate_model_modifiers() {
  let code = transform(
    "<Comp v-model:foo_trim={foo} v-model:foo-bar_number={bar} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, {
  		foo: () => foo,
  		"onUpdate:foo": () => (_value) => foo = _value,
  		fooModifiers: () => ({ trim: true }),
  		"onUpdate:foo-bar": () => (_value) => bar = _value,
  		"foo-bar": () => bar,
  		"foo-barModifiers": () => ({ number: true })
  	}, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_with_dynamic_arguments_should_generate_model_modifiers() {
  let code = transform(
    "<Comp v-model:$foo$_trim={foo} v-model:$bar_value$_number={bar} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, { $: [() => ({
  		[foo]: foo,
  		["onUpdate:" + foo]: () => (_value) => foo = _value,
  		[foo + "Modifiers"]: () => ({ trim: true })
  	}), () => ({
  		[bar.value]: bar,
  		["onUpdate:" + bar.value]: () => (_value) => bar = _value,
  		[bar.value + "Modifiers"]: () => ({ number: true })
  	})] }, null, true);
  	return _n0;
  })();
  "#);
}

#[test]
fn component_v_model_should_merge_with_explicit() {
  let code = transform(
    "<Comp v-model={counter} onUpdate:modelValue={onUpdate} />",
    None,
  )
  .code;
  assert_snapshot!(code, @r#"
  import { createComponent as _createComponent } from "/vue-jsx-vapor/vapor";
  (() => {
  	const _n0 = _createComponent(Comp, {
  		modelValue: () => counter,
  		"onUpdate:modelValue": () => [(_value) => counter = _value, onUpdate]
  	}, null, true);
  	return _n0;
  })();
  "#);
}
