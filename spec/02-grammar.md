## Grammar of Units and Unit Terms

**§1 preliminaries** {#para-1}

1. _The Unified Code for Units of Measure_ consists of a basic set of terminal symbols for units, called _atomic unit symbols_ or _unit atoms_, and multiplier prefixes. It also consists of an expression syntax by which these symbols can be combined to yield valid units. 
2. The tables of terminal symbols are fixed as of every revision of _The Unified Code for Units of Measure_, additions, deletions or changes are _not_ allowed. 
3. All expression that can be derived from these terminal symbols and the expression syntax are valid codes. Any expression of _The Unified Code for Units of Measure_ has a precisely defined semantics.

The expression syntax of _The Unified Code for Units of Measure_ generates an infinite number of codes with the consequence that it is impossible to compile a table of all valid units.

That the tables of terminal symbols may not be extended does not mean that missing symbols will never be available in _The Unified Code for Units of Measure_. Suggestions for additions of new symbols are welcome and revisions of _The Unified Code for Units of Measure_ will be released as soon as a change request has been approved.

**§2 full and limited conformance** {#para-2}

1. The semantics of _The Unified Code for Units of Measure_ implies equivalence classes such that different expressions may have the same meaning. 
2. Programs that declare _full conformance_ with _The Unified Code for Units of Measure_ must compare unit expressions by their semantics, i.e. they must detect equivalence for different expressions with the same meaning. 
3. Programs with _limited conformance_ may compare unit expressions literally and thus may not detect equivalence of unit expressions.

The option for "limited conformance" allows _The Unified Code for Units of Measure_ to be adopted even by less powerful systems that can not or do not want to deal with the full semantics of units. Those systems typically have a table of fixed unit expression literals that may be related to other literals with fixed conversion factors. Although these systems will have difficulties to receive unit expressions from various sources, they will at least send out valid expressions of _The Unified Code for Units of Measure_, which is an important step towards a commonly used coding scheme for units.

### 2.1 Character Set and Lexical Rules 

**§3 character set** {#para-3}

1. All expressions of _The Unified Code for Units of Measure_ shall be built from characters of the 7-bit US-ASCII character set exclusively. 
2. Terminal unit symbols can consist of all ASCII characters in the range of 33–126 (0x21–0x7E) excluding double quotes ('`"`'), parentheses ('`(`' and '`)`'), plus sign ('`+`''), minus sign ('`-`''), period ('`.`''), solidus ('`/`''), equal sign ('`=`''), square brackets ('`[`' and '`]`'), and curly braces ('`{`' and '`}`'), which have special meaning. 
3. A terminal unit symbol can not consist of only digits ('`0`'–'`9`') because those digit strings are interpreted as positive integer numbers. However, a symbol "`10*`" is allowed because it ends with a non-digit allowed to be part of a symbol. 
4. For every terminal symbol there is a case insensitive variant defined, to be used when there is a risk of upper and lower case to be confused. Although upper and lower case can be mixed in case insensitive symbols there is no meaning to the case. Case insensitive symbols are incompatible to the case sensitive symbols.

The 7-bit US-ASCII character code is the greatest common denominator that can be expected to be available in any communication environment. Only very few units normally require symbols from the Greek alphabet and thus the cost of requiring Unicode does not outweigh the benefit. As explained above, the real issue about writing unit terms naturally is not the character set but the ability to write subscripts and superscripts and distinguish roman letters from italics.

Some computer systems or programming languages still have the requirement of case insensitivity and some humans who are not familiar with SI units tend to confuse upper and lower case or can not interpret the difference in upper and lower case correctly. For this reason the case insensitive symbols are defined. Although _The Unified Code for Units of Measure_ does not encourage use of case insensitive symbols where not absolutely necessary, in some circumstances the case insensitive representation may be the greatest common denominator. Thus some systems that can handle case sensitivity may end up using case insensitive symbols in order to communicate with less powerful systems.

ISO 2955 and ANSI X3.50 call case sensitive symbols "mixed case" and case insensitive symbols "single case" and list two columns for "single case" symbols, one for upper case and one for lower case. In _The Unified Code for Units of Measure_ all units can be written in mixed upper and lower case, but in the case insensitive variant the mixing of case does not matter.

White space is not recognized in a a unit term and should generally not occur. UCUM implementations may flag whitespace as an error rather than ignore it. Whitespace is not used as a separator of otherwise ambiguous parts of a unit term.

**§4 prefixes** {#para-4}

1. Metric units (cf. [§11](#para-11)) may be combinations of a unit symbol with a prefix symbol. 
2. The unit symbol to be combined with the prefix must not itself contain a prefix. Such a prefix-less unit symbol is called _unit atom_. 
3. Prefix and atom are connected immediately without any delimiter. Separation of an optional prefix from the atom occurs on the lexical level by finding a matching combination of an optional prefix and a unit atom. 
4. The prefix is the longest leading substring that matches a valid prefix where the remainder is a valid metric unit atom. If no such prefix can be matched, the unit atom is without prefix and may be both metric or non-metric.\[1–3: ISO 1000, 3; ISO 2955-1983, 3.7; ANSI X3.50-1986, 3.7 (Rule No. 6).\]

**§5 square brackets** {#para-5}

1. Square brackets ('`[`' and '`]`') may be part of a unit atom at any place but only as matched pairs. Square brackets are lexical elements and not separate syntactical tokens. 
2. Within a matching pair of square brackets the full range of characters 33–126 can be used.[3](#fn3) 
3. Square brackets do _not_ determine the boundary between prefix and unit atom, but they never span the boundary of unit atoms. 
4. Square brackets must not be nested.

For example % "`[abc+ef]`", "`ab[c+ef]`", "`[abc+]ef`", and "`ab[c+ef]`" % could all be valid symbols if defined in the tables. In "`ab[c+ef]`" either "`a`" or "`ab`" could be defined as a prefix, but not "`ab[c`".

Square brackets take on one task of round parentheses in HL7's "ISO+" code, where one use of parentheses is to augment unit symbols with suffixes, as in "`mm(Hg)`". Another use is to enclose one full unit symbol into parentheses, as "`(ka_u)`" (for the King-Armstrong unit of catalytic amount of phosphatase). Apparently, in a unit symbol such enclosed one is supposed not to expect a prefix. Thus, even if "`a_u`" would have been defined, "`(ka_u)`" should not be matched against kilo-`a_u`.

Parentheses, however, were also used for the nesting of terms since HL7 version 2.3. At this point it became ambiguous whether parentheses are part of the unit symbol or whether they are syntactic tokens. For instance, "`(ka_u)`" could mean a nested "`ka_u`" (where "`k`" could possibly be a prefix), but also the proper symbol "`(ka_u)`" that happens to have parentheses as part of the symbol. _The Unified Code for Units of Measure_ uses parentheses for the usual meaning of term nesting and uses square brackets where HL7's "ISO+" assumes parentheses to be part of the unit symbol.

**§6 curly braces** {#para-6}

1. The full range of characters 33–126 can be used within a pair of curly braces ('`{`' and '`}`'). The material enclosed in curly braces is called _annotation_. 
2. Annotations do not contribute to the semantics of the unit but are meaningless by definition. Therefore, any fully conformant parser must discard all annotations. Parsers of limited conformance _should_ not value annotations in comparison of units. 
3. Annotations do, however, signify the end of a unit symbol. 
4. An annotation without a leading symbol implies the default unit 1 (the unity). 
5. Curly braces must not be nested.

Curly braces are here because people want annotations and deeply believe that they need annotations. Especially in chemistry and biomedical sciences, there are traditional habits to write annotations at units or instead of units, such as "%vol.", "RBC", "CFU", "kg(wet tis.)", or "mL(total)". These habits are hard to overcome. Any attempt of a coding scheme to restrict this perceived expressiveness will ultimately result in the coding scheme not being adopted, or just "half-way" adopted (which is as bad as not adopted).

Two alternative responses to this reality exist: either give in to the bad habits and blow up of the code with dimension- and meaningless unit atoms, or canalize this habit so that it does no harm. _The Unified Code for Units of Measure_ canalizes this habit using curly braces. Nevertheless we do continuing efforts to upgrade doubtful units to genuine units of _The Unified Code for Units of Measure_ by defining and linking them to the other units as good as possible. Thus, "`g%`" is a valid metric unit atom (so that "`mg%`" is a valid unit too.) A _drops_, although quite imprecise, is a valid unit of volume "`[drp]`". Even HPF and LPF (the so called "high-" and "low power field" in the microscope) have been defined so that at least they relate to each other.

### 2.2 Syntax Rules 

**§7 algebraic unit terms** {#para-7}

1. All units can be combined in an algebraic term using the operators for multiplication (period '`.`') and division (solidus '`/`'). 
2. The multiplication operator is mandatory it must not be omitted or replaced by a space. The multiplication operator is a strict binary operator that may occur only _between two_ unit terms. 
3. The division operator can be used as a binary and unary operator, i.e. a leading solidus will invert the unit that directly follows it. 
4. Terms are evaluated from left to right with the period and the solidus having the same operator precedence. Multiple division operators are allowed within one term. \[ISO 1000, 4.5.2; ISO 2955-1983, 3.3f; ANSI X3.50-1986, 3.3f (Rule No. 2f).\]

The use of the period instead of the asterisk ('`*`') as a multiplication operator continues a tradition codified in ISO 1000 and maintained in ISO 2955. Because floating point numbers may not occur in unit terms the period is not ambiguous. A period in a unit term has no other meaning than to be the multiplication operator.

Since Resolution 7 of the 9th CGPM in 1948 the myth of ambiguity being introduced by more than one solidus lives on and is quoted in all standards concerning the writing of SI units. However, when the strict left to right rule is followed there is no ambiguity, neither with one solidus nor with more than one solidus. However, in human practice we find the tendency to assign a lower precedence to the solidus which misleads people to write _a_/_b_·_c_ when they really mean _a_/(_b_·_c_). When this is rewritten as _a_/_b_/_c_ there is actually less ambiguity that in _a_/_b_·_c_. So the real source of ambiguity is when a multiplication operator follows a solidus, not when there is more than one solidus in a term. Hence, we remove the restriction for only one solidus and introduce parentheses which may be used to remove any perceived ambiguity.

**§8 integer numbers** {#para-8}

1. A positive integer number may appear in place of a simple unit symbol. 
2. Only a pure string of decimal digits ('`0`'–'`9`') is interpreted as a number. If after one or more digits there is any non-digit character found that is valid for unit atoms, all the characters (including the digits) will be interpreted as a simple unit symbol.

For example, the string "`123`" is a positive integer number while "`12a`" is a symbol.

Note that the period is only used as a multiplication operator, thus "`2.5`" means 2 × 5 and is not equal to 5/2.

**§9 exponents** {#para-9}
1. Simple units may be raised to a power. The exponent is an integer number and is written immediately behind the unit term. Negative exponents must be preceded by a minus sign ('`-`' positive exponents may be preceded by an optional plus sign ('`+`'). 
2. If the simple unit raised to a power is a combination of a prefix and a unit atom, both are raised to the power, e.g. "1 `cm3`" equals "10-6`m3`" not "10-2`m3`". \[ISO 2955-1983, 3.5f; ANSI X3.50-1986, 3.5f (Rule No. 4f).\]

ISO 2955 and ANSI X3.50 actually do not allow a plus sign leading a positive exponent. However, if there can be any perceived ambiguities, an explicit leading plus sign may be of help sometimes. _The Unified Code for Units of Measures_ therefore allows such plus signs at exponents. The plus sign on positive exponents can be used to delimit exponents from integer numbers used as simple units. Thus, `2+10` means 210 = 1024.

**§10 nested terms** {#para-10}
1. Unit terms with operators may be enclosed in parentheses ('`(`' and '`)`') and used in place of simple units. Normal left-to-right evaluation can be overridden with parentheses. 
2. Parenthesized terms are _not_ considered unit atoms and hence must not be preceded by a prefix.

Up until revision 1.9 there was a third clause "Since a unit term in parenthesis can be used in place of a simple unit, an exponent may follow on a closing parenthesis which raises the whole term within the parentheses to the power." However this feature was inconsistent with any BNF or other syntax description ever provided, was never used and seems to have no relevant use case. For this reason this clause has been stricken. This is a _tentative_ change. Users who have used this feature in the past, should please comment on this deprecation. If we receive indication that this feature was used by anyone, we would undo the deprecation. If no comments are received, the deprecation continues to take effect.

Exhibit 1: The complete syntax in the Backus-Naur Form.

See [./grammar.g4](./grammar.g4) for the complete syntax in the Backus-Naur Form.

![Pushdown-state automaton describing the syntax.](https://raw.githubusercontent.com/ucum-org/ucum/main/assets/images/ucum-state-automaton.gif)

### 2.3 The Predicate "Metric"

**§11 metric and non-metric unit atoms** {#para-11}

1. Only metric unit atoms may be combined with a prefix. 
2. To be metric or not to be metric is a predicate assigned to each unit atom where that unit atom is defined. 
3. All base units are metric. No non-metric unit can be part of the basis. 
4. A unit must be a quantity on a ratio scale in order to be metric.

The metric predicate accounts for the fact that there are units that are prefixed and others that are not. This helps to disambiguate the parsing of simple units into prefix and atom.

To determine whether a given unit atom is metric or not is not trivial. It is a cultural phenomenon, subject to change, just like language, the meaning of words and how words can be used. At one time we can clearly tell right or wrong usage of words, but these decisions may need to be revised with the passage of time.

Generally, metric units are those defined "in the spirit" of the metric system, that emerged in France of the 18th century and was rapidly adopted by scientists. Metric units are usually based on reproducible natural phenomena and are usually not part of a system of comparable units with different magintudes, especially not if the ratios of these units are not powers of 10. Instead, metric units use multiplier prefixes that magnify or diminish the value of the unit by powers of ten.

Conversely, customary units are in the spirit of the middle age as most of them can be traced back into a time around the 10th century, some are even older from the Roman and Babylonian empires. Most customary units are based on the average size of human anatomical or botanic structures (e.g., foot, ell, fathom, grain, rod) and come in series of comparable units with ratios 1/2, 1/4, 1/12, 1/16, and others. Thus all customary units are non-metric

Not all units from ISO 1000 are metric as degree, minute and second of plane angle are non-metric as well as minute, hour, day, month, and year. The second is a metric unit because it is a part of the SI basis, although it used to be part of a series of customary units (originating in the Babylonian era).

Furthermore, for a unit to be metric it must be a quantity on a ratio scale where multiplication and division with scalars are defined. The _Comité Consultatif d'Unités_ (CCU) decided in February 1995 that SI prefixes may be used with the degree Celsius. This statement has not been made explicitly before. This is an unfortunate decision because difference-scale units like the degree Celsius have no multiplication operation, so that the prefix value could be multiplied with the unit. Instead the prefix at non-ratio units scales the measurement value. One dekameter is 10 times of a meter, but there is no meaning to 10 times of 1 °C in the same way as 30 °C are not 3 times as much as 10 °C. See [§§21ff](#para-21) on how _The Unified Code for Units of Measure_ finds a way to accommodate this different use of prefixes at units such as the degree Celsius, bel or neper.

| ### 2.4 Style

Except for the rule on curly braces ([§12](#para-12)), the rules on style govern the creation of the tables of unit atoms not their individual use. Users of _The Unified Code for Units of Measure_ need not care about style rules ([§§13–15](#para-13)) because users just use the symbols defined in the tables. Hence, style rules do not affect conformance to _The Unified Code for Units of Measure_. New submissions of unit atoms, however, must conform to the style rules.

**§12 curly braces** {#para-12}
1. Curly braces may be used to enclose annotations that are often written in place of units or behind units but that do not have a proper meaning of a unit and do not change the meaning of a unit. 
2. Annotations have no semantic value.

For example one can write "`%{vol}`", "`kg{total}`", or "`{RBC}`" (for "red blood cells") as pseudo-units. However, these annotations do not have any effect on the semantics, which is why these example expressions are equivalent to "`%`", "`kg`", and "`1`" respectively.

**§13 underscore** {#para-13}
1. When in print a unit would have a subscript, an underscore ('`_`') is used to separate the subscript from the stem of the unit symbol. 
2. The subscript is part of the unit atom. 
3. subscripts are used to disambiguate the two units with the same name but different meanings.

For example when distinguishing the International Table calorie from the thermochemical calorie, we would use 1 calIT or 1 calth in print. _The Unified Code for Units of Measure_ defines the symbols "`cal_IT`" and "`cal_th`" with the underscore signifying that "IT" and "th" are subscripts. Other examples are the distinctions between the Julian and Gregorian calendar year from the tropical year or the British imperial gallon from the U.S. gallon (see [§31](#para-31) and [§§37ff](#para-37)).

**§14 square brackets** {#para-14}
1. Square brackets enclose suffixes of unit symbols that change the meaning of a unit stem. 
2. All customary units shall be enclosed completely by square brackets. 
3. Other unit atoms shall be enclosed in square brackets if they are very rare, if they will conflict with other units, or if they are normally not used as a unit symbol but do have a proper meaning as a unit in _The Unified Code for Units of Measure_. 
4. Square brackets are part of the unit atom.

For example 1 m H2O is written as "`m[H2O]`" in _The Unified Code for Units of Measure_ because the suffix H2O changes the meaning of the unit atom for meter (length) to a unit of pressure.

Customary units are defined in _The Unified Code for Units of Measure_ in order to accommodate practical needs. However metric units are still preferred and the customary symbols should not interfere with metric symbols in any way. Thus, customary units are "stigmatized" by enclosing them into square brackets.

If unit symbols for the purpose of display and print are derived from _The Unified Code for Units of Measure_ units, the square brackets can be removed. However, display units are out of scope of _The Unified Code for Units of Measure_.

**§15 apostrophe** {#para-15}
1. The apostrophe ('`'`') is used to separate words or abbreviated words in a multi-word unit symbol. 
2. Since units are mathematically defined symbols and not abbreviations of words, multi-word unit symbols should be defined only to reflect existing habits, not in order to create new ones. 
3. Multi-word units should always be enclosed in square brackets.

For example, such legacy units called "Bodansky unit" or "Todd unit" have the unit symbols "`[bdsk'U]`", and "`[todd'U]`" respectively.