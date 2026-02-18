use common::directive::Modifiers;
use indexmap::IndexMap;
use napi::{
  Either,
  bindgen_prelude::{Either3, Either4},
};
use oxc_ast::ast::Expression;
use oxc_span::Atom;

use crate::ir::index::{BlockIRNode, IRFor};

#[derive(Debug)]
pub struct IRProp<'a> {
  pub key: Expression<'a>,
  pub modifier: Option<&'a str>,
  pub runtime_camelize: bool,
  pub handler: bool,
  pub handler_modifiers: Option<Modifiers<'a>>,
  pub model: bool,
  pub model_modifiers: Option<Vec<String>>,

  pub values: Vec<Expression<'a>>,
  pub dynamic: bool,
}

pub type IRPropsStatic<'a> = Vec<IRProp<'a>>;

#[derive(Debug)]
pub struct IRPropsDynamicExpression<'a> {
  pub value: Expression<'a>,
  pub handler: bool,
}

pub type IRProps<'a> = Either3<IRPropsStatic<'a>, IRProp<'a>, IRPropsDynamicExpression<'a>>;

// slots
#[derive(Debug)]
pub enum IRSlotType {
  STATIC,
  DYNAMIC,
  CONDITIONAL,
  EXPRESSION,
}

#[derive(Debug)]
pub struct IRSlotsStatic<'a> {
  pub slot_type: IRSlotType,
  pub slots: IndexMap<Atom<'a>, BlockIRNode<'a>>,
}

#[derive(Debug)]
pub struct IRSlotDynamicBasic<'a> {
  pub slot_type: IRSlotType,
  pub name: Expression<'a>,
  pub _fn: BlockIRNode<'a>,
  pub _loop: Option<IRFor<'a>>,
}

#[derive(Debug)]
pub struct IRSlotDynamicConditional<'a> {
  pub slot_type: IRSlotType,
  pub condition: Expression<'a>,
  pub positive: IRSlotDynamicBasic<'a>,
  pub negative: Option<Box<Either<IRSlotDynamicBasic<'a>, IRSlotDynamicConditional<'a>>>>,
}

#[derive(Debug)]
pub struct IRSlotsExpression<'a> {
  pub slot_type: IRSlotType,
  pub slots: Expression<'a>,
}

pub type IRSlots<'a> = Either4<
  IRSlotsStatic<'a>,
  IRSlotDynamicBasic<'a>,
  IRSlotDynamicConditional<'a>,
  IRSlotsExpression<'a>,
>;
