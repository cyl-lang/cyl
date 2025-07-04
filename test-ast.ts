// Test file to verify AST types work correctly
import { ASTNodeType, Identifier, IntLiteral, Expression } from './design/src/generated/generated_ast';

// Test creating an Identifier node
const identifierNode: Identifier = {
    type: ASTNodeType.Identifier,
    name: "myVariable"
};

// Test creating an IntLiteral node  
const intLiteralNode: IntLiteral = {
    type: ASTNodeType.IntLiteral,
    value: 42
};

// Test using Expression union type
const expressions: Expression[] = [
    identifierNode,
    intLiteralNode
];

console.log("AST types work correctly!");
console.log("Identifier:", identifierNode);
console.log("IntLiteral:", intLiteralNode);
console.log("Expressions:", expressions);

// EXPECTED OUTPUT
/**
 * AST types work correctly!
 * Identifier: { type: 'Identifier', name: 'myVariable' }
 * IntLiteral: { type: 'IntLiteral', value: 42 }
 * Expressions: [
 *   { type: 'Identifier', name: 'myVariable' },
 *   { type: 'IntLiteral', value: 42 }
 * ]
 */