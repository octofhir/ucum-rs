# UCUM Glossary

## Core Concepts

**Unified Code for Units of Measure (UCUM)**  
The Unified Code for Units of Measure is a comprehensive code system designed to enable unambiguous electronic communication of quantities with their units in various domains including science, engineering, medicine, and commerce. UCUM provides a systematic approach to representing units of measure through a defined syntax and semantics that ensures each unit expression has exactly one meaning. The system covers both metric and non-metric units, supports algebraic operations on units, and includes provisions for special units that don't follow ratio scales. UCUM's primary goal is to eliminate ambiguity in unit representation across different systems and contexts while maintaining compatibility with existing standards and practices.

**Unit Atom**  
A unit atom is the most basic, indivisible unit symbol in the UCUM system that represents a single unit of measurement without any prefix or modification. Unit atoms serve as the fundamental building blocks from which more complex unit expressions are constructed through algebraic operations or prefix application. Examples include 'm' for meter, 's' for second, and 'g' for gram. Each unit atom has specific properties including whether it accepts prefixes (metric property), its dimension, and its magnitude relative to the coherent unit of its dimension. Unit atoms are defined in the UCUM unit atom table and form the terminal symbols of the UCUM grammar.

**Atomic Unit Symbol**  
An atomic unit symbol is a terminal symbol in the UCUM grammar that represents a basic unit in the system's vocabulary. These symbols are "atomic" in the sense that they cannot be further decomposed into smaller meaningful parts within the UCUM syntax. Atomic unit symbols include all the base units, derived units, and special units defined in the UCUM tables. They are distinguished from composite unit expressions that combine multiple atomic symbols through operations. Each atomic unit symbol has a precisely defined meaning, dimension, and relationship to other units in the system.

**Terminal Symbol**  
Terminal symbols are the fixed, predefined symbols in the UCUM grammar that cannot be extended or decomposed further. They consist of two main categories: unit atoms (like 'm', 'g', 's') and prefixes (like 'k', 'm', 'u'). Terminal symbols are the leaves of the parse tree when analyzing UCUM expressions and represent the fundamental vocabulary from which all valid unit expressions are constructed. Unlike non-terminal symbols which represent grammatical structures, terminal symbols have direct semantic meaning in terms of measurement units or scaling factors.

**Unit Term**  
A unit term is a complete expression in UCUM that represents a unit of measure, constructed by combining unit atoms with algebraic operators according to the UCUM grammar rules. Unit terms can range from simple (a single unit atom) to complex (multiple units combined with multiplication, division, and exponentiation). Examples include 'kg.m/s2' for newton, 'mol/L' for molar concentration, and 'mm[Hg]' for millimeters of mercury. Unit terms follow specific syntactic rules including left-to-right evaluation, mandatory multiplication operators, and proper use of parentheses for grouping.

## Unit Categories

**Base Units**  
Base units are the seven fundamental units of measurement that form the foundation of the UCUM system and from which all other units are derived. These units are: meter (m) for length, second (s) for time, gram (g) for mass, radian (rad) for plane angle, kelvin (K) for temperature, coulomb (C) for electric charge, and candela (cd) for luminous intensity. Base units are dimensionally independent, meaning none can be expressed in terms of the others. They establish the dimensional framework for the entire system of units and serve as the reference points for defining all derived units through algebraic relationships.

**Derived Units**  
Derived units are units that are defined in terms of base units through algebraic expressions, representing measurements of quantities that can be expressed as combinations of the base dimensions. Examples include the newton (kg.m/s²) for force, the joule (kg.m²/s²) for energy, and the pascal (kg/m.s²) for pressure. Derived units can be expressed either through their defining algebraic formula or given special names and symbols for convenience. The UCUM system includes a comprehensive set of derived units covering various scientific and technical domains, each with precisely defined relationships to the base units.

**Metric Units**  
Metric units are units that can be combined with metric prefixes to create scaled versions representing different orders of magnitude. A unit's metric property is a fundamental characteristic that determines whether prefixes like kilo-, milli-, or micro- can be applied to it. Most SI units are metric, allowing expressions like kilometer (km), milligram (mg), or microsecond (us). However, not all units in UCUM are metric; for example, units already containing a prefix or certain special units cannot accept additional prefixes. The metric property is explicitly defined for each unit atom in the UCUM tables.

**Non-metric Units**  
Non-metric units are units that cannot be combined with metric prefixes, either because they already contain a prefix, represent non-ratio scale measurements, or are defined as inherently non-metric in the UCUM specification. Examples include the kilogram (kg), which already contains the prefix 'kilo', and various customary units like feet or pounds. Non-metric units must be used as-is without prefix modification. This distinction is important for proper UCUM expression validation and ensures that nonsensical combinations like "kilo-kilogram" are prevented.

**Special Units**  
Special units are units that represent measurements on non-ratio scales, meaning they don't support general multiplication and division operations in the same way as regular units. These include temperature units like degree Celsius (Cel), logarithmic units like pH and decibel (dB), and other specialized scales. Special units often require special conversion functions rather than simple scaling factors when converting to other units. They are marked with specific indicators in UCUM and have restricted algebraic properties to prevent meaningless operations like "pH squared" or "Celsius per meter".

**Arbitrary Units**  
Arbitrary units are units whose precise meaning depends on the specific measurement procedure, context, or convention rather than having a fixed relationship to physical standards. These units are commonly used in laboratory medicine, biology, and other fields where the measurement method itself defines the unit. Examples include enzyme activity units ([IU]), colony forming units ([CFU]), and various assay-specific units. Arbitrary units are typically enclosed in square brackets in UCUM notation and cannot be meaningfully converted to other units without additional context about the measurement procedure.

**Customary Units**  
Customary units are traditional units of measurement that are not part of the metric system but are included in UCUM for practical compatibility with existing systems and data. These units, such as feet ([ft_i]), pounds ([lb_av]), and fluid ounces ([foz_us]), are enclosed in square brackets to distinguish them from metric units. Customary units often have precisely defined relationships to metric units (e.g., 1 international foot = 0.3048 meters exactly) but retain their traditional names and symbols. UCUM includes various national and international customary unit systems to ensure comprehensive coverage.

## Grammar and Syntax

**Prefix**  
A prefix in UCUM is a symbol that represents a decimal scaling factor applied to a metric unit, allowing expression of quantities across many orders of magnitude. The standard metric prefixes range from yotta- (10²⁴) to yocto- (10⁻²⁴), with common examples including kilo- (k, 10³), milli- (m, 10⁻³), and micro- (u, 10⁻⁶). Prefixes can only be applied to units with the metric property set to true, and they must appear immediately before the unit symbol without any separator. The prefix system enables concise representation of very large or very small quantities while maintaining precision and readability.

**Case Sensitive (c/s)**  
Case sensitivity in UCUM refers to unit symbols where the capitalization of letters carries meaning and must be preserved exactly as specified. For case-sensitive symbols, 'M' (mega-) and 'm' (meter or milli-) represent entirely different concepts, and this distinction is critical for correct interpretation. Most metric unit symbols and prefixes are case-sensitive to allow for a compact notation that can distinguish between many different units and scales. The case-sensitive property is indicated in the UCUM tables and must be respected by conforming implementations to avoid ambiguity.

**Case Insensitive (c/i)**  
Case insensitive symbols in UCUM are those where capitalization does not affect meaning, allowing flexibility in representation while maintaining unambiguous interpretation. These symbols can be written in any combination of upper and lower case letters without changing their meaning. Case insensitive symbols are typically used for units where there is no risk of confusion with other symbols, or for certain customary units where traditional usage varies. The UCUM specification clearly marks which symbols are case insensitive to guide proper implementation.

**Square Brackets []**  
Square brackets in UCUM serve multiple purposes: they enclose customary units to distinguish them from metric units, indicate certain arbitrary units, and mark special annotations or qualifiers. When a unit symbol appears in square brackets, it signals that this is not a standard metric unit and may have special properties or limitations. Examples include [ft_i] for international foot, [IU] for international unit, and [pH] for the pH scale. Square brackets are also used in certain contexts to provide additional clarification or to denote units that require special handling in calculations or conversions.

**Curly Braces {}**  
Curly braces in UCUM are used to enclose annotations, which are descriptive text that provides additional context or clarification but does not affect the formal meaning or computational properties of the unit. Annotations are considered "meaningless additions" from a computational standpoint - they are preserved for human readability but ignored in unit calculations and comparisons. For example, 'mg{total}' and 'mg{free}' both represent milligrams computationally, but the annotations help distinguish different clinical measurements. This feature allows UCUM to maintain computational clarity while supporting the descriptive needs of various domains.

**Operators**  
UCUM defines two primary algebraic operators for combining units: the period (.) for multiplication and the solidus (/) for division. The multiplication operator is mandatory and cannot be omitted - 'kg.m' not 'kgm'. Division creates a fraction with all following factors going into the denominator until overridden by parentheses or another division operator. Exponentiation is indicated by numeric suffixes (m2 for square meters). These operators follow specific precedence rules with left-to-right evaluation, and parentheses can be used to override default grouping. The careful definition of operators ensures that every valid UCUM expression has exactly one interpretation.

**Exponents**  
Exponents in UCUM are integer powers applied to units, indicated by numeric digits immediately following the unit symbol without any separator. Positive exponents represent repeated multiplication (m2 for square meters, s3 for cubic seconds), while negative exponents represent division (s-1 for "per second", m-2 for "per square meter"). Exponents must be integers - fractional exponents are not allowed in UCUM syntax. The exponent applies only to the immediately preceding unit atom or parenthesized expression, following standard algebraic conventions for clarity and unambiguous parsing.

## Semantic Concepts

**Dimension**  
Dimension in UCUM represents the fundamental nature of a physical quantity, independent of its units or scale. It defines an equivalence class of commensurable units - units that measure the same kind of physical phenomenon. The seven base dimensions correspond to the base units: length (L), time (T), mass (M), plane angle (A), temperature (Θ), electric charge (Q), and luminous intensity (F). Derived dimensions are expressed as products of powers of base dimensions; for example, velocity has dimension LT⁻¹, and force has dimension MLT⁻². Understanding dimensions is crucial for unit conversion and for verifying the dimensional consistency of calculations.

**Magnitude**  
Magnitude in UCUM refers to the numeric conversion factor that relates a unit to the coherent unit of its dimension. It represents how many coherent units are contained in one of the given unit. For example, a kilometer has magnitude 1000 relative to the meter (the coherent unit of length), while a millimeter has magnitude 0.001. Magnitudes are used in unit conversion calculations and are derived from the combination of prefix values and unit definitions. The magnitude system allows UCUM to maintain precise relationships between all units of the same dimension while supporting arbitrary precision arithmetic.

**Commensurable**  
Commensurable units are units that measure the same kind of quantity and therefore can be converted between each other through multiplication by an appropriate conversion factor. Units are commensurable if and only if they have the same dimension. For example, meters, feet, and miles are all commensurable (dimension: length), as are seconds, minutes, and hours (dimension: time). However, meters and seconds are not commensurable as they have different dimensions. The concept of commensurability is fundamental to unit conversion and to determining which unit operations are physically meaningful.

**Equivalence**  
Equivalence in UCUM refers to different unit expressions that represent the same physical measurement. Two unit terms are equivalent if they have the same dimension and their magnitudes differ only by a numeric factor. For example, 'km/h' and 'm/s' are equivalent (both representing velocity) with a defined conversion factor between them. UCUM supports both syntactic equivalence (different expressions of the same unit, like 'kg.m.s-2' and 'N') and semantic equivalence (different units measuring the same quantity). Understanding equivalence is essential for unit simplification and comparison.

**Full Conformance**  
Full conformance in UCUM refers to implementations that can parse unit expressions, understand their semantic meaning, and perform operations based on dimensional analysis and magnitude calculations. Fully conformant systems can determine if units are commensurable, perform unit conversions, simplify unit expressions, and validate dimensional consistency in calculations. They maintain the full semantic model of UCUM including dimensions, magnitudes, and special unit properties. Full conformance requires more complex implementation but enables powerful unit-aware computations and validations.

**Limited Conformance**  
Limited conformance in UCUM refers to implementations that can correctly parse and display unit expressions according to UCUM syntax but do not perform semantic analysis or unit conversions. Limited conformant systems treat units as structured strings, ensuring syntactic validity and proper formatting while avoiding the complexity of dimensional analysis. This level of conformance is suitable for systems that need to store and transmit unit information reliably without performing calculations. Limited conformance provides interoperability benefits while requiring significantly less implementation complexity than full conformance.

## Special Features

**Annotations**  
Annotations in UCUM are free-text descriptions enclosed in curly braces that provide additional context or qualification to a unit without affecting its computational meaning. They allow users to distinguish between different applications of the same unit while maintaining computational compatibility. For instance, 'mol{creatinine}/L' and 'mol{glucose}/L' both represent molar concentration computationally, but the annotations clarify what substance is being measured. Annotations are preserved through unit operations but ignored in dimensional analysis and unit conversion. This feature bridges the gap between computational needs and domain-specific descriptive requirements.

**Subscripts**  
Subscripts in UCUM, indicated by underscore characters, are used to disambiguate unit symbols that might otherwise be confused with unit expressions or to maintain traditional notation from specific domains. For example, 'H_2O' distinguishes a symbol for water from 'H2O' which might be parsed as H times 2 times O. Subscripts are also used in certain unit definitions like 'ft_i' for international foot to distinguish different standards. The subscript mechanism ensures that traditional scientific notation can be preserved while maintaining unambiguous parsing within the UCUM grammar.

**Algebraic Terms**  
Algebraic terms in UCUM are expressions that combine multiple unit atoms using mathematical operators to represent complex units. These terms follow standard algebraic rules with modifications specific to unit arithmetic: multiplication must be explicit (using '.'), division creates fractions with specific grouping rules, and exponentiation uses integer powers only. Examples include 'kg.m/s2' for force, 'mol/L.s' for reaction rate, and 'J/mol.K' for molar heat capacity. The algebraic system allows UCUM to represent virtually any conceivable unit while maintaining mathematical rigor and parseable syntax.

**Function Symbols**  
Function symbols in UCUM are special notations used to represent units on non-ratio scales that require mathematical transformations beyond simple scaling. These include logarithmic functions like 'ln' and 'lg' for natural and common logarithms, and special functions for specific scales like pH or sound pressure levels. Function symbols indicate that the quantity involves a mathematical transformation of the underlying physical measurement. They have special rules for algebraic operations - for instance, you cannot multiply or divide logarithmic units in the usual way. Function symbols enable UCUM to handle complex measurement scales used in various scientific domains.

## Measurement Scales

**Ratio Scale**  
A ratio scale in UCUM represents measurements where ratios between values are meaningful, and there is a true zero point representing the absence of the quantity. Most physical measurements use ratio scales - for example, mass, length, and time. On ratio scales, operations like multiplication and division produce meaningful results: twice the mass is genuinely twice as much matter. Units on ratio scales can be freely combined algebraically, converted using simple multiplication factors, and used in dimensional analysis. The ratio scale property is the default for most units in UCUM and enables the full range of unit arithmetic operations.

**Interval Scale**  
An interval scale represents measurements where differences between values are meaningful, but ratios are not, due to an arbitrary zero point. The classic example is temperature in Celsius or Fahrenheit, where 0° doesn't represent the absence of temperature. On interval scales, you can meaningfully say that 20°C is 10 degrees warmer than 10°C, but not that it's "twice as hot." UCUM handles interval scales through special units that require affine transformations (involving both multiplication and addition) for conversion. Interval scale units have restricted algebraic properties - for instance, you cannot meaningfully compute °C² or kg/°C.

**Logarithmic Scale**  
Logarithmic scales in UCUM represent measurements where the quantity of interest is expressed as a logarithm of a ratio or absolute value. Common examples include pH (negative log of hydrogen ion concentration), decibels (log ratio of powers or amplitudes), and Richter magnitude (log of seismic wave amplitude). These scales compress wide ranges of values into manageable numbers and often represent human perception more accurately than linear scales. Logarithmic units have special algebraic properties - they can be added or subtracted (representing multiplication or division of the underlying quantities) but cannot be multiplied or divided in the usual sense.

## Important Rules

**Left-to-right evaluation**  
UCUM expressions are evaluated strictly from left to right, without the traditional precedence of multiplication over division found in general mathematics. This means 'a/b.c' is interpreted as '(a/b).c', not 'a/(b.c)'. This rule simplifies parsing and eliminates ambiguity but requires careful attention when constructing complex expressions. Parentheses must be used to override the default left-to-right grouping when needed. For example, to express "a per bc", you must write 'a/(b.c)' with explicit parentheses. This rule ensures that every UCUM expression has exactly one unambiguous interpretation.

**Mandatory multiplication operator**  
In UCUM, the multiplication operator (period) must always be explicit between unit symbols - concatenation without an operator is not allowed. You must write 'kg.m' not 'kgm', and 'N.m' not 'Nm'. This rule prevents ambiguity that could arise from unit symbols that might be interpreted as prefixes or combined units. The only exception is that prefixes attach directly to their units without a separator ('km' not 'k.m'). This requirement ensures clear parsing boundaries between unit components and prevents misinterpretation of unit expressions, particularly important given UCUM's extensive vocabulary of unit symbols.

**No nested prefixes**  
UCUM strictly prohibits the application of prefixes to units that already contain a prefix, preventing constructions like "kilo-kilometer" or "milli-kilogram". This rule maintains clarity and prevents confusion about the actual scale being represented. Each metric unit can have at most one prefix, applied directly to the base unit. For units that are defined with a prefix (like kilogram, the SI base unit of mass), no additional prefixes can be added. This constraint is enforced through the metric property of units - units already containing a prefix are marked as non-metric to prevent additional prefix application.

**Metric predicate**  
The metric predicate is a boolean property assigned to each unit that determines whether metric prefixes can be applied to it. Units with metric=true can be combined with any standard metric prefix (kilo-, milli-, micro-, etc.), while units with metric=false cannot accept prefixes. This property is determined by several factors: base units without existing prefixes are generally metric, units already containing a prefix are non-metric, special units on non-ratio scales are non-metric, and customary units are typically non-metric. The metric predicate is fundamental to UCUM syntax validation and ensures that only meaningful prefix-unit combinations are allowed.