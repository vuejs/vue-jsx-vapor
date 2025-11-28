use indexmap::IndexMap;
use napi::{
  Either,
  bindgen_prelude::{Either3, Either4},
};

use crate::ir::index::{BlockIRNode, IRFor, Modifiers, SimpleExpressionNode};

#[derive(Clone, Debug)]
pub struct IRProp<'a> {
  pub key: SimpleExpressionNode<'a>,
  pub modifier: Option<String>,
  pub runtime_camelize: Option<bool>,
  pub handler: Option<bool>,
  pub handler_modifiers: Option<Modifiers>,
  pub model: Option<bool>,
  pub model_modifiers: Option<Vec<String>>,

  pub values: Vec<SimpleExpressionNode<'a>>,
  pub dynamic: bool,
}

pub type IRPropsStatic<'a> = Vec<IRProp<'a>>;

#[derive(Clone, Debug)]
pub struct IRPropsDynamicExpression<'a> {
  pub value: SimpleExpressionNode<'a>,
  pub handler: Option<bool>,
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
  pub slots: IndexMap<String, BlockIRNode<'a>>,
}

#[derive(Debug)]
pub struct IRSlotDynamicBasic<'a> {
  pub slot_type: IRSlotType,
  pub name: SimpleExpressionNode<'a>,
  pub _fn: BlockIRNode<'a>,
  pub _loop: Option<IRFor<'a>>,
}

#[derive(Debug)]
pub struct IRSlotDynamicConditional<'a> {
  pub slot_type: IRSlotType,
  pub condition: SimpleExpressionNode<'a>,
  pub positive: IRSlotDynamicBasic<'a>,
  pub negative: Option<Box<Either<IRSlotDynamicBasic<'a>, IRSlotDynamicConditional<'a>>>>,
}

#[derive(Debug)]
pub struct IRSlotsExpression<'a> {
  pub slot_type: IRSlotType,
  pub slots: SimpleExpressionNode<'a>,
}

pub type IRSlots<'a> = Either4<
  IRSlotsStatic<'a>,
  IRSlotDynamicBasic<'a>,
  IRSlotDynamicConditional<'a>,
  IRSlotsExpression<'a>,
>;
