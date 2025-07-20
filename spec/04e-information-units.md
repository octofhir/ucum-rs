### 4.6 Prefixes and Units Used in Information Technology

**§48 units used in information technology** {#para-48} 

* 1. Units used in information technology are defined in table 22.  
* 2. The meaning of the columns is declared in [§43](#para-43).  
* 3. The function pair denoted "`ld`" is defined as the dual logarithm with its respective inverse function _f_-1(_x_) = 2x).

This table is not complete. There are other units such as shannon (Sh), erlang (E), or hartley (Hart), for which we had no quantitative definitions. Any advice is appreciated.

The bit is defined twice. One definition with a subscript letter 's' is defined as the logarithmus dualis of the number of distinct signals. However this unit can not practically be used to express more than 1000 bits. Especially when the bit is used to express transmission rate or memory capacities, floating point registers would quickly overflow. Therefore we define a second symbol for bit, without the suffix, to be the dimensionless unit 1.

The baud (Bd) is the number of distinct signals transmitted per second, it is _not_ the same as bits per second since one distinct signal usually carries more than one bit of information.

#### Table 22: Units used in Information Science and Technology {#infotech}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| bit | amount of information | bits | `bit_s` | `BIT_S` | no  | •   | `ld(1 1)` |
| bit | amount of information | bit | `bit` | `BIT` | yes | 1   | `1` |
| byte | amount of information | B   | `By` | `BY` | yes | 8   | `bit` |
| baud | signal transmission rate | Bd  | `Bd` | `BD` | yes | 1   | `/s` |

**§49 prefixes** {#para-49} 

* 1. The prefix symbols based on powers of two for use in information technology as proposed by the IEEE are defined in [Table 23](#infopfx).  
* 2. The meaning of the columns is declared in [§49](#para-49).2\.  
* 3. Only the columns titled "c/s," "c/i," and "value," are normative. Full name and print symbol are out of scope of _The Unified Code for Units of Measure_.

This table reflects proposed prefixes which are not yet standardized. [Bruce Barrow, _A Lesson in Megabytes._ IEEE Standards Bearer, January 1997]

#### Table 23: The special prefix symbols for powers of 2 {#infopfx}

| name | print | c/s | c/i | value |
| --- | --- | --- | --- | --- |
| kibi | Ki  | `Ki` | `KIB` | 1024 |
| mebi | Mi  | `Mi` | `MIB` | 1048576 |
| gibi | Gi  | `Gi` | `GIB` | 1073741824 |
| tebi | Ti  | `Ti` | `TIB` | 1099511627776 |


### 4.7 Examples for some Non-Units.

**§50 Non-units** {#para-50}

* 1. Symbols commonly used as units that are no real units of measurements are not defined by _The Unified Code for Units of Measure_.  
* 2. Users are free to use curly braces expressions ([§12](#para-12)) if they think it is important to use symbols rather than the default unit **1**.  
* 3. Curly braces expressions are equivalent to the unit **1**. The details of the annotations in the curly braces have no defined meaning in _The Unified Code for Units of Measure_.  
* 4. [Table 24](#junk) gives some example for those non-units but is not normative.

#### Table 24: Examples for Non-Units {#junk}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| particles total count | number | tot. | `{tot}` | `{TOT}` | no  |     | `1` |
| tablets | number | tbl. | `{tbl}` | `{TBL}` | no  |     | `1` |
| red blood cell count | number | R.B.C. | `{rbc}` | `{RBC}` | no  |     | `1` |
| gram meter per heartbeat | proportional to ventricular stroke work | g· m/H.B. | `g.m/{H.B.}` | `G.M/{H.B.}` | no  |     | `g.m` |
| gram-force meter per heartbeat | ventricular stroke work | gf· m/H.B. | `gf.m/{H.B.}` | `GF.M/{H.B.}` | no  |     | `gf.m` |
| kilogram of wet tissue | mass | kg(wet tissue) | `kg{wet'tis}` | `KG{wet'tis}` | no  |     | `kg` |
| milligram of creatinine | mass | mg(creat.) | `mg{creat}` | `MG{creat}` | no  |     | `mg` |

Although customarily cardiac stroke work is notated as "g.m" this is not a true unit of work. Instead one should use gram-force meter.

### Summary of Conflicts

_The Unified Code for Units of Measure_ is designed and maintained so that severe name conflicts do not occur. However, avoiding all conflicts is possible only at the cost of defining very unusual symbols for those units. As the [Table 25](#conflicts) shows, all current conflicts are of type IVa between metric and nonmetric units. This means that there is only a conflict if the metric predicate is violated so that non-metric units are used with a prefix. [Schadow G, McDonald CJ et al: Units of Measure in Clinical Information Systems. _JAMIA_ 6(2); Mar/Apr 1999. p. 151-162.]

#### Table 25: Summary of name conflicts {#conflicts}

| Gb  | G-b | Type IVa (metric-nonmetric) |
| Pa  | P-a | Type IVa (metric-nonmetric) |
| ph  | p-h | Type IVa (metric-nonmetric) |
| cd  | c-d | Type IVa (metric-nonmetric) |
| CD  | C-D | Type IVa (metric-nonmetric) |