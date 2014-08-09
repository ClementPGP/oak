// Copyright 2014 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use rust::{ExtCtxt, Span};
use attribute::model::*;

pub enum RuleTypeStyle
{
  New,
  Inline(Span),
  Invisible(Span)
}

impl RuleTypeStyle
{
  pub fn new(cx: &ExtCtxt, model: &AttributeDict) -> RuleTypeStyle
  {
    let inline_type = model.plain_value("inline_type");
    let invisible_type = model.plain_value("invisible_type");
    let inline = inline_type.value_or(false);
    let invisible = invisible_type.value_or(false);
    if inline && invisible {
      cx.span_err(inline_type.span(),
        "Incoherent rule type specifiers, a rule can't be inlined and invisible.");
      cx.span_note(invisible_type.span(),
        "Previous declaration here.");
      New
    } else if inline {
      Inline(inline_type.span())
    } else if invisible {
      Invisible(invisible_type.span())
    } else {
      New
    }
  }

  pub fn register(model: &mut AttributeDict)
  {
    model.push_all(vec![
      AttributeInfo::simple(
        "inline_type",
        "the type of the rule will be merged with the type of the calling site. No rule type will be created.",
      ),
      AttributeInfo::simple(
        "invisible_type",
        "the calling site will ignore the type of this rule. The AST of the calling rule will not reference this rule.",
      )
    ]);
  }
}

pub struct RuleType
{
  pub type_style: RuleTypeStyle
  // name: Option<RuleTypeNameAttribute>,
  // auto_gen_name: AutoGenName
}

impl RuleType
{
  pub fn new(cx: &ExtCtxt, model: &AttributeDict) -> RuleType
  {
    RuleType {
      type_style: RuleTypeStyle::new(cx, model)
    }
  }

  pub fn register(model: &mut AttributeDict)
  {
    RuleTypeStyle::register(model);
  }
}

// enum AutoGenName
// {
//   NoTransformation,
//   CamelCased
// }

// struct RuleTypeNameAttribute
// {
//   type_name: Option<String>,
//   fields_names: Vec<String>
// }