// This file is automatically generated by the generate-visitor.php script.
// Do not modify this file directly.
#![allow(unused)]

use super::*;
use crate::*;
use pxp_span::Span;
use pxp_type::Type;

pub trait VisitorMut {
    fn visit(&mut self, node: &mut [Statement]) {
        walk_mut(self, node);
    }

    fn visit_statement(&mut self, node: &mut Statement) {
        walk_statement_mut(self, node);
    }

    fn visit_statement_kind(&mut self, node: &mut StatementKind) {
        walk_statement_kind_mut(self, node);
    }

    fn visit_expression(&mut self, node: &mut Expression) {
        walk_expression_mut(self, node);
    }

    fn visit_expression_kind(&mut self, node: &mut ExpressionKind) {
        walk_expression_kind_mut(self, node);
    }

    fn visit_missing_expression(&mut self, node: &mut MissingExpression) {}

    fn visit_static_expression(&mut self, node: &mut StaticExpression) {}

    fn visit_self_expression(&mut self, node: &mut SelfExpression) {}

    fn visit_parent_expression(&mut self, node: &mut ParentExpression) {}

    fn visit_comment_statement(&mut self, node: &mut CommentStatement) {}

    fn visit_inline_html_statement(&mut self, node: &mut InlineHtmlStatement) {}

    fn visit_full_opening_tag_statement(&mut self, node: &mut FullOpeningTagStatement) {}

    fn visit_short_opening_tag_statement(&mut self, node: &mut ShortOpeningTagStatement) {}

    fn visit_echo_opening_tag_statement(&mut self, node: &mut EchoOpeningTagStatement) {}

    fn visit_closing_tag_statement(&mut self, node: &mut ClosingTagStatement) {}

    fn visit_expression_statement(&mut self, node: &mut ExpressionStatement) {
        walk_expression_statement_mut(self, node);
    }

    fn visit_global_statement(&mut self, node: &mut GlobalStatement) {
        walk_global_statement_mut(self, node);
    }

    fn visit_block_statement(&mut self, node: &mut BlockStatement) {
        walk_block_statement_mut(self, node);
    }

    fn visit_cast_kind(&mut self, node: &mut CastKind) {}

    fn visit_case(&mut self, node: &mut Case) {
        walk_case_mut(self, node);
    }

    fn visit_use(&mut self, node: &mut Use) {
        walk_use_mut(self, node);
    }

    fn visit_use_kind(&mut self, node: &mut UseKind) {
        walk_use_kind_mut(self, node);
    }

    fn visit_eval_expression(&mut self, node: &mut EvalExpression) {
        walk_eval_expression_mut(self, node);
    }

    fn visit_empty_expression(&mut self, node: &mut EmptyExpression) {
        walk_empty_expression_mut(self, node);
    }

    fn visit_die_expression(&mut self, node: &mut DieExpression) {
        walk_die_expression_mut(self, node);
    }

    fn visit_exit_expression(&mut self, node: &mut ExitExpression) {
        walk_exit_expression_mut(self, node);
    }

    fn visit_isset_expression(&mut self, node: &mut IssetExpression) {
        walk_isset_expression_mut(self, node);
    }

    fn visit_unset_expression(&mut self, node: &mut UnsetExpression) {
        walk_unset_expression_mut(self, node);
    }

    fn visit_print_expression(&mut self, node: &mut PrintExpression) {
        walk_print_expression_mut(self, node);
    }

    fn visit_concat_expression(&mut self, node: &mut ConcatExpression) {
        walk_concat_expression_mut(self, node);
    }

    fn visit_instanceof_expression(&mut self, node: &mut InstanceofExpression) {
        walk_instanceof_expression_mut(self, node);
    }

    fn visit_reference_expression(&mut self, node: &mut ReferenceExpression) {
        walk_reference_expression_mut(self, node);
    }

    fn visit_parenthesized_expression(&mut self, node: &mut ParenthesizedExpression) {
        walk_parenthesized_expression_mut(self, node);
    }

    fn visit_error_suppress_expression(&mut self, node: &mut ErrorSuppressExpression) {
        walk_error_suppress_expression_mut(self, node);
    }

    fn visit_include_expression(&mut self, node: &mut IncludeExpression) {
        walk_include_expression_mut(self, node);
    }

    fn visit_include_once_expression(&mut self, node: &mut IncludeOnceExpression) {
        walk_include_once_expression_mut(self, node);
    }

    fn visit_require_expression(&mut self, node: &mut RequireExpression) {
        walk_require_expression_mut(self, node);
    }

    fn visit_require_once_expression(&mut self, node: &mut RequireOnceExpression) {
        walk_require_once_expression_mut(self, node);
    }

    fn visit_function_call_expression(&mut self, node: &mut FunctionCallExpression) {
        walk_function_call_expression_mut(self, node);
    }

    fn visit_function_closure_creation_expression(
        &mut self,
        node: &mut FunctionClosureCreationExpression,
    ) {
        walk_function_closure_creation_expression_mut(self, node);
    }

    fn visit_method_call_expression(&mut self, node: &mut MethodCallExpression) {
        walk_method_call_expression_mut(self, node);
    }

    fn visit_method_closure_creation_expression(
        &mut self,
        node: &mut MethodClosureCreationExpression,
    ) {
        walk_method_closure_creation_expression_mut(self, node);
    }

    fn visit_nullsafe_method_call_expression(&mut self, node: &mut NullsafeMethodCallExpression) {
        walk_nullsafe_method_call_expression_mut(self, node);
    }

    fn visit_static_method_call_expression(&mut self, node: &mut StaticMethodCallExpression) {
        walk_static_method_call_expression_mut(self, node);
    }

    fn visit_static_variable_method_call_expression(
        &mut self,
        node: &mut StaticVariableMethodCallExpression,
    ) {
        walk_static_variable_method_call_expression_mut(self, node);
    }

    fn visit_static_method_closure_creation_expression(
        &mut self,
        node: &mut StaticMethodClosureCreationExpression,
    ) {
        walk_static_method_closure_creation_expression_mut(self, node);
    }

    fn visit_static_variable_method_closure_creation_expression(
        &mut self,
        node: &mut StaticVariableMethodClosureCreationExpression,
    ) {
        walk_static_variable_method_closure_creation_expression_mut(self, node);
    }

    fn visit_property_fetch_expression(&mut self, node: &mut PropertyFetchExpression) {
        walk_property_fetch_expression_mut(self, node);
    }

    fn visit_nullsafe_property_fetch_expression(
        &mut self,
        node: &mut NullsafePropertyFetchExpression,
    ) {
        walk_nullsafe_property_fetch_expression_mut(self, node);
    }

    fn visit_static_property_fetch_expression(&mut self, node: &mut StaticPropertyFetchExpression) {
        walk_static_property_fetch_expression_mut(self, node);
    }

    fn visit_constant_fetch_expression(&mut self, node: &mut ConstantFetchExpression) {
        walk_constant_fetch_expression_mut(self, node);
    }

    fn visit_short_array_expression(&mut self, node: &mut ShortArrayExpression) {
        walk_short_array_expression_mut(self, node);
    }

    fn visit_array_expression(&mut self, node: &mut ArrayExpression) {
        walk_array_expression_mut(self, node);
    }

    fn visit_list_expression(&mut self, node: &mut ListExpression) {
        walk_list_expression_mut(self, node);
    }

    fn visit_new_expression(&mut self, node: &mut NewExpression) {
        walk_new_expression_mut(self, node);
    }

    fn visit_interpolated_string_expression(&mut self, node: &mut InterpolatedStringExpression) {
        walk_interpolated_string_expression_mut(self, node);
    }

    fn visit_heredoc_expression(&mut self, node: &mut HeredocExpression) {
        walk_heredoc_expression_mut(self, node);
    }

    fn visit_nowdoc_expression(&mut self, node: &mut NowdocExpression) {}

    fn visit_shell_exec_expression(&mut self, node: &mut ShellExecExpression) {
        walk_shell_exec_expression_mut(self, node);
    }

    fn visit_bool_expression(&mut self, node: &mut BoolExpression) {}

    fn visit_array_index_expression(&mut self, node: &mut ArrayIndexExpression) {
        walk_array_index_expression_mut(self, node);
    }

    fn visit_short_ternary_expression(&mut self, node: &mut ShortTernaryExpression) {
        walk_short_ternary_expression_mut(self, node);
    }

    fn visit_ternary_expression(&mut self, node: &mut TernaryExpression) {
        walk_ternary_expression_mut(self, node);
    }

    fn visit_coalesce_expression(&mut self, node: &mut CoalesceExpression) {
        walk_coalesce_expression_mut(self, node);
    }

    fn visit_clone_expression(&mut self, node: &mut CloneExpression) {
        walk_clone_expression_mut(self, node);
    }

    fn visit_match_expression(&mut self, node: &mut MatchExpression) {
        walk_match_expression_mut(self, node);
    }

    fn visit_throw_expression(&mut self, node: &mut ThrowExpression) {
        walk_throw_expression_mut(self, node);
    }

    fn visit_yield_expression(&mut self, node: &mut YieldExpression) {
        walk_yield_expression_mut(self, node);
    }

    fn visit_yield_from_expression(&mut self, node: &mut YieldFromExpression) {
        walk_yield_from_expression_mut(self, node);
    }

    fn visit_cast_expression(&mut self, node: &mut CastExpression) {
        walk_cast_expression_mut(self, node);
    }

    fn visit_default_match_arm(&mut self, node: &mut DefaultMatchArm) {
        walk_default_match_arm_mut(self, node);
    }

    fn visit_match_arm(&mut self, node: &mut MatchArm) {
        walk_match_arm_mut(self, node);
    }

    fn visit_magic_constant_expression(&mut self, node: &mut MagicConstantExpression) {
        walk_magic_constant_expression_mut(self, node);
    }

    fn visit_magic_constant_kind(&mut self, node: &mut MagicConstantKind) {
        walk_magic_constant_kind_mut(self, node);
    }

    fn visit_string_part(&mut self, node: &mut StringPart) {
        walk_string_part_mut(self, node);
    }

    fn visit_literal_string_part(&mut self, node: &mut LiteralStringPart) {}

    fn visit_expression_string_part(&mut self, node: &mut ExpressionStringPart) {
        walk_expression_string_part_mut(self, node);
    }

    fn visit_array_item(&mut self, node: &mut ArrayItem) {
        walk_array_item_mut(self, node);
    }

    fn visit_array_item_value(&mut self, node: &mut ArrayItemValue) {
        walk_array_item_value_mut(self, node);
    }

    fn visit_array_item_referenced_value(&mut self, node: &mut ArrayItemReferencedValue) {
        walk_array_item_referenced_value_mut(self, node);
    }

    fn visit_array_item_spread_value(&mut self, node: &mut ArrayItemSpreadValue) {
        walk_array_item_spread_value_mut(self, node);
    }

    fn visit_array_item_key_value(&mut self, node: &mut ArrayItemKeyValue) {
        walk_array_item_key_value_mut(self, node);
    }

    fn visit_array_item_referenced_key_value(&mut self, node: &mut ArrayItemReferencedKeyValue) {
        walk_array_item_referenced_key_value_mut(self, node);
    }

    fn visit_list_entry(&mut self, node: &mut ListEntry) {
        walk_list_entry_mut(self, node);
    }

    fn visit_list_entry_value(&mut self, node: &mut ListEntryValue) {
        walk_list_entry_value_mut(self, node);
    }

    fn visit_list_entry_key_value(&mut self, node: &mut ListEntryKeyValue) {
        walk_list_entry_key_value_mut(self, node);
    }

    fn visit_positional_argument(&mut self, node: &mut PositionalArgument) {
        walk_positional_argument_mut(self, node);
    }

    fn visit_named_argument(&mut self, node: &mut NamedArgument) {
        walk_named_argument_mut(self, node);
    }

    fn visit_argument(&mut self, node: &mut Argument) {
        walk_argument_mut(self, node);
    }

    fn visit_argument_list(&mut self, node: &mut ArgumentList) {
        walk_argument_list_mut(self, node);
    }

    fn visit_single_argument(&mut self, node: &mut SingleArgument) {
        walk_single_argument_mut(self, node);
    }

    fn visit_argument_placeholder(&mut self, node: &mut ArgumentPlaceholder) {}

    fn visit_attribute(&mut self, node: &mut Attribute) {
        walk_attribute_mut(self, node);
    }

    fn visit_attribute_group(&mut self, node: &mut AttributeGroup) {
        walk_attribute_group_mut(self, node);
    }

    fn visit_class_body(&mut self, node: &mut ClassBody) {
        walk_class_body_mut(self, node);
    }

    fn visit_class_statement(&mut self, node: &mut ClassStatement) {
        walk_class_statement_mut(self, node);
    }

    fn visit_anonymous_class_body(&mut self, node: &mut AnonymousClassBody) {
        walk_anonymous_class_body_mut(self, node);
    }

    fn visit_anonymous_class_expression(&mut self, node: &mut AnonymousClassExpression) {
        walk_anonymous_class_expression_mut(self, node);
    }

    fn visit_class_extends(&mut self, node: &mut ClassExtends) {
        walk_class_extends_mut(self, node);
    }

    fn visit_class_implements(&mut self, node: &mut ClassImplements) {
        walk_class_implements_mut(self, node);
    }

    fn visit_classish_member(&mut self, node: &mut ClassishMember) {
        walk_classish_member_mut(self, node);
    }

    fn visit_method(&mut self, node: &mut Method) {
        walk_method_mut(self, node);
    }

    fn visit_method_body(&mut self, node: &mut MethodBody) {
        walk_method_body_mut(self, node);
    }

    fn visit_method_body_kind(&mut self, node: &mut MethodBodyKind) {
        walk_method_body_kind_mut(self, node);
    }

    fn visit_missing_method_body(&mut self, node: &mut MissingMethodBody) {}

    fn visit_abstract_method_body(&mut self, node: &mut AbstractMethodBody) {}

    fn visit_concrete_method_body(&mut self, node: &mut ConcreteMethodBody) {
        walk_concrete_method_body_mut(self, node);
    }

    fn visit_method_parameter_list(&mut self, node: &mut MethodParameterList) {
        walk_method_parameter_list_mut(self, node);
    }

    fn visit_method_parameter(&mut self, node: &mut MethodParameter) {
        walk_method_parameter_mut(self, node);
    }

    fn visit_missing_classish_member(&mut self, node: &mut MissingClassishMember) {}

    fn visit_constant_entry(&mut self, node: &mut ConstantEntry) {
        walk_constant_entry_mut(self, node);
    }

    fn visit_classish_constant_entry(&mut self, node: &mut ClassishConstantEntry) {
        walk_classish_constant_entry_mut(self, node);
    }

    fn visit_constant_statement(&mut self, node: &mut ConstantStatement) {
        walk_constant_statement_mut(self, node);
    }

    fn visit_classish_constant(&mut self, node: &mut ClassishConstant) {
        walk_classish_constant_mut(self, node);
    }

    fn visit_if_statement(&mut self, node: &mut IfStatement) {
        walk_if_statement_mut(self, node);
    }

    fn visit_if_statement_body(&mut self, node: &mut IfStatementBody) {
        walk_if_statement_body_mut(self, node);
    }

    fn visit_if_statement_body_statement(&mut self, node: &mut IfStatementBodyStatement) {
        walk_if_statement_body_statement_mut(self, node);
    }

    fn visit_if_statement_body_block(&mut self, node: &mut IfStatementBodyBlock) {
        walk_if_statement_body_block_mut(self, node);
    }

    fn visit_if_statement_else_if(&mut self, node: &mut IfStatementElseIf) {
        walk_if_statement_else_if_mut(self, node);
    }

    fn visit_if_statement_else(&mut self, node: &mut IfStatementElse) {
        walk_if_statement_else_mut(self, node);
    }

    fn visit_if_statement_else_if_block(&mut self, node: &mut IfStatementElseIfBlock) {
        walk_if_statement_else_if_block_mut(self, node);
    }

    fn visit_if_statement_else_block(&mut self, node: &mut IfStatementElseBlock) {
        walk_if_statement_else_block_mut(self, node);
    }

    fn visit_data_type(&mut self, node: &mut DataType) {
        walk_data_type_mut(self, node);
    }

    fn visit_declare_entry(&mut self, node: &mut DeclareEntry) {
        walk_declare_entry_mut(self, node);
    }

    fn visit_declare_entry_group(&mut self, node: &mut DeclareEntryGroup) {
        walk_declare_entry_group_mut(self, node);
    }

    fn visit_declare_body(&mut self, node: &mut DeclareBody) {
        walk_declare_body_mut(self, node);
    }

    fn visit_declare_body_noop(&mut self, node: &mut DeclareBodyNoop) {}

    fn visit_declare_body_braced(&mut self, node: &mut DeclareBodyBraced) {
        walk_declare_body_braced_mut(self, node);
    }

    fn visit_declare_body_expression(&mut self, node: &mut DeclareBodyExpression) {
        walk_declare_body_expression_mut(self, node);
    }

    fn visit_declare_body_block(&mut self, node: &mut DeclareBodyBlock) {
        walk_declare_body_block_mut(self, node);
    }

    fn visit_declare_statement(&mut self, node: &mut DeclareStatement) {
        walk_declare_statement_mut(self, node);
    }

    fn visit_unit_enum_case(&mut self, node: &mut UnitEnumCase) {
        walk_unit_enum_case_mut(self, node);
    }

    fn visit_unit_enum_member(&mut self, node: &mut UnitEnumMember) {
        walk_unit_enum_member_mut(self, node);
    }

    fn visit_unit_enum_body(&mut self, node: &mut UnitEnumBody) {
        walk_unit_enum_body_mut(self, node);
    }

    fn visit_unit_enum_statement(&mut self, node: &mut UnitEnumStatement) {
        walk_unit_enum_statement_mut(self, node);
    }

    fn visit_backed_enum_case(&mut self, node: &mut BackedEnumCase) {
        walk_backed_enum_case_mut(self, node);
    }

    fn visit_backed_enum_member(&mut self, node: &mut BackedEnumMember) {
        walk_backed_enum_member_mut(self, node);
    }

    fn visit_backed_enum_body(&mut self, node: &mut BackedEnumBody) {
        walk_backed_enum_body_mut(self, node);
    }

    fn visit_backed_enum_statement(&mut self, node: &mut BackedEnumStatement) {
        walk_backed_enum_statement_mut(self, node);
    }

    fn visit_backed_enum_type(&mut self, node: &mut BackedEnumType) {
        walk_backed_enum_type_mut(self, node);
    }

    fn visit_return_type(&mut self, node: &mut ReturnType) {
        walk_return_type_mut(self, node);
    }

    fn visit_function_parameter(&mut self, node: &mut FunctionParameter) {
        walk_function_parameter_mut(self, node);
    }

    fn visit_function_parameter_list(&mut self, node: &mut FunctionParameterList) {
        walk_function_parameter_list_mut(self, node);
    }

    fn visit_function_body(&mut self, node: &mut FunctionBody) {
        walk_function_body_mut(self, node);
    }

    fn visit_function_statement(&mut self, node: &mut FunctionStatement) {
        walk_function_statement_mut(self, node);
    }

    fn visit_closure_use_variable(&mut self, node: &mut ClosureUseVariable) {
        walk_closure_use_variable_mut(self, node);
    }

    fn visit_closure_use(&mut self, node: &mut ClosureUse) {
        walk_closure_use_mut(self, node);
    }

    fn visit_closure_expression(&mut self, node: &mut ClosureExpression) {
        walk_closure_expression_mut(self, node);
    }

    fn visit_arrow_function_expression(&mut self, node: &mut ArrowFunctionExpression) {
        walk_arrow_function_expression_mut(self, node);
    }

    fn visit_label_statement(&mut self, node: &mut LabelStatement) {
        walk_label_statement_mut(self, node);
    }

    fn visit_goto_statement(&mut self, node: &mut GotoStatement) {
        walk_goto_statement_mut(self, node);
    }

    fn visit_identifier(&mut self, node: &mut Identifier) {
        walk_identifier_mut(self, node);
    }

    fn visit_simple_identifier(&mut self, node: &mut SimpleIdentifier) {}

    fn visit_dynamic_identifier(&mut self, node: &mut DynamicIdentifier) {
        walk_dynamic_identifier_mut(self, node);
    }

    fn visit_interface_extends(&mut self, node: &mut InterfaceExtends) {
        walk_interface_extends_mut(self, node);
    }

    fn visit_interface_body(&mut self, node: &mut InterfaceBody) {
        walk_interface_body_mut(self, node);
    }

    fn visit_interface_statement(&mut self, node: &mut InterfaceStatement) {
        walk_interface_statement_mut(self, node);
    }

    fn visit_literal(&mut self, node: &mut Literal) {
        walk_literal_mut(self, node);
    }

    fn visit_literal_kind(&mut self, node: &mut LiteralKind) {
        walk_literal_kind_mut(self, node);
    }

    fn visit_foreach_statement(&mut self, node: &mut ForeachStatement) {
        walk_foreach_statement_mut(self, node);
    }

    fn visit_foreach_statement_iterator(&mut self, node: &mut ForeachStatementIterator) {
        walk_foreach_statement_iterator_mut(self, node);
    }

    fn visit_foreach_statement_iterator_value(&mut self, node: &mut ForeachStatementIteratorValue) {
        walk_foreach_statement_iterator_value_mut(self, node);
    }

    fn visit_foreach_statement_iterator_key_and_value(
        &mut self,
        node: &mut ForeachStatementIteratorKeyAndValue,
    ) {
        walk_foreach_statement_iterator_key_and_value_mut(self, node);
    }

    fn visit_foreach_statement_body(&mut self, node: &mut ForeachStatementBody) {
        walk_foreach_statement_body_mut(self, node);
    }

    fn visit_foreach_statement_body_statement(&mut self, node: &mut ForeachStatementBodyStatement) {
        walk_foreach_statement_body_statement_mut(self, node);
    }

    fn visit_foreach_statement_body_block(&mut self, node: &mut ForeachStatementBodyBlock) {
        walk_foreach_statement_body_block_mut(self, node);
    }

    fn visit_for_statement(&mut self, node: &mut ForStatement) {
        walk_for_statement_mut(self, node);
    }

    fn visit_for_statement_iterator(&mut self, node: &mut ForStatementIterator) {
        walk_for_statement_iterator_mut(self, node);
    }

    fn visit_for_statement_body(&mut self, node: &mut ForStatementBody) {
        walk_for_statement_body_mut(self, node);
    }

    fn visit_for_statement_body_statement(&mut self, node: &mut ForStatementBodyStatement) {
        walk_for_statement_body_statement_mut(self, node);
    }

    fn visit_for_statement_body_block(&mut self, node: &mut ForStatementBodyBlock) {
        walk_for_statement_body_block_mut(self, node);
    }

    fn visit_do_while_statement(&mut self, node: &mut DoWhileStatement) {
        walk_do_while_statement_mut(self, node);
    }

    fn visit_while_statement(&mut self, node: &mut WhileStatement) {
        walk_while_statement_mut(self, node);
    }

    fn visit_while_statement_body(&mut self, node: &mut WhileStatementBody) {
        walk_while_statement_body_mut(self, node);
    }

    fn visit_while_statement_body_statement(&mut self, node: &mut WhileStatementBodyStatement) {
        walk_while_statement_body_statement_mut(self, node);
    }

    fn visit_while_statement_body_block(&mut self, node: &mut WhileStatementBodyBlock) {
        walk_while_statement_body_block_mut(self, node);
    }

    fn visit_level(&mut self, node: &mut Level) {
        walk_level_mut(self, node);
    }

    fn visit_literal_level(&mut self, node: &mut LiteralLevel) {
        walk_literal_level_mut(self, node);
    }

    fn visit_parenthesized_level(&mut self, node: &mut ParenthesizedLevel) {}

    fn visit_break_statement(&mut self, node: &mut BreakStatement) {
        walk_break_statement_mut(self, node);
    }

    fn visit_continue_statement(&mut self, node: &mut ContinueStatement) {
        walk_continue_statement_mut(self, node);
    }

    fn visit_visibility_modifier(&mut self, node: &mut VisibilityModifier) {}

    fn visit_promoted_property_modifier(&mut self, node: &mut PromotedPropertyModifier) {}

    fn visit_promoted_property_modifier_group(&mut self, node: &mut PromotedPropertyModifierGroup) {
        walk_promoted_property_modifier_group_mut(self, node);
    }

    fn visit_property_modifier(&mut self, node: &mut PropertyModifier) {}

    fn visit_property_modifier_group(&mut self, node: &mut PropertyModifierGroup) {
        walk_property_modifier_group_mut(self, node);
    }

    fn visit_method_modifier(&mut self, node: &mut MethodModifier) {}

    fn visit_method_modifier_group(&mut self, node: &mut MethodModifierGroup) {
        walk_method_modifier_group_mut(self, node);
    }

    fn visit_class_modifier(&mut self, node: &mut ClassModifier) {}

    fn visit_class_modifier_group(&mut self, node: &mut ClassModifierGroup) {
        walk_class_modifier_group_mut(self, node);
    }

    fn visit_constant_modifier(&mut self, node: &mut ConstantModifier) {}

    fn visit_constant_modifier_group(&mut self, node: &mut ConstantModifierGroup) {
        walk_constant_modifier_group_mut(self, node);
    }

    fn visit_unbraced_namespace(&mut self, node: &mut UnbracedNamespace) {
        walk_unbraced_namespace_mut(self, node);
    }

    fn visit_braced_namespace(&mut self, node: &mut BracedNamespace) {
        walk_braced_namespace_mut(self, node);
    }

    fn visit_braced_namespace_body(&mut self, node: &mut BracedNamespaceBody) {
        walk_braced_namespace_body_mut(self, node);
    }

    fn visit_namespace_statement(&mut self, node: &mut NamespaceStatement) {
        walk_namespace_statement_mut(self, node);
    }

    fn visit_arithmetic_operation_expression(&mut self, node: &mut ArithmeticOperationExpression) {
        walk_arithmetic_operation_expression_mut(self, node);
    }

    fn visit_arithmetic_operation_kind(&mut self, node: &mut ArithmeticOperationKind) {
        walk_arithmetic_operation_kind_mut(self, node);
    }

    fn visit_assignment_operation_expression(&mut self, node: &mut AssignmentOperationExpression) {
        walk_assignment_operation_expression_mut(self, node);
    }

    fn visit_assignment_operation_kind(&mut self, node: &mut AssignmentOperationKind) {
        walk_assignment_operation_kind_mut(self, node);
    }

    fn visit_bitwise_operation_expression(&mut self, node: &mut BitwiseOperationExpression) {
        walk_bitwise_operation_expression_mut(self, node);
    }

    fn visit_bitwise_operation_kind(&mut self, node: &mut BitwiseOperationKind) {
        walk_bitwise_operation_kind_mut(self, node);
    }

    fn visit_comparison_operation_expression(&mut self, node: &mut ComparisonOperationExpression) {
        walk_comparison_operation_expression_mut(self, node);
    }

    fn visit_comparison_operation_kind(&mut self, node: &mut ComparisonOperationKind) {
        walk_comparison_operation_kind_mut(self, node);
    }

    fn visit_logical_operation_expression(&mut self, node: &mut LogicalOperationExpression) {
        walk_logical_operation_expression_mut(self, node);
    }

    fn visit_logical_operation_kind(&mut self, node: &mut LogicalOperationKind) {
        walk_logical_operation_kind_mut(self, node);
    }

    fn visit_name(&mut self, node: &mut Name) {
        walk_name_mut(self, node);
    }

    fn visit_name_kind(&mut self, node: &mut NameKind) {
        walk_name_kind_mut(self, node);
    }

    fn visit_special_name(&mut self, node: &mut SpecialName) {
        walk_special_name_mut(self, node);
    }

    fn visit_special_name_kind(&mut self, node: &mut SpecialNameKind) {}

    fn visit_unresolved_name(&mut self, node: &mut UnresolvedName) {}

    fn visit_resolved_name(&mut self, node: &mut ResolvedName) {}

    fn visit_property(&mut self, node: &mut Property) {
        walk_property_mut(self, node);
    }

    fn visit_simple_property(&mut self, node: &mut SimpleProperty) {
        walk_simple_property_mut(self, node);
    }

    fn visit_hooked_property(&mut self, node: &mut HookedProperty) {
        walk_hooked_property_mut(self, node);
    }

    fn visit_property_hook_list(&mut self, node: &mut PropertyHookList) {
        walk_property_hook_list_mut(self, node);
    }

    fn visit_property_hook(&mut self, node: &mut PropertyHook) {
        walk_property_hook_mut(self, node);
    }

    fn visit_property_hook_body(&mut self, node: &mut PropertyHookBody) {
        walk_property_hook_body_mut(self, node);
    }

    fn visit_concrete_property_hook_body(&mut self, node: &mut ConcretePropertyHookBody) {
        walk_concrete_property_hook_body_mut(self, node);
    }

    fn visit_concrete_property_hook_body_block(
        &mut self,
        node: &mut ConcretePropertyHookBodyBlock,
    ) {
        walk_concrete_property_hook_body_block_mut(self, node);
    }

    fn visit_concrete_property_hook_body_expression(
        &mut self,
        node: &mut ConcretePropertyHookBodyExpression,
    ) {
        walk_concrete_property_hook_body_expression_mut(self, node);
    }

    fn visit_property_hook_kind(&mut self, node: &mut PropertyHookKind) {}

    fn visit_property_entry(&mut self, node: &mut PropertyEntry) {
        walk_property_entry_mut(self, node);
    }

    fn visit_property_entry_kind(&mut self, node: &mut PropertyEntryKind) {
        walk_property_entry_kind_mut(self, node);
    }

    fn visit_uninitialized_property_entry(&mut self, node: &mut UninitializedPropertyEntry) {
        walk_uninitialized_property_entry_mut(self, node);
    }

    fn visit_initialized_property_entry(&mut self, node: &mut InitializedPropertyEntry) {
        walk_initialized_property_entry_mut(self, node);
    }

    fn visit_trait_body(&mut self, node: &mut TraitBody) {
        walk_trait_body_mut(self, node);
    }

    fn visit_trait_statement(&mut self, node: &mut TraitStatement) {
        walk_trait_statement_mut(self, node);
    }

    fn visit_trait_usage(&mut self, node: &mut TraitUsage) {
        walk_trait_usage_mut(self, node);
    }

    fn visit_trait_usage_adaptation(&mut self, node: &mut TraitUsageAdaptation) {
        walk_trait_usage_adaptation_mut(self, node);
    }

    fn visit_trait_usage_adaptation_kind(&mut self, node: &mut TraitUsageAdaptationKind) {
        walk_trait_usage_adaptation_kind_mut(self, node);
    }

    fn visit_trait_usage_adaptation_alias(&mut self, node: &mut TraitUsageAdaptationAlias) {
        walk_trait_usage_adaptation_alias_mut(self, node);
    }

    fn visit_trait_usage_adaptation_visibility(
        &mut self,
        node: &mut TraitUsageAdaptationVisibility,
    ) {
        walk_trait_usage_adaptation_visibility_mut(self, node);
    }

    fn visit_trait_usage_adaptation_precedence(
        &mut self,
        node: &mut TraitUsageAdaptationPrecedence,
    ) {
        walk_trait_usage_adaptation_precedence_mut(self, node);
    }

    fn visit_catch_type(&mut self, node: &mut CatchType) {
        walk_catch_type_mut(self, node);
    }

    fn visit_catch_type_kind(&mut self, node: &mut CatchTypeKind) {
        walk_catch_type_kind_mut(self, node);
    }

    fn visit_catch_type_kind_identifier(&mut self, node: &mut CatchTypeKindIdentifier) {
        walk_catch_type_kind_identifier_mut(self, node);
    }

    fn visit_catch_type_kind_union(&mut self, node: &mut CatchTypeKindUnion) {
        walk_catch_type_kind_union_mut(self, node);
    }

    fn visit_try_statement(&mut self, node: &mut TryStatement) {
        walk_try_statement_mut(self, node);
    }

    fn visit_catch_block(&mut self, node: &mut CatchBlock) {
        walk_catch_block_mut(self, node);
    }

    fn visit_finally_block(&mut self, node: &mut FinallyBlock) {
        walk_finally_block_mut(self, node);
    }

    fn visit_variable(&mut self, node: &mut Variable) {
        walk_variable_mut(self, node);
    }

    fn visit_simple_variable(&mut self, node: &mut SimpleVariable) {}

    fn visit_variable_variable(&mut self, node: &mut VariableVariable) {
        walk_variable_variable_mut(self, node);
    }

    fn visit_braced_variable_variable(&mut self, node: &mut BracedVariableVariable) {
        walk_braced_variable_variable_mut(self, node);
    }

    fn visit_ending(&mut self, node: &mut Ending) {}

    fn visit_static_statement(&mut self, node: &mut StaticStatement) {
        walk_static_statement_mut(self, node);
    }

    fn visit_switch_statement(&mut self, node: &mut SwitchStatement) {
        walk_switch_statement_mut(self, node);
    }

    fn visit_echo_statement(&mut self, node: &mut EchoStatement) {
        walk_echo_statement_mut(self, node);
    }

    fn visit_return_statement(&mut self, node: &mut ReturnStatement) {
        walk_return_statement_mut(self, node);
    }

    fn visit_use_statement(&mut self, node: &mut UseStatement) {
        walk_use_statement_mut(self, node);
    }

    fn visit_group_use_statement(&mut self, node: &mut GroupUseStatement) {
        walk_group_use_statement_mut(self, node);
    }

    fn visit_halt_compiler_statement(&mut self, node: &mut HaltCompilerStatement) {}

    fn visit_static_var(&mut self, node: &mut StaticVar) {
        walk_static_var_mut(self, node);
    }

    fn visit_comment(&mut self, node: &mut Comment) {
        walk_comment_mut(self, node);
    }

    fn visit_comment_kind(&mut self, node: &mut CommentKind) {
        walk_comment_kind_mut(self, node);
    }

    fn visit_single_line_comment(&mut self, node: &mut SingleLineComment) {}

    fn visit_multi_line_comment(&mut self, node: &mut MultiLineComment) {}

    fn visit_hash_mark_comment(&mut self, node: &mut HashMarkComment) {}

    fn visit_doc_block_comment(&mut self, node: &mut DocBlockComment) {
        walk_doc_block_comment_mut(self, node);
    }

    fn visit_doc_block(&mut self, node: &mut DocBlock) {
        walk_doc_block_mut(self, node);
    }

    fn visit_doc_block_node(&mut self, node: &mut DocBlockNode) {
        walk_doc_block_node_mut(self, node);
    }

    fn visit_doc_block_text_node(&mut self, node: &mut DocBlockTextNode) {}

    fn visit_doc_block_tag_node(&mut self, node: &mut DocBlockTagNode) {
        walk_doc_block_tag_node_mut(self, node);
    }

    fn visit_doc_block_tag(&mut self, node: &mut DocBlockTag) {
        walk_doc_block_tag_mut(self, node);
    }

    fn visit_doc_block_param_closure_this_tag(&mut self, node: &mut DocBlockParamClosureThisTag) {
        walk_doc_block_param_closure_this_tag_mut(self, node);
    }

    fn visit_doc_block_param_tag(&mut self, node: &mut DocBlockParamTag) {
        walk_doc_block_param_tag_mut(self, node);
    }

    fn visit_doc_block_return_tag(&mut self, node: &mut DocBlockReturnTag) {
        walk_doc_block_return_tag_mut(self, node);
    }

    fn visit_doc_block_throws_tag(&mut self, node: &mut DocBlockThrowsTag) {
        walk_doc_block_throws_tag_mut(self, node);
    }

    fn visit_doc_block_var_tag(&mut self, node: &mut DocBlockVarTag) {
        walk_doc_block_var_tag_mut(self, node);
    }

    fn visit_doc_block_property_tag(&mut self, node: &mut DocBlockPropertyTag) {
        walk_doc_block_property_tag_mut(self, node);
    }

    fn visit_doc_block_method_tag(&mut self, node: &mut DocBlockMethodTag) {
        walk_doc_block_method_tag_mut(self, node);
    }

    fn visit_doc_block_template_tag(&mut self, node: &mut DocBlockTemplateTag) {
        walk_doc_block_template_tag_mut(self, node);
    }

    fn visit_doc_block_extends_tag(&mut self, node: &mut DocBlockExtendsTag) {
        walk_doc_block_extends_tag_mut(self, node);
    }

    fn visit_doc_block_implements_tag(&mut self, node: &mut DocBlockImplementsTag) {
        walk_doc_block_implements_tag_mut(self, node);
    }

    fn visit_doc_block_uses_tag(&mut self, node: &mut DocBlockUsesTag) {
        walk_doc_block_uses_tag_mut(self, node);
    }

    fn visit_doc_block_deprecated_tag(&mut self, node: &mut DocBlockDeprecatedTag) {}

    fn visit_doc_block_generic_tag(&mut self, node: &mut DocBlockGenericTag) {}

    fn visit_comment_group(&mut self, node: &mut CommentGroup) {}
}
