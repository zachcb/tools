# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-destructuring-assignment-array-pattern > nested-assignment`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
		end: Object {
			column: 0
			index: 26
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
				end: Object {
					column: 25
					index: 25
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			expression: JSAssignmentExpression {
				operator: "="
				loc: Object {
					filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
					end: Object {
						column: 24
						index: 24
						line: 1
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				right: JSNumericLiteral {
					value: 0
					format: undefined
					loc: Object {
						filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
						end: Object {
							column: 24
							index: 24
							line: 1
						}
						start: Object {
							column: 23
							index: 23
							line: 1
						}
					}
				}
				left: JSAssignmentArrayPattern {
					rest: undefined
					loc: Object {
						filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
						end: Object {
							column: 22
							index: 22
							line: 1
						}
						start: Object {
							column: 0
							index: 0
							line: 1
						}
					}
					elements: Array [
						JSAssignmentIdentifier {
							name: "a"
							loc: Object {
								filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
								identifierName: "a"
								end: Object {
									column: 2
									index: 2
									line: 1
								}
								start: Object {
									column: 1
									index: 1
									line: 1
								}
							}
						}
						JSAssignmentAssignmentPattern {
							operator: "="
							loc: Object {
								filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
								end: Object {
									column: 6
									index: 6
									line: 1
								}
								start: Object {
									column: 3
									index: 3
									line: 1
								}
							}
							left: JSAssignmentIdentifier {
								name: "b"
								loc: Object {
									filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
									identifierName: "b"
									end: Object {
										column: 4
										index: 4
										line: 1
									}
									start: Object {
										column: 3
										index: 3
										line: 1
									}
								}
							}
							right: JSNumericLiteral {
								value: 0
								format: undefined
								loc: Object {
									filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
									end: Object {
										column: 6
										index: 6
										line: 1
									}
									start: Object {
										column: 5
										index: 5
										line: 1
									}
								}
							}
						}
						JSAssignmentAssignmentPattern {
							operator: "="
							loc: Object {
								filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
								end: Object {
									column: 21
									index: 21
									line: 1
								}
								start: Object {
									column: 7
									index: 7
									line: 1
								}
							}
							right: JSObjectExpression {
								properties: Array []
								loc: Object {
									filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
									end: Object {
										column: 21
										index: 21
										line: 1
									}
									start: Object {
										column: 19
										index: 19
										line: 1
									}
								}
							}
							left: JSAssignmentArrayPattern {
								loc: Object {
									filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
									end: Object {
										column: 18
										index: 18
										line: 1
									}
									start: Object {
										column: 7
										index: 7
										line: 1
									}
								}
								elements: Array [
									JSAssignmentIdentifier {
										name: "c"
										loc: Object {
											filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
											identifierName: "c"
											end: Object {
												column: 9
												index: 9
												line: 1
											}
											start: Object {
												column: 8
												index: 8
												line: 1
											}
										}
									}
								]
								rest: JSMemberExpression {
									loc: Object {
										filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
										end: Object {
											column: 17
											index: 17
											line: 1
										}
										start: Object {
											column: 13
											index: 13
											line: 1
										}
									}
									object: JSReferenceIdentifier {
										name: "a"
										loc: Object {
											filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
											identifierName: "a"
											end: Object {
												column: 14
												index: 14
												line: 1
											}
											start: Object {
												column: 13
												index: 13
												line: 1
											}
										}
									}
									property: JSComputedMemberProperty {
										value: JSNumericLiteral {
											value: 0
											format: undefined
											loc: Object {
												filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
												end: Object {
													column: 16
													index: 16
													line: 1
												}
												start: Object {
													column: 15
													index: 15
													line: 1
												}
											}
										}
										loc: Object {
											filename: "esprima/es2015-destructuring-assignment-array-pattern/nested-assignment/input.js"
											end: Object {
												column: 17
												index: 17
												line: 1
											}
											start: Object {
												column: 14
												index: 14
												line: 1
											}
										}
									}
								}
							}
						}
					]
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```