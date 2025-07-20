### 4 Tables of Terminal Symbols

**§27 prefixes** {#para-27} 

* 1. Prefix symbols are those defined in [Table 1](#prefixes).  
* 2. There are five columns titled "name," "print," "c/s," "c/i," and "value" The name is the full (official) name of the unit. The official symbol used in print this is listed in the column "print" "C/s," and "c/i" list the symbol in the case sensitive and the case insensitive variants respectively. "Value" is the scalar value by which the unit atom is multiplied if combined with the prefix.  
* 3. Only the columns titled "c/s," "c/i," and "value," are normative. Full name and print symbol are defined by the CGPM and are out of scope of _The Unified Code for Units of Measure_.

The case insensitive prefix symbols are slightly different from those defined by ISO 2955 and ANSI X3.50, where "giga-," "tera-," and "peta-" have been "`G`," "`T`," and "`PE`." _The Unified Code for Units of Measure_ has a larger set of unit atoms and needs to prevent more name conflicts. Tera and giga have a second letter to be safe in the future. The change of "`PE`" to "`PT`" would be the way to go for ISO 2955 which currently has a name conflict (among others) with peta-volt and pico-electronvolt.

The new prefixes "yotta-," "zetta-," "yocto-," and "zepto-" that were adopted by the 19th CGPM (1990) have a second letter 'A' and 'O' resp. to avoid current and future conflicts and to disambiguate among themselves. The other submultiples "micro-" to "atto-" are represented by a single letter to keep with the tradition.

#### Table 1: The prefix symbols {#prefixes}

| name | print | c/s | c/i | value |
| --- | --- | --- | --- | --- |
| yotta | Y   | `Y` | `YA` | 1 × 1024 |
| zetta | Z   | `Z` | `ZA` | 1 × 1021 |
| exa | E   | `E` | `EX` | 1 × 1018 |
| peta | P   | `P` | `PT` | 1 × 1015 |
| tera | T   | `T` | `TR` | 1 × 1012 |
| giga | G   | `G` | `GA` | 1 × 109 |
| mega | M   | `M` | `MA` | 1 × 106 |
| kilo | k   | `k` | `K` | 1 × 103 |
| hecto | h   | `h` | `H` | 1 × 102 |
| deka | da  | `da` | `DA` | 1 × 101 |
| deci | d   | `d` | `D` | 1 × 10-1 |
| centi | c   | `c` | `C` | 1 × 10-2 |
| milli | m   | `m` | `M` | 1 × 10-3 |
| micro | μ   | `u` | `U` | 1 × 10-6 |
| nano | n   | `n` | `N` | 1 × 10-9 |
| pico | p   | `p` | `P` | 1 × 10-12 |
| femto | f   | `f` | `F` | 1 × 10-15 |
| atto | a   | `a` | `A` | 1 × 10-18 |
| zepto | z   | `z` | `ZO` | 1 × 10-21 |
| yocto | y   | `y` | `YO` | 1 × 10-24 |

### 4.2 Base Units

**§28 base units** {#para-28} 
* 1. The base units shown in [Table 2](#baseunits) are used to define all the unit atoms of _The Unified Code for Units of Measure_ according to its grammar and semantics.  
* 2. There are five columns titled "name," "kind of quantity," "print," "c/s," and "c/i." The name is the full (official) name of the unit. The official symbol used in print this is listed in the column "print" "C/s," and "c/i" list the symbol in the case sensitive and the case insensitive variants respectively.  
* 3. Only the columns titled "c/s," and "c/i," are normative. Full name and print symbol are defined by other bodies and are out of scope of _The Unified Code for Units of Measure_.  
* 4. The selection of base units and the particular order are not normative. Any other basis **B**' that generates an isomorphic group of units is conformant with _The Unified Code for Units of Measure_.  
* 5. If the other base **B**' generates a different system of units _U_' it conforms to _The Unified Code for Units of Measure_ only if there is an homomorphism that maps _U_' onto _U_.  
* 6. Base units must be metric units only. Special units can not be base units.

As can be seen the base system used to define _The Unified Code for Units of Measure_ is different from the system used by the _Système International d'Unités_ (SI) The SI base unit kilogram has been replaced by gram and the mole has been replaced by the radian that is defined dimensionless in the SI. Because of the latter change _The Unified Code for Units of Measure_ is not isomorphic with the SI.

The replacement of the kilogram is trivial. In order to bring syntax and semantics in line we can not have a unit with prefix in the base. We need a valid unit of mass before we can combine it with the prefix "kilo-" This change does not have any effect on the semantics whatsoever. The base unit kilogram is one of the oddities of the SI: if the gram would have been chosen as a base units the CGPM could have saved the rather annoying exception of the prefixing rules with the kilogram. At times where we have to multiply the wavelength of excited krypton-86 atoms by 1650763.73 to yield one meter, it seems trivial to divide the prototype of the kilogram by thousand to yield a base unit gram.

The rationale for removing the mole from the base is that the mole is essentially a count of particles expressed in a unit of very high magnitude (Avogadro's number). There is no fundamental difference between the count of particles and the count other things.

The radian has been adopted as the base unit of plane angle _α_ to facilitate the distinction from the solid angle _Ω_ by the relation _Ω_ = _α_2 and to distinguish rotational frequency _f_ from angular velocity _ω_ = 2 _π_ · rad · _f_.

#### Table 2: The base units upon which the semantics of all the unit atoms in The Unified Code for Units of Measure are defined. The selection of the base and the order of the units in the base are not normative. Any other base is acceptable as long as there is an isomorphism between the group of units generated by the other base system and this one. All base units are metric. {#baseunits}

| name | kind of quantity | print | c/s | c/i |
| --- | --- | --- | --- | --- |
| meter | length | m   | `m` | `M` |
| second | time | s   | `s` | `S` |
| gram | mass | g   | `g` | `G` |
| radian | plane angle | rad | `rad` | `RAD` |
| kelvin | temperature | K   | `K` | `K` |
| coulomb | electric charge | C   | `C` | `C` |
| candela | luminous intensity | cd  | `cd` | `CD` |