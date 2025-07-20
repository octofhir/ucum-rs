### 4 Tables of Terminal Symbols

**§27 prefixes**      ** ■1** Prefix symbols are those defined in [Table 1](#prefixes).  ** ■2** There are five columns titled “name,” “print,” “c/s,” “c/i,” and “value” The name is the full (official) name of the unit. The official symbol used in print this is listed in the column “print” “C/s,” and “c/i” list the symbol in the case sensitive and the case insensitive variants respectively. “Value” is the scalar value by which the unit atom is multiplied if combined with the prefix.  ** ■3** Only the columns titled “c/s,” “c/i,” and “value,” are normative. Full name and print symbol are defined by the CGPM and are out of scope of _The Unified Code for Units of Measure_.

The case insensitive prefix symbols are slightly different from those defined by ISO 2955 and ANSI X3.50, where “giga-,” “tera-,” and “peta-” have been “`G`,” “`T`,” and “`PE`.” _The Unified Code for Units of Measure_ has a larger set of unit atoms and needs to prevent more name conflicts. Tera and giga have a second letter to be safe in the future. The change of “`PE`” to “`PT`” would be the way to go for ISO 2955 which currently has a name conflict (among others) with peta-volt and pico-electronvolt.

The new prefixes “yotta-,” “zetta-,” “yocto-,” and “zepto-” that were adopted by the 19th CGPM (1990) have a second letter ‘A’ and ‘O’ resp. to avoid current and future conflicts and to disambiguate among themselves. The other submultiples “micro-” to “atto-” are represented by a single letter to keep with the tradition.

#### Table 1: The prefix symbols

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

**§28 base units**      ** ■1** The base units shown in [Table 2](#baseunits) are used to define all the unit atoms of _The Unified Code for Units of Measure_ according to its grammar and semantics.  ** ■2** There are five columns titled “name,” “kind of quantity,” “print,” “c/s,” and “c/i.” The name is the full (official) name of the unit. The official symbol used in print this is listed in the column “print” “C/s,” and “c/i” list the symbol in the case sensitive and the case insensitive variants respectively.  ** ■3** Only the columns titled “c/s,” and “c/i,” are normative. Full name and print symbol are defined by other bodies and are out of scope of _The Unified Code for Units of Measure_.  ** ■4** The selection of base units and the particular order are not normative. Any other basis **B**' that generates an isomorphic group of units is conformant with _The Unified Code for Units of Measure_.  ** ■5** If the other base **B**' generates a different system of units _U_' it conforms to _The Unified Code for Units of Measure_ only if there is an homomorphism that maps _U_' onto _U_.  ** ■6** Base units must be metric units only. Special units can not be base units.

As can be seen the base system used to define _The Unified Code for Units of Measure_ is different from the system used by the _Système International d'Unités_ (SI) The SI base unit kilogram has been replaced by gram and the mole has been replaced by the radian that is defined dimensionless in the SI. Because of the latter change _The Unified Code for Units of Measure_ is not isomorphic with the SI.

The replacement of the kilogram is trivial. In order to bring syntax and semantics in line we can not have a unit with prefix in the base. We need a valid unit of mass before we can combine it with the prefix “kilo-” This change does not have any effect on the semantics whatsoever. The base unit kilogram is one of the oddities of the SI: if the gram would have been chosen as a base units the CGPM could have saved the rather annoying exception of the prefixing rules with the kilogram. At times where we have to multiply the wavelength of excited krypton-86 atoms by 1650763.73 to yield one meter, it seems trivial to divide the prototype of the kilogram by thousand to yield a base unit gram.

The rationale for removing the mole from the base is that the mole is essentially a count of particles expressed in a unit of very high magnitude (Avogadro's number). There is no fundamental difference between the count of particles and the count other things.

The radian has been adopted as the base unit of plane angle _α_ to facilitate the distinction from the solid angle _Ω_ by the relation _Ω_ = _α_2 and to distinguish rotational frequency _f_ from angular velocity _ω_ = 2 _π_ · rad · _f_.

#### Table 2: The base units upon which the semantics of all the unit atoms in The Unified Code for Units of Measure are defined. The selection of the base and the order of the units in the base are not normative. Any other base is acceptable as long as there is an isomorphism between the group of units generated by the other base system and this one. All base units are metric.

| name | kind of quantity | print | c/s | c/i |
| --- | --- | --- | --- | --- |
| meter | length | m   | `m` | `M` |
| second | time | s   | `s` | `S` |
| gram | mass | g   | `g` | `G` |
| radian | plane angle | rad | `rad` | `RAD` |
| kelvin | temperature | K   | `K` | `K` |
| coulomb | electric charge | C   | `C` | `C` |
| candela | luminous intensity | cd  | `cd` | `CD` |

### 4.3 Derived Unit Atoms

**§29 dimensionless units**      ** ■1** Dimensionless unit atoms are defined in [Table 3](#dimless).  ** ■2** There are seven columns titled “name,” “print,” “c/s,” “c/i,” “M,” “value,” and “definition.” The name is the full (official) name of the unit. The symbol recommended for use in print is listed in the column “print.” “C/s,” and “c/i” list the symbol in the case sensitive and the case insensitive variants respectively. The column “M” specifies whether this is a metric unit. The definition is a valid case sensitive expression of _The Unified Code for Units of Measure_ that defines the unit atom.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are out of scope of _The Unified Code for Units of Measure_.  ** ■4** The units named “parts per _N_” are provided to be used where absolutely necessary but are not endorsed. Especially “ppb” and “pptr” are deprecated since “billion” and “trillion” are ambiguous names internationally. The explicit powers of ten should be given instead.

#### Table 3: Dimensionless units. The units ppb and ppt are deprecated because the names “billion” and “trillion” are ambiguous. The expression “10*-9” or “10*-12” should be used instead. When the units percent or “parts per N” are used for concentrations specific units are preferred, e.g., “ug/l” for mass concentration. The expression “ug/kg” for ppb is also valid.

| name | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| the number ten for arbitrary powers | 10n | `10*` | `10*` | no  | 10  | `1` |
| the number ten for arbitrary powers | 10n | `10^` | `10^` | no  | 10  | `1` |
| the number pi | π   | `[pi]` | `[PI]` | no  | π   | `1` |
| percent | %   | `%` | `%` | no  | 1   | `10*-2` |
| parts per thousand | ppth | `[ppth]` | `[PPTH]` | no  | 1   | `10*-3` |
| parts per million | ppm | `[ppm]` | `[PPM]` | no  | 1   | `10*-6` |
| parts per billion | ppb | `[ppb]` | `[PPB]` | no  | 1   | `10*-9` |
| parts per trillion | pptr | `[pptr]` | `[PPTR]` | no  | 1   | `10*-12` |

The notation “`10*`” for powers of ten originated in the HL7 “ISO+“ extension of ISO 2955. In HL7 the character carat (‘`^`’) was thought as reserved. Since most people would expect to see “`10^3`” for the “third power of ten” and might in fact confuse “`10*3`” to mean “ten times 3”, the symbol using the carat was later added to _The Unified Code for Units of Measure_.

**§30 SI units**      ** ■1** SI units are defined by the international _Conférence Générale des Poids et Mesures_ (CGPM). _The Unified Code for Units of Measure_ definitions for those units are given in [Table 4](#si).  ** ■2** There are seven columns titled “name,” “print,” “c/s,” “c/i,” “M,” “value,” and “definition.” The name is the full (official) name of the unit. The symbol recommended for use in print is listed in the column “print.” “C/s,” and “c/i” list the symbol in the case sensitive and the case insensitive variants respectively. The column “M” specifies whether this is a metric unit. The definition is a valid case sensitive expression of _The Unified Code for Units of Measure_ that defines the unit atom.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are defined by the CGPM and are out of scope of _The Unified Code for Units of Measure_.  ** ■4** The function pair denoted “`cel(1 K)`” is defined as _f_C(_x_) = _x_ \- 273.15 to convert from kelvin to degree Celsius, and _f_C-1(_x_) = _x_ \+ 273.15 to convert from degree Celsius back to kelvin.

The case insensitive symbol for pascal is “`PAL`” which conforms to ISO 2955 and prevents the name conflict between pascal and pico-ampère.

Without reference to history, it is difficult to explain that the degree Celsius is part of the SI, because the degree Celsius is in a unique way incoherent with the SI, and is even superfluous since the base unit kelvin measures the same kind of quantity.

#### Table 4: SI units

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| mole | amount of substance | mol | `mol` | `MOL` | yes | 6.02214076 | `10*23` |
| steradian | solid angle | sr  | `sr` | `SR` | yes | 1   | `rad2` |
| hertz | frequency | Hz  | `Hz` | `HZ` | yes | 1   | `s-1` |
| newton | force | N   | `N` | `N` | yes | 1   | `kg.m/s2` |
| pascal | pressure | Pa  | `Pa` | `PAL` | yes | 1   | `N/m2` |
| joule | energy | J   | `J` | `J` | yes | 1   | `N.m` |
| watt | power | W   | `W` | `W` | yes | 1   | `J/s` |
| ampère | electric current | A   | `A` | `A` | yes | 1   | `C/s` |
| volt | electric potential | V   | `V` | `V` | yes | 1   | `J/C` |
| farad | electric capacitance | F   | `F` | `F` | yes | 1   | `C/V` |
| ohm | electric resistance | Ω   | `Ohm` | `OHM` | yes | 1   | `V/A` |
| siemens | electric conductance | S   | `S` | `SIE` | yes | 1   | `Ohm-1` |
| weber | magnetic flux | Wb  | `Wb` | `WB` | yes | 1   | `V.s` |
| degree Celsius | temperature | °C  | `Cel` | `CEL` | yes | •   | `cel(1 K)` |
| tesla | magnetic flux density | T   | `T` | `T` | yes | 1   | `Wb/m2` |
| henry | inductance | H   | `H` | `H` | yes | 1   | `Wb/A` |
| lumen | luminous flux | lm  | `lm` | `LM` | yes | 1   | `cd.sr` |
| lux | illuminance | lx  | `lx` | `LX` | yes | 1   | `lm/m2` |
| becquerel | radioactivity | Bq  | `Bq` | `BQ` | yes | 1   | `s-1` |
| gray | energy dose | Gy  | `Gy` | `GY` | yes | 1   | `J/kg` |
| sievert | dose equivalent | Sv  | `Sv` | `SV` | yes | 1   | `J/kg` |

**§31 other units from ISO 1000, ISO 2955 and ANSI X3.50**      ** ■1** Those unit atoms listed by ISO 2955 under the heading “other units from ISO 1000” and some units from ANSI X3.50 are defined in [Table 5](#iso1000).  ** ■2** The meaning of the columns is declared in [§30](#para-30).2\.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are defined by ISO 1000 and are out of scope of _The Unified Code for Units of Measure_.

#### Table 5: Other units from ISO 1000, ISO 2955, and some from ANSI X3.50.

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| gon, grade | plane angle | g   | `gon` | `GON` | no  | 0.9 | `deg` |
| degree | plane angle | °   | `deg` | `DEG` | no  | 2   | `[pi].rad/360` |
| minute | plane angle | '   | `'` | `'` | no  | 1   | `deg/60` |
| second | plane angle | ''  | `''` | `''` | no  | 1   | `'/60` |
| liter | volume | l   | `l` | `L` | yes | 1   | `dm3` |
| liter | volume | L   | `L` | `L` | yes | 1   | `l` |
| are | area | a   | `ar` | `AR` | yes | 100 | `m2` |
| minute | time | min | `min` | `MIN` | no  | 60  | `s` |
| hour | time | h   | `h` | `HR` | no  | 60  | `min` |
| day | time | d   | `d` | `D` | no  | 24  | `h` |
| tropical year | time | at  | `a_t` | `ANN_T` | no  | 365.24219 | `d` |
| mean Julian year | time | aj  | `a_j` | `ANN_J` | no  | 365.25 | `d` |
| mean Gregorian year | time | ag  | `a_g` | `ANN_G` | no  | 365.2425 | `d` |
| year | time | a   | `a` | `ANN` | no  | 1   | `a_j` |
| week | time | wk  | `wk` | `WK` | no  | 7   | `d` |
| synodal month | time | mos | `mo_s` | `MO_S` | no  | 29.53059 | `d` |
| mean Julian month | time | moj | `mo_j` | `MO_J` | no  | 1   | `a_j/12` |
| mean Gregorian month | time | mog | `mo_g` | `MO_G` | no  | 1   | `a_g/12` |
| month | time | mo  | `mo` | `MO` | no  | 1   | `mo_j` |
| tonne | mass | t   | `t` | `TNE` | yes | 1 × 103 | `kg` |
| bar | pressure | bar | `bar` | `BAR` | yes | 1 × 105 | `Pa` |
| unified atomic mass unit | mass | u   | `u` | `AMU` | yes | 1.66053906660 × 10-24 | `g` |
| electronvolt | energy | eV  | `eV` | `EV` | yes | 1   | `[e].V` |
| astronomic unit | length | AU  | `AU` | `ASU` | no  | 149597.870691 | `Mm` |
| parsec | length | pc  | `pc` | `PRS` | yes | 3.085678 × 1016 | `m` |

In the case sensitive variant the liter is defined both with an upper case ‘`L`” and a lower case ‘`l`’. NIST \[63 FR 40338\] declares the upper case ‘L’ as the preferred symbol for the U.S., while in many other countries the lower case ‘l’ is used. In fact the lower case ‘l’ was in effect since 1879. A hundred years later in 1979 the 16th CGPM decided to adopt the upper case ‘L’ as a second symbol for the liter. In the case insensitive variant there is only one symbol defined since there is no difference between upper case ‘L’ and lower case ‘l’.

The unit “are” competes with year for the symbol “a” not only in ISO 2955, and ANSI X3.50, but also in ISO 1000 as stating the official CGPM approved symbols. This is why the symbol for are is “`ar`” in _The Unified Code for Units of Measure_. ISO 2955 explicitly adds the unit atom “`ha`” for hectare, while “hectare” is just the correct spelling of the compositum of “hecto” and “are” and thus would not require a separate unit atom. Nevertheless, ISO 2955 in its case insensitive variant assigns “`ARE`” to the are and “`har`” to the hectare. This is obviously an anomaly which _The Unified Code for Units of Measure_ will not follow. As a metric unit, “`ar`” can be prefixed with “`h`” to yield “`har`”

ANSI X3.50 had two different series of symbols for the units of time, the ones from ISO 2955 as adopted by _The Unified Code for Units of Measure_ and the symbols “`yr`” “`mo`” “`wk`” “`hr`” and “`sec`” while “`d`” and “`min`” were defined twice. _The Unified Code for Units of Measure_ does not define these synonyms of ISO 2955 symbols, but does adopt those units from ANSI X3.50 that are not part of ISO 2955, namely “`mo`” and “`wk`” Month and week are useful units mainly in business or clinical medicine.

The semantics of the units of time is difficult to capture. The difficulties start with the day: There is the sidereal and the solar day that depend on the earth's rotation. The earth's rotation is variable during one day and is continually slowing down in the long run. The usual subdivisions of the day in 24 hours of 60 minutes and 60 seconds originated in Babylonia. The earth's rotation was too inexact to measure time, which is why the 11th CGPM (1954) defined the second based on a standardized historical tropical year (see below) which was later (13th CGPM 1967-1968) replaced by frequency measurement. Thus the second came to be the base unit of time and the day is now 864000 s exactly with the _Universal Coordinated Time_ (UTC) adding leap seconds every now and then.

For the year we have to distinguish the “tropical” (solar, sidereal) year from the calendar year. And both are difficult. The tropical year is the year defined by time the earth travels around the sun. This is difficult to measure and varies over time. Around 1900 it was 365.242196 d, currently it is 365.242190 d and around 2100 it will be 365.242184 d. In addition these durations are averages. The actual length of each year may vary by several minutes due to the gravitational influence of other planets. Thus there is quite a high uncertainty already in the fourth decimal digit.

The calendar year is also difficult because there is the Julian calendar (Sosigenes of Alexandria and Julius Caesar, 45 BC) with a slightly too long year of 365.25 d that causes the calendar to be one day ahead of the tropical year in 128 years. The Gregorian calendar (Christopher Clavius 1537-1612 and Pope Gregory XIII 1545-1563) leaves out three leap years in 400 years (let _n_ be the year number, the leap year is dropped if _n_ mod 100 = 0 but not _n_ mod 400 = 0.) The Gregorian mean year is thus 365.2425 d. This leap year arithmetic seems to be too much even for astronomers, which is why the light year ends up being defined based on the Julian year \[NIST Sp. Pub. 811, 1995 Edition\]. For this reason _The Unified Code for Units of Measure_ defines Tropical, Julian and Gregorian year by means of subscripts, but assigns the default year symbol to the Julian year.

The week is 7 days, this is a biblic truth we can count on (it is actually quite plausible that the week of seven days originated in Babylonia and entered Jewish tradition during the Babylonian exile.)

The difficulty continues with the month. The lunar (so called “synodal” month is variable. Around 1900 it was 29.5305886 d currently it is 29.5305889 d and in 2100 it will be 29.5305891 d, which we fixate in the 5th decimal digit with a considerable uncertainty. The calendar month is difficult because of the uneven distribution of days in a month over the year, and because of the two different calendar years. But we will usually use the mean calendar month, which is the Julian calendar year divided by 12.

As a conclusion, great care has to be taken when the “customary units” of time are used to measure time. The SI has fixated the second which should be used whenever accuracy is required. For business purposes the Julian calendar is sufficient especially since the notion of the Work-Day (vs. Holiday) is more important than the imprecision over 128 years. \[Sources: “Calendar” _Britannica Online._`http://www.eb.com:180/cgi-bin/g?DocF=macro/5000/98/toc.html`. Claus Tondering, _Frequently asked questions about calendars._ Part 1. 1998. `http://www.pip.dknet.dk/~c-t/calendar.faq1.txt`\]

**§32 natural units**      ** ■1** Fundamental constants of nature and units derived from these constants are defined in [Table 6](#const).  ** ■2** The meaning of the columns is declared in [§30](#para-30).2\.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are defined by ISO 1000 and are out of scope of _The Unified Code for Units of Measure_.

#### Table 6: Natural units.

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| velocity of light | velocity | _c_ | `[c]` | `[C]` | yes | 299792458 | `m/s` |
| Planck constant | action | _h_ | `[h]` | `[H]` | yes | 6.62607015 × 10-34 | `J.s` |
| Boltzmann constant | (unclassified) | _k_ | `[k]` | `[K]` | yes | 1.380649 × 10-23 | `J/K` |
| permittivity of vacuum | electric permittivity | _ε0_ | `[eps_0]` | `[EPS_0]` | yes | 8.854187817 × 10-12 | `F/m` |
| permeability of vacuum | magnetic permeability | _μ0_ | `[mu_0]` | `[MU_0]` | yes | 1   | `4.[pi].10*-7.N/A2` |
| elementary charge | electric charge | _e_ | `[e]` | `[E]` | yes | 1.602176634 × 10-19 | `C` |
| electron mass | mass | _me_ | `[m_e]` | `[M_E]` | yes | 9.1093837139 × 10-31 | `kg` |
| proton mass | mass | _mp_ | `[m_p]` | `[M_P]` | yes | 1.67262192595 × 10-27 | `kg` |
| Newtonian constant of gravitation | (unclassified) | _G_ | `[G]` | `[GC]` | yes | 6.67430 × 10-11 | `m3.kg-1.s-2` |
| standard acceleration of free fall | acceleration | _gn_ | `[g]` | `[G]` | yes | 9.80665 | `m/s2` |
| standard atmosphere | pressure | atm | `atm` | `ATM` | no  | 101325 | `Pa` |
| light-year | length | l.y. | `[ly]` | `[LY]` | yes | 1   | `[c].a_j` |
| gram-force | force | gf  | `gf` | `GF` | yes | 1   | `g.[g]` |
| pound force | force | lbf | `[lbf_av]` | `[LBF_AV]` | no  | 1   | `[lb_av].[g]` |

This list is not complete. It does not list all constants but only those that are fundamental and from which many other constants can be derived. The source of this table is _The NIST Reference on Constants, Units, and Uncertainty_ Version 2.1, 21 May 1998. NIST Physics Laboratory. `http://physics.nist.gov/cuu/Constants/index.html`

In the base system of _The Unified Code for Units of Measure_, the general gas constant _R_ is identical to the Boltzmann constant _k_. In the SI both are related through _R_ = _k_ × _N_A, where _N_A = 6.02214076 × 1023 /mol is the Avogadro constant. Because _The Unified Code for Units of Measure_ defines the mole to be the dimensionless Avogadro number (number of particles in 1 g of 12C itself, there is no difference anymore if the Boltzmann constant is given as _k_ = 1.380649 × 1023 J/K or _R_ = 8.314511 J mol-1 K-1.

**§33 CGS units**      ** ■1** The units of the older Centimeter-Gram-Second (CGS) system are defined in [Table 7](#cgs).  ** ■2** The meaning of the columns is declared in [§30](#para-30).2\.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are out of scope of _The Unified Code for Units of Measure_.

#### Table 7: CGS units

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Kayser | lineic number | K   | `Ky` | `KY` | yes | 1   | `cm-1` |
| Gal | acceleration | Gal | `Gal` | `GL` | yes | 1   | `cm/s2` |
| dyne | force | dyn | `dyn` | `DYN` | yes | 1   | `g.cm/s2` |
| erg | energy | erg | `erg` | `ERG` | yes | 1   | `dyn.cm` |
| Poise | dynamic viscosity | P   | `P` | `P` | yes | 1   | `dyn.s/cm2` |
| Biot | electric current | Bi  | `Bi` | `BI` | yes | 10  | `A` |
| Stokes | kinematic viscosity | St  | `St` | `ST` | yes | 1   | `cm2/s` |
| Maxwell | flux of magnetic induction | Mx  | `Mx` | `MX` | yes | 1 × 10-8 | `Wb` |
| Gauss | magnetic flux density | Gs, G | `G` | `GS` | yes | 1 × 10-4 | `T` |
| Oersted | magnetic field intensity | Oe  | `Oe` | `OE` | yes | 250 | `/[pi].A/m` |
| Gilbert | magnetic tension | Gb  | `Gb` | `GB` | yes | 1   | `Oe.cm` |
| stilb | lum. intensity density | sb  | `sb` | `SB` | yes | 1   | `cd/cm2` |
| Lambert | brightness | L   | `Lmb` | `LMB` | yes | 1   | `cd/cm2/[pi]` |
| phot | illuminance | ph  | `ph` | `PHT` | yes | 1 × 10-4 | `lx` |
| Curie | radioactivity | Ci  | `Ci` | `CI` | yes | 3.7 × 1010 | `Bq` |
| Roentgen | ion dose | R   | `R` | `ROE` | yes | 2.58 × 10-4 | `C/kg` |
| radiation absorbed dose | energy dose | RAD | `RAD` | `[RAD]` | yes | 100 | `erg/g` |
| radiation equivalent man | dose equivalent | REM | `REM` | `[REM]` | yes | 1   | `RAD` |

Although the CGPM “accepts” only very few CGS units “for use with the SI,” CGS units are proper metric units. CGS units are still used in many physiological laboratories and in clinical diagnostics (e.g., cardiology). In addition CGS units acquired a special dignity as this was the system of units used by the great physicists of the early 20th century, Albert Einstein, Max Planck, and many others who worked on the scientific revolution that had quite a cultural impact.

The CGS system defined electric and magnetic phenomena differently which is why the units named “oersted” and “maxwell” have no proper SI counterpart. This table was compiled from various sources and is not complete and not very systematic. We therefore welcome suggestions and advice as to how this table could be completed.

### 4.4 Customary Unit Atoms

Customary units have once been used all over Europe. Units were taken from nature: anatomical structures (e.g., arm, foot, finger), botanical objects (e.g., grains of various sorts, rod), or processes of everyday life (e.g., amount of land one could plow in a morning, the length of 1000 steps, an hour of walking, etc.).

Many of these units can be traced back in history to the Romans (mile), Greeks (carat) and even more ancient times. It is thus no wonder that this heritage was in some way ordered. Indeed, one finds the same names for units used in different countries and most of these units where divided into smaller or multiplied to larger units in the same way.

For example, there was the foot (de. “Fuß” fr. “pied” nl. “voet”) that was divided into 12 inches (de. “Zoll” fr. “pouce”). An inch was divided into 12 lines (de. “Linie” fr. “ligne” ). Two feet was one ell (de. “Elle” da. “Alen” sv. “Aln”). The ell was, however, not very popular in England, as opposed to the rest of Europe. Conversely, the yard is hard to find elsewhere, aside from the Argentinian “vara.” But it is perhaps no accident that the meter ended up as the 40 × 10-6 of an earth's meridian, which is approximately one yard (43.7 × 10-6). The rod (de. “Rute” fr. “perche” nl. “roede” sv. “stång”) was very popular all over Europe and so was the fathom (de. “Klafter”).

The square rod (de. “Quadratrute” fr. “perche-carrée” nl. “vierkante-roede” was mainly used to measure land. The acre as the legendary land to sow in one morning (or day) is also widespread (de. “Morgen, Tagwerk, Acker” fr. “arpent” sv. “tunnland” , although the exact amount in square rod varies considerably from region to region. Interestingly, even the special purpose measures such as the “hand” for measuring horses have international equivalents (de. “faust”).

One can indeed say that there was once a “_système international d'unités coutumières_“ but the magnitudes of the units were not standardized internationally. Of course, Great Britain had the most impact in standardizing the customary system, because of its colonies, including its most important colony, America. However, after the customary units were established in the U.S. a major reform took place through the British Weights and Measures Act of 1824. For instance, Queen Anne's wine Gallon of 231 cubic inches, still used in the U.S., was discarded then, and the older bushel was standardized differently in Great Britain. Other deviations between the English and U.S. measures are due to various alignments with the metric system. Thus, in the U.S., the yard was standardized as 3600/3937 m and the inch was 2.540005 cm while in England the inch was still 2.539998 cm.

In 1959 major parts of the U.S. and British system of customary units were standardized internationally, again aligned to the metric system which is why the international yard is 0.9144 m exactly and the nautical mile became 1852 m exactly. However, traditional subdivisions and multiples have not been abolished in favor of the international standard. Furthermore the old U.S. standard for the yard is still legally used for land surveying.

Conclusively, there are different systems of customary units that are in use today. These systems use the same names for units that have different equivalents in the metric system, because the customary systems are based on different reference quantities but multiples and subdivisions of the reference quantities are very similar, though with notable exceptions.

In the following tables we tried to give the original definitions to the customary units. This means in general that the references to the metric system are as few as possible, with most of the units of one system defined as multiples and subdivisions of one reference unit.

We use the subscript notation to disambiguate units with same names in the different systems. Subscript notation means, for instance that if the print symbol for foot is “ft” we use subscripts to distinguish the international foot “fti” the U.S. survey foot “ftus” and the British Imperial foot “ftbr” We do not actually list print symbols for customary units, because there seems to be no standard for it, and because defining print symbols is out of scope of _The Unified Code for Units of Measure_. However, we presume that subscripts be used to disambiguate whatever print symbols are being used. According to [§§13ff](#para-13), _The Unified Code for Units of Measure_ uses the underscore to denote those subscripts, and also encloses the entire unit atom into square brackets. Hence, the symbols for the international foot, the U.S. survey foot and the British Imperial foot are defined as “`[ft_i]`,” “`[ft_us]`,” and “`[ft_br]`” respectively.

Prospective users of _The Unified Code for Units of Measure_ may be disappointed by the fact that there are many different symbols for foot and inch defined but all of them have a subscript and thus none of them are equal to the ANSI X3.50 symbols. We considered to define default symbols for customary units, where, e.g., the common units of length (foot, inch) would default to the international customary units, while mass units (pound, ounce) would default to the avoirdupois system. However, because the customary system is quite complex, and units by the same names can differ by more than 20%, defining defaults will probably cause even more confusion. There is no denial: a gallon is not just a gallon and a pound is not just a pound, this is the disadvantage of dealing with a unit system of medieval origin.

**§34 international customary units**      ** ■1** The unified U.S. and British Imperial customary units, so called “international” customary units are defined in [Table 8](#intcust).  ** ■2** The meaning of the columns is declared in [§30](#para-30).2\. With the exception that the column named “print” is not available.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. The full name is out of scope of _The Unified Code for Units of Measure_.  ** ■4** The special symbols for “square inch,” “cubic foot,” etc. are deprecated. The preferred expressions use the exponents 2 and 3 respectively as shown in the column “definition”

#### Table 8: International customary units

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| inch | length | `[in_i]` | `[IN_I]` | no  | 2.54 | `cm` |
| foot | length | `[ft_i]` | `[FT_I]` | no  | 12  | `[in_i]` |
| yard | length | `[yd_i]` | `[YD_I]` | no  | 3   | `[ft_i]` |
| mile | length | `[mi_i]` | `[MI_I]` | no  | 5280 | `[ft_i]` |
| fathom | depth of water | `[fth_i]` | `[FTH_I]` | no  | 6   | `[ft_i]` |
| nautical mile | length | `[nmi_i]` | `[NMI_I]` | no  | 1852 | `m` |
| knot | velocity | `[kn_i]` | `[KN_I]` | no  | 1   | `[nmi_i]/h` |
| square inch | area | `[sin_i]` | `[SIN_I]` | no  | 1   | `[in_i]2` |
| square foot | area | `[sft_i]` | `[SFT_I]` | no  | 1   | `[ft_i]2` |
| square yard | area | `[syd_i]` | `[SYD_I]` | no  | 1   | `[yd_i]2` |
| cubic inch | volume | `[cin_i]` | `[CIN_I]` | no  | 1   | `[in_i]3` |
| cubic foot | volume | `[cft_i]` | `[CFT_I]` | no  | 1   | `[ft_i]3` |
| cubic yard | volume | `[cyd_i]` | `[CYD_I]` | no  | 1   | `[yd_i]3` |
| board foot | volume | `[bf_i]` | `[BF_I]` | no  | 144 | `[in_i]3` |
| cord | volume | `[cr_i]` | `[CR_I]` | no  | 128 | `[ft_i]3` |
| mil | length | `[mil_i]` | `[MIL_I]` | no  | 1 × 10-3 | `[in_i]` |
| circular mil | area | `[cml_i]` | `[CML_I]` | no  | 1   | `[pi]/4.[mil_i]2` |
| hand | height of horses | `[hd_i]` | `[HD_I]` | no  | 4   | `[in_i]` |

In general the international customary units are effective in the U.S. and in Great Britain since 1959. We are unsure, however, about this in countries that formerly or at present belong to the Commonwealth. We therefore appreciate advice and reference to original sources on this transition. Conceivably other countries may have made exceptions in the transition to the international definitions of customary units, such as the U.S. where the old definitions have been retained for the purpose of land surveying.

It is not quite clear exactly what units the international customary system comprises. According to the Encyclopedia Britannica \[_British Imperial System. Britannica Online_\], the rod was removed in Great Britain in 1963. Since the definition of the acre is based on the rod, we did not include rod and acre in the international customary system. In the U.S. the acre is still defined on the older U.S. customary system as of 1893.

In general, we did not include special customary units of area and volume in [Table 8](#intcust), since these are still used differently in the U.S. Special symbols such as square inch and cubic foot have been included according to ANSI X3.50. Generally the “square-” and “cubic-” prefixes are unnecessary in ISO 2955 and ANSI X3.50 and are deprecated by _The Unified Code for Units of Measure_. We placed the board foot, cord and circular mil into the international table because these units are suggested by ANSI X3.50 but we were not sure in what sense they are still used. We did, however, not include the square mile in the international table because in the U.S. measurements in square miles are most likely based on the survey mile that is part of the older system, see [§35](#para-35).

The circular mil is exactly the area of a circle with a diameter of one mil. One mil, in turn, equals 1/1000 inch (“mil” is the etymological equivalent of “milli-inch” ) The mil has been defined in [Table 8](#intcust) to support the exact definition of the circular mil.

ANSI X3.50 does not define a symbol for the “hand,” but this unit is mentioned in the table given by the Encyclopedia Britannica. The hand is used in measuring the height of horses from foot to shoulder. It was probably not subject to the internationalization of customary units. Any advice as whether the hand is used based on an older British or U.S. definition is appreciated.

**§35 U.S. survey lengths**      ** ■1** The older U.S. units according to the definition of the inch in the U.S. Metric Law of 1866 and the definition of foot and yard that was valid from 1893 until 1959.  ** ■2** The meaning of the columns is declared in [§34](#para-34). \[Barry N. Taylor, _Guide to the Use of the International System of Units (SI)_ \[NIST Special Publication 811\], National Institute for Standards and Technology (NIST), 1995. Available from: URL: `http://physics.nist.gov/Document/sp811.pdf`\]

#### Table 9: Older U.S. “survey” lengths (also called "statute" lengths)

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| foot | length | `[ft_us]` | `[FT_US]` | no  | 1200 | `m/3937` |
| yard | length | `[yd_us]` | `[YD_US]` | no  | 3   | `[ft_us]` |
| inch | length | `[in_us]` | `[IN_US]` | no  | 1   | `[ft_us]/12` |
| rod | length | `[rd_us]` | `[RD_US]` | no  | 16.5 | `[ft_us]` |
| Gunter's chain, Surveyor's chain | length | `[ch_us]` | `[CH_US]` | no  | 4   | `[rd_us]` |
| link for Gunter's chain | length | `[lk_us]` | `[LK_US]` | no  | 1   | `[ch_us]/100` |
| Ramden's chain, Engineer's chain | length | `[rch_us]` | `[RCH_US]` | no  | 100 | `[ft_us]` |
| link for Ramden's chain | length | `[rlk_us]` | `[RLK_US]` | no  | 1   | `[rch_us]/100` |
| fathom | length | `[fth_us]` | `[FTH_US]` | no  | 6   | `[ft_us]` |
| furlong | length | `[fur_us]` | `[FUR_US]` | no  | 40  | `[rd_us]` |
| mile | length | `[mi_us]` | `[MI_US]` | no  | 8   | `[fur_us]` |
| acre | area | `[acr_us]` | `[ACR_US]` | no  | 160 | `[rd_us]2` |
| square rod | area | `[srd_us]` | `[SRD_US]` | no  | 1   | `[rd_us]2` |
| square mile | area | `[smi_us]` | `[SMI_US]` | no  | 1   | `[mi_us]2` |
| section | area | `[sct]` | `[SCT]` | no  | 1   | `[mi_us]2` |
| township | area | `[twp]` | `[TWP]` | no  | 36  | `[sct]` |
| mil | length | `[mil_us]` | `[MIL_US]` | no  | 1 × 10-3 | `[in_us]` |

After the 1959 international agreement changed the definition of the yard in the US to be 0.9144 m exactly, surveyors and civil engineers complained that voluminous legacy surveys and so forth used the previous definition of (1200/3937) m and that this change would be disruptive. So, by statute, Congress created a survey foot of (1200/3937) m (the old 1893 Mendenhall Order definition). Thus, by statute, miles used in surveying are referred to as statute miles of 5280 survey feet each. The fathom, rod, and furlong are likewise based on the survey foot.

According to NIST, the acre as normally used in the U.S. is defined in terms of U.S. survey lengths, and not in terms of the international customary system. This older U.S. customary system of survey lengths is still used for geodesic measurements.

**§36 British Imperial lengths**      ** ■1**[Table 10](#brit-length) defines symbols for the older British Imperial lengths as of the British Weights and Measures Act of 1824.  ** ■2** The meaning of the columns is declared in [§34](#para-34).

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |Table 10: British Imperial lengths
| inch | length | `[in_br]` | `[IN_BR]` | no  | 2.539998 | `cm` |
| foot | length | `[ft_br]` | `[FT_BR]` | no  | 12  | `[in_br]` |
| rod | length | `[rd_br]` | `[RD_BR]` | no  | 16.5 | `[ft_br]` |
| Gunter's chain | length | `[ch_br]` | `[CH_BR]` | no  | 4   | `[rd_br]` |
| link for Gunter's chain | length | `[lk_br]` | `[LK_BR]` | no  | 1   | `[ch_br]/100` |
| fathom | length | `[fth_br]` | `[FTH_BR]` | no  | 6   | `[ft_br]` |
| pace | length | `[pc_br]` | `[PC_BR]` | no  | 2.5 | `[ft_br]` |
| yard | length | `[yd_br]` | `[YD_BR]` | no  | 3   | `[ft_br]` |
| mile | length | `[mi_br]` | `[MI_BR]` | no  | 5280 | `[ft_br]` |
| nautical mile | length | `[nmi_br]` | `[NMI_BR]` | no  | 6080 | `[ft_br]` |
| knot | velocity | `[kn_br]` | `[KN_BR]` | no  | 1   | `[nmi_br]/h` |
| acre | area | `[acr_br]` | `[ACR_BR]` | no  | 4840 | `[yd_br]2` |

The older British Imperial system is predominantly of historical interest. However, it may be that some former members of the Commonwealth have retained this system after 1959, when the unified international definitions where established, and after 1963, when the British system was revised in England.

The chain was proposed by Edmund Gunter in England of the 17th century. It is possible that Gunter's chain and Ramden's chain are related to other European traditional units such as the English “rope” (measuring 20 feet) or the old German “Landseil” (measuring 52 ells or 104 feet) named after ropes or chains that could be spanned in order to measure land. The difference in the definitions of those units is no surprise as there is nothing that restricts a chain or rope to a particular length. However, these units are still similar in magnitude.

**§37 U.S. volumes**      ** ■1** The U.S. volumes, so called “capacity” measures, which are different for fluid goods (wine) and dry goods (grain), are defined in [Table 11](#us-volumes).  ** ■2** The meaning of the columns is declared in [§34](#para-34).

#### Table 11: U.S. volumes including so called “dry measures”

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| Queen Anne's wine gallon | fluid volume | `[gal_us]` | `[GAL_US]` | no  | 231 | `[in_i]3` |
| barrel | fluid volume | `[bbl_us]` | `[BBL_US]` | no  | 42  | `[gal_us]` |
| quart | fluid volume | `[qt_us]` | `[QT_US]` | no  | 1   | `[gal_us]/4` |
| pint | fluid volume | `[pt_us]` | `[PT_US]` | no  | 1   | `[qt_us]/2` |
| gill | fluid volume | `[gil_us]` | `[GIL_US]` | no  | 1   | `[pt_us]/4` |
| fluid ounce | fluid volume | `[foz_us]` | `[FOZ_US]` | no  | 1   | `[gil_us]/4` |
| fluid dram | fluid volume | `[fdr_us]` | `[FDR_US]` | no  | 1   | `[foz_us]/8` |
| minim | fluid volume | `[min_us]` | `[MIN_US]` | no  | 1   | `[fdr_us]/60` |
| cord | fluid volume | `[crd_us]` | `[CRD_US]` | no  | 128 | `[ft_i]3` |
| bushel | dry volume | `[bu_us]` | `[BU_US]` | no  | 2150.42 | `[in_i]3` |
| historical winchester gallon | dry volume | `[gal_wi]` | `[GAL_WI]` | no  | 1   | `[bu_us]/8` |
| peck | dry volume | `[pk_us]` | `[PK_US]` | no  | 1   | `[bu_us]/4` |
| dry quart | dry volume | `[dqt_us]` | `[DQT_US]` | no  | 1   | `[pk_us]/8` |
| dry pint | dry volume | `[dpt_us]` | `[DPT_US]` | no  | 1   | `[dqt_us]/2` |
| tablespoon | volume | `[tbs_us]` | `[TBS_US]` | no  | 1   | `[foz_us]/2` |
| teaspoon | volume | `[tsp_us]` | `[TSP_US]` | no  | 1   | `[tbs_us]/3` |
| cup | volume | `[cup_us]` | `[CUP_US]` | no  | 16  | `[tbs_us]` |
| metric fluid ounce | fluid volume | `[foz_m]` | `[FOZ_M]` | no  | 30  | `mL` |
| metric cup | volume | `[cup_m]` | `[CUP_M]` | no  | 240 | `mL` |
| metric teaspoon | volume | `[tsp_m]` | `[TSP_M]` | no  | 5   | `mL` |
| metric tablespoon | volume | `[tbs_m]` | `[TBS_M]` | no  | 15  | `mL` |

The U.S. fluid volumes have been defined based on Queen Anne's wine gallon which was in turn defined exactly as 231 cubic inch. Although we used international inch, we are not sure what inch definition is actually used for defining the exact size of a U.S. gallon. However, the differences between the various inches are minimal, even when raised to the 3rd power (i.e., the difference between the U.S. inch and the British Imperial inch remains in the sixth decimal digit.)

Dry measures are based on the bushel (corn bushel), originally defined in 1701 as “any round measure with a plain and even bottom, being 18.5 inches wide throughout and 8 inches deep.” This definition, being (18.5/2)2 π × 8 = 2150.42017138221... cubic inch was later truncated to 2150.42 cubic inch exactly. At times the bushel was closely related with the Winchester gallon (corn gallon), which has been mentioned as an historical curiosity.

ANSI X3.50 defines symbols for the units cup, tablespoon and teaspoon which are predominantly used in cooking recipes but also in practical medicine. Similar units can often be found in European cook books, but are usually translated into metric units outside the U.S. For practical medicine these are still very handy units to give instructions to patients.

**§38 British Imperial volumes**      ** ■1** British Imperial volumes according to the Weights and Measures Act of 1824 are defined in [Table 12](#brit-volumes).  ** ■2** The meaning of the columns is declared in [§34](#para-34).

#### Table 12: British Imperial volumes

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| gallon | volume | `[gal_br]` | `[GAL_BR]` | no  | 4.54609 | `l` |
| peck | volume | `[pk_br]` | `[PK_BR]` | no  | 2   | `[gal_br]` |
| bushel | volume | `[bu_br]` | `[BU_BR]` | no  | 4   | `[pk_br]` |
| quart | volume | `[qt_br]` | `[QT_BR]` | no  | 1   | `[gal_br]/4` |
| pint | volume | `[pt_br]` | `[PT_BR]` | no  | 1   | `[qt_br]/2` |
| gill | volume | `[gil_br]` | `[GIL_BR]` | no  | 1   | `[pt_br]/4` |
| fluid ounce | volume | `[foz_br]` | `[FOZ_BR]` | no  | 1   | `[gil_br]/5` |
| fluid dram | volume | `[fdr_br]` | `[FDR_BR]` | no  | 1   | `[foz_br]/8` |
| minim | volume | `[min_br]` | `[MIN_BR]` | no  | 1   | `[fdr_br]/60` |

The British Weights and Measures Act of 1824 removed the medieval distinction between wine and grain measures and defined one unified system of volumes based on a new Gallon that was defined similarly as the metric unit liter: “10 imperial pounds weight of distilled water weighed in air against brass weights with the water and the air at a temperature of 62 degrees of Fahrenheit's thermometer and with the barometer at 30 inches.”

With the current definition of the gallon as 277.421 cubic inches (approximately) and a density of water of 0.99878 kg/l according to NIST data, the inch must have been approximately 2.5371 cm at that time. Because of this difficulty with the original definition of the British gallon we based the British Imperial volumes on the gallon for which there is an exact metric equivalence, according to NIST, which provides usually well researched data.

Note that the subdivisions of the British Imperial system of volumes differs from the U.S. system of fluid volumes between gill and fluid ounce: in the British system 1 oz fl equals 1/5 gill where in the U.S. system 1 oz fl equals 1/4 gill. Thus, although the British system starts out with a 20% larger gallon, the British fluid ounce, fluid dram and minim are 4% smaller than the U.S. units with the same name.

**§39 avoirdupois weights**      ** ■1** The avoirdupois system of mass units is defined in [Table 13](#avoirdupois).  ** ■2** The meaning of the columns is declared in [§34](#para-34).

The avoirdupois system is used in the U.S. as well as in countries that use the British Imperial system. Avoirdupois is the default system of mass units used for all goods that “have weight” (fr. _avoir du poids_). Interestingly all three systems of weight are based on the same grain of barley, standardized to 64.79891 mg exactly \[NIST\].

#### Table 13: Avoirdupois weights

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| grain | mass | `[gr]` | `[GR]` | no  | 64.79891 | `mg` |
| pound | mass | `[lb_av]` | `[LB_AV]` | no  | 7000 | `[gr]` |
| ounce | mass | `[oz_av]` | `[OZ_AV]` | no  | 1   | `[lb_av]/16` |
| dram | mass | `[dr_av]` | `[DR_AV]` | no  | 1   | `[oz_av]/16` |
| short hundredweight, U.S. hundredweight | mass | `[scwt_av]` | `[SCWT_AV]` | no  | 100 | `[lb_av]` |
| long hundredweight, British hundredweight | mass | `[lcwt_av]` | `[LCWT_AV]` | no  | 112 | `[lb_av]` |
| short ton, U.S. ton | mass | `[ston_av]` | `[STON_AV]` | no  | 20  | `[scwt_av]` |
| long ton, British ton | mass | `[lton_av]` | `[LTON_AV]` | no  | 20  | `[lcwt_av]` |
| stone, British stone | mass | `[stone_av]` | `[STONE_AV]` | no  | 14  | `[lb_av]` |

**§40 troy weights**      ** ■1** The troy system of mass units is defined in [Table 14](#troy).  ** ■2** The meaning of the columns is declared in [§34](#para-34).

The troy system originates in Troyes, a City in the Champagne (France) that hosted a major European fair. The troy system was later used for measuring precious metals. The World Monetary Fund valued all currencies against the troy ounce of gold at least until the 1960s (advice appreciated). The troy ounce is still used in worldwide trade with gold, even in countries that otherwise use metric units (de. “feinunze”). The troy system retains the original Roman subdivision of the pound in 12 ounces. The Roman _uncia_ was “one twelfth” of a _libra_ (hence the symbol “lb” for the pound), just as the inch (also originating from la. “libra” is one twelfth of a foot. The subdivision of 12 ounces/inches per pound/foot and 2 foot per ell (la. “cubit” apparently originated in the ancient Egypt and was carried on by the Greeks and Romans into the medieval Europe. However, there was always an ambiguity such that the subdivision of 1/12 could become 1/16 and vice versa, hence the avoirdupois ounce of 1/16 pound.

Note also that the troy pound was abolished in England on January 6, 1879 \[Jacques J. Proot, _Anglo-Saxon weights & measures_, URL: `http://members.aol.com/JackProot/met/spvolas.html`\].

#### Table 14: Troy weights 

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| pennyweight | mass | `[pwt_tr]` | `[PWT_TR]` | no  | 24  | `[gr]` |
| ounce | mass | `[oz_tr]` | `[OZ_TR]` | no  | 20  | `[pwt_tr]` |
| pound | mass | `[lb_tr]` | `[LB_TR]` | no  | 12  | `[oz_tr]` |

**§41 apothecaries' weights.**      ** ■1** The apothecaries' system of mass units is defined in [Table 15](#apoth).  ** ■2** The meaning of the columns is declared in [§34](#para-34).

#### Table 15: Apothecaries' weights

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| scruple | mass | `[sc_ap]` | `[SC_AP]` | no  | 20  | `[gr]` |
| dram, drachm | mass | `[dr_ap]` | `[DR_AP]` | no  | 3   | `[sc_ap]` |
| ounce | mass | `[oz_ap]` | `[OZ_AP]` | no  | 8   | `[dr_ap]` |
| pound | mass | `[lb_ap]` | `[LB_AP]` | no  | 12  | `[oz_ap]` |
| metric ounce | mass | `[oz_m]` | `[OZ_M]` | no  | 28  | `g` |

Note that some U.S. pharmacies still use this system of apothecaries' weights when measuring the amount of drugs. This system is very different from the avoirdupois system though based on the same grain. The apothecaries' dram is more than twice as much as the avoirdupois dram, the ounce is still 10% greater than the avoirdupois ounce while the pound is 20% less than the avoirdupois pound. The apothecaries' system, just as the troy system, keeps the original Roman subdivision of an ounce (la. “uncia” to be 1/12 pound (la. “libra”). Hence is the apothecaries' pound about 22% smaller than the avoirdupois pound, while its subdivisions are greater than the respective avoirdupois subdivisions (ounce 10%, dram 119%). This difference in the weight systems is the most important reason why ANSI X3.50 should not be applied in medicine, where both systems are being used and therefore misinterpretations are inevitable.

**§42 typesetter's lengths**      ** ■1** The units of length as used in typesetting are defined in [Table 16](#typeset).  ** ■2** The meaning of the columns is declared in [§34](#para-34).

#### Table 16: Units used in typesetting

| name | kind of quantity | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| line | length | `[lne]` | `[LNE]` | no  | 1   | `[in_i]/12` |
| point | length | `[pnt]` | `[PNT]` | no  | 1   | `[lne]/6` |
| pica | length | `[pca]` | `[PCA]` | no  | 12  | `[pnt]` |
| Printer's point | length | `[pnt_pr]` | `[PNT_PR]` | no  | 0.013837 | `[in_i]` |
| Printer's pica | length | `[pca_pr]` | `[PCA_PR]` | no  | 12  | `[pnt_pr]` |
| pied, French foot | length | `[pied]` | `[PIED]` | no  | 32.48 | `cm` |
| pouce, French inch | length | `[pouce]` | `[POUCE]` | no  | 1   | `[pied]/12` |
| ligne, French line | length | `[ligne]` | `[LIGNE]` | no  | 1   | `[pouce]/12` |
| didot, Didot's point | length | `[didot]` | `[DIDOT]` | no  | 1   | `[ligne]/6` |
| cicero, Didot's pica | length | `[cicero]` | `[CICERO]` | no  | 12  | `[didot]` |

There are three systems of typesetter's lengths in use today: Françcois-Ambroise Didot (1730-1804), a publisher in Paris, invented this system based on the traditional subdivisions of the customary units: 1 line was 1/12 inch and 1/6 line was one point. Henceforth the size of letters were measured in point. However, the Didot system is based on the _pouce_, i.e. the french inch, which, just as the English inch, is 1/12 _pied_ (foot). But the French foot was about 6.5% greater than the British Imperial foot. In the Anglo-American realm the typesetter's point was based on the British Imperial inch, with the same subdivisions. However, in the type foundries' industry the original definition of a point drifted apart, and in the late 19th century U.S. type foundries reestablished a slightly (0.375%) greater standard point. This point made its way back to the British. However, recently, the computer typesetting industry readjusted the point to its original size of 1/72 inch. All three systems, however, are still being used today.

### 4.5 Other Legacy Units

**§43 legacy units for heat and temperature**      ** ■1** Older units of heat (energy) and temperature are defined in [Table 17](#heat).  ** ■2** The meaning of the columns is declared in [§30](#para-30).2\.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are either not standardized or standardized by other bodies and are out of scope of _The Unified Code for Units of Measure_.  ** ■4** The function pair denoted “`degf(5 K/9)`” is defined as _f_F(_x_) = 9/5 _x_ \- 459.67 to convert from kelvin to degree Fahrenheit, and _f_F-1(_x_) = 5/9 (_x_ \+ 459.67) to convert from degree Fahrenheit back to kelvin.  ** ■5** The function pair denoted “`degre(5 K/4)`” is defined as _f_Ré(_x_) = 4/5 _x_ \- 218.52 to convert from kelvin to degree Réaumur, and _f_Ré-1(_x_) = 5/4 (_x_ \+ 218.52) to convert from degree Réaumur back to kelvin.

#### Table 17: Other Units for Heat and Temperature

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| degree Fahrenheit | temperature | °F  | `[degF]` | `[DEGF]` | no  | •   | `degf(5 K/9)` |
| degree Rankine | temperature | °R  | `[degR]` | `[degR]` | no  | 5   | `K/9` |
| degree Réaumur | temperature | °Ré | `[degRe]` | `[degRe]` | no  | •   | `degre(5 K/4)` |
| calorie at 15 °C | energy | cal15°C | `cal_[15]` | `CAL_[15]` | yes | 4.18580 | `J` |
| calorie at 20 °C | energy | cal20°C | `cal_[20]` | `CAL_[20]` | yes | 4.18190 | `J` |
| mean calorie | energy | calm | `cal_m` | `CAL_M` | yes | 4.19002 | `J` |
| international table calorie | energy | calIT | `cal_IT` | `CAL_IT` | yes | 4.1868 | `J` |
| thermochemical calorie | energy | calth | `cal_th` | `CAL_TH` | yes | 4.184 | `J` |
| calorie | energy | cal | `cal` | `CAL` | yes | 1   | `cal_th` |
| nutrition label Calories | energy | Cal | `[Cal]` | `[CAL]` | no  | 1   | `kcal_th` |
| British thermal unit at 39 °F | energy | Btu39°F | `[Btu_39]` | `[BTU_39]` | no  | 1.05967 | `kJ` |
| British thermal unit at 59 °F | energy | Btu59°F | `[Btu_59]` | `[BTU_59]` | no  | 1.05480 | `kJ` |
| British thermal unit at 60 °F | energy | Btu60°F | `[Btu_60]` | `[BTU_60]` | no  | 1.05468 | `kJ` |
| mean British thermal unit | energy | Btum | `[Btu_m]` | `[BTU_M]` | no  | 1.05587 | `kJ` |
| international table British thermal unit | energy | BtuIT | `[Btu_IT]` | `[BTU_IT]` | no  | 1.05505585262 | `kJ` |
| thermochemical British thermal unit | energy | Btuth | `[Btu_th]` | `[BTU_TH]` | no  | 1.054350 | `kJ` |
| British thermal unit | energy | btu | `[Btu]` | `[BTU]` | no  | 1   | `[Btu_th]` |
| horsepower | power |     | `[HP]` | `[HP]` | no  | 550 | `[ft_i].[lbf_av]/s` |
| tex | linear mass density (of textile thread) | tex | `tex` | `TEX` | yes | 1   | `g/km` |
| Denier | linear mass density (of textile thread) | den | `[den]` | `[DEN]` | no  | 1   | `g/9/km` |

The degree Fahrenheit was missing in ANSI X3.50. HL7's “ISO+/ANS+” code defined the degree Fahrenheit under the symbol “`DEGF`” which is reflected here. This is the reason why _The Unified Code for Units of Measure_ does not define a new symbol “`Fah`” similar to “`Cel`” of ISO 2955 for the degree Celsius.

Defining precise semantics for legacy units for “quantity of heat” is difficult. The many variants of these units are frequently confused because there is not just a calorie and not just a British thermal unit. The different calories usually being used vary by 1% but the confusion can result in an error as high as 100000%! Thus, if exactness and non-ambiguity is important one should use the joule to report amounts of heat, just like for any other energy and work kind-of-quantities.

The gram-calorie, sometimes called “small calorie” is defined as the amount of heat required to raise the temperature of 1 gram of Water from 14.5 °C to 15.5 °C. According to _Encyclopedia Britannica_, this is the calorie most often used in engineering. There is also a less frequently used gram-calorie at 19.5 °C to 20.5 °C and a mean calorie that is 1/100 of the amount of heat required to raise the temperature from 0 °C to 100 °C. The _International Table_ calorie is defined by the _International Conference on the Properties of Steam_ (1956) and is used in steam engineering. In chemistry a “thermochemical” calorie is used for reaction enthalpies.

To complete the confusion, there is also a kilogram-calorie (“large calorie” , that has a similar definition based on a kilogram instead of a gram of water. This kilocalorie has also been called “calorie” in the sloppy speech of everyday life about food. U.S. “Nutrition Facts” that label almost every American food say “Calories: xxx” The _International Union of Nutritional Sciences_ recommends using either the joule or a kilocalorie based on the thermochemical calorie. Because of a perceived popular demand _The Unified Code for Units of Measure_ defines the nutrition Calorie as “`Cal`” with the conventional capital first letter. For the case insensitive variant of _The Unified Code for Units of Measure_, the symbol is enclosed in square brackets (“`[CAL]`”).

Only the International Table calorie and the thermochemical calorie have exact definitions. To give some guidance in the confusing plenty of different calories, _The Unified Code for Units of Measure_ defines a default symbol “`cal`” as an alias for the thermochemical calorie, because the calorie is mostly used today in medicine and biochemistry. On the other hand, we consider engineers smart enough to select the precise calorie they mean.

Similar to the calories, various “British Thermal Unit” (Btu) are defined and the confusion continues. One Btu is defined as the amount of heat necessary to raise the temperature of one avoirdupois pound of water by one degree Fahrenheit beginning from various temperatures (39 °F, 59 °F, or 60 °F). There is also the International Table Btu and the thermochemical Btu. Just as with the calorie we define a default symbol “`Btu`” as an alias for the thermochemical Btu.

**§44 units used predominantly in clinical medicine**      ** ■1** Units used mainly in clinical medicine are defined in [Table 18](#clinical).  ** ■2** The meaning of the columns is declared in [§34](#para-34).  ** ■3** The function pair denoted “`hpX(1 l)`” is defined as _f_hp X(_x_) = \- lg _x_ to convert from a number fraction (dilution) per liter to the homeopathic potency value of the decimal (X) series, and _f_hp X-1(_x_) = 10-_x_ to convert from the potency value back to the number fraction. Likewise, the function pair denoted “`hpC(1 l)`” is defined as _f_hp C(_x_) = \- ln(_x_) / ln(100) to convert from a number fraction (dilution) per liter to the homeopathic potency value of the centesimal (C) series, and _f_hp C-1(_x_) = 100-_x_ to convert from the potency value back to the number fraction. Analogous functions are defined for the millesimal (M) series with basis 1,000 and the series and the quintamillesimal (Q) series with basis 50,000.  ** ■4** The function pair denoted “`100tan(1 rad)`” is defined as _f_PD(_α_) = tan(_α_) × 100 to convert from a plane angle _α_ to a prism diopter value (or a slope percent value) and _f_PD-1(_x_) = arctan(_x / 100_) to convert from prism diopter (or slope percent) value _x_ back to a plane angle.

#### Table 18: Units Used Predominantly in Clinical Medicine

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| meter of water column | pressure | m H2O | `m[H2O]` | `M[H2O]` | yes | 9.80665 | `kPa` |
| meter of mercury column | pressure | m Hg | `m[Hg]` | `M[HG]` | yes | 133.3220 | `kPa` |
| inch of water column | pressure | in H2O | `[in_i'H2O]` | `[IN_I'H2O]` | no  | 1   | `m[H2O].[in_i]/m` |
| inch of mercury column | pressure | in Hg | `[in_i'Hg]` | `[IN_I'HG]` | no  | 1   | `m[Hg].[in_i]/m` |
| peripheral vascular resistance unit | fluid resistance | P.R.U. | `[PRU]` | `[PRU]` | no  | 1   | `mm[Hg].s/ml` |
| Wood unit | fluid resistance | Wood U. | `[wood'U]` | `[WOOD'U]` | no  | 1   | `mm[Hg].min/L` |
| diopter | refraction of a lens | dpt | `[diop]` | `[DIOP]` | no  | 1   | `/m` |
| prism diopter | refraction of a prism | PD  | `[p'diop]` | `[P'DIOP]` | no  | •   | `100tan(1 rad)` |
| percent of slope | slope | %   | `%[slope]` | `%[SLOPE]` | no  | •   | `100tan(1 rad)` |
| mesh | lineic number |     | `[mesh_i]` | `[MESH_I]` | no  | 1   | `/[in_i]` |
| Charrière, french | gauge of catheters | Ch  | `[Ch]` | `[CH]` | no  | 1   | `mm/3` |
| drop | volume | drp | `[drp]` | `[DRP]` | no  | 1   | `ml/20` |
| Hounsfield unit | x-ray attenuation | HF  | `[hnsf'U]` | `[HNSF'U]` | no  | 1   | `1` |
| metabolic equivalent | metabolic cost of physical activity | MET | `[MET]` | `[MET]` | no  | 3.5 | `mL/min/kg` |
| homeopathic potency of decimal series (retired) | homeopathic potency (retired) | X   | `[hp'_X]` | `[HP'_X]` | no  | •   | `hpX(1 1)` |
| homeopathic potency of centesimal series (retired) | homeopathic potency (retired) | C   | `[hp'_C]` | `[HP'_C]` | no  | •   | `hpC(1 1)` |
| homeopathic potency of millesimal series (retired) | homeopathic potency (retired) | M   | `[hp'_M]` | `[HP'_M]` | no  | •   | `hpM(1 1)` |
| homeopathic potency of quintamillesimal series (retired) | homeopathic potency (retired) | Q   | `[hp'_Q]` | `[HP'_Q]` | no  | •   | `hpQ(1 1)` |
| homeopathic potency of decimal hahnemannian series | homeopathic potency (Hahnemann) | X   | `[hp_X]` | `[HP_X]` | no  | •   | •   |
| homeopathic potency of centesimal hahnemannian series | homeopathic potency (Hahnemann) | C   | `[hp_C]` | `[HP_C]` | no  | •   | •   |
| homeopathic potency of millesimal hahnemannian series | homeopathic potency (Hahnemann) | M   | `[hp_M]` | `[HP_M]` | no  | •   | •   |
| homeopathic potency of quintamillesimal hahnemannian series | homeopathic potency (Hahnemann) | Q   | `[hp_Q]` | `[HP_Q]` | no  | •   | •   |
| homeopathic potency of decimal korsakovian series | homeopathic potency (Korsakov) | X   | `[kp_X]` | `[KP_X]` | no  | •   | •   |
| homeopathic potency of centesimal korsakovian series | homeopathic potency (Korsakov) | C   | `[kp_C]` | `[KP_C]` | no  | •   | •   |
| homeopathic potency of millesimal korsakovian series | homeopathic potency (Korsakov) | M   | `[kp_M]` | `[KP_M]` | no  | •   | •   |
| homeopathic potency of quintamillesimal korsakovian series | homeopathic potency (Korsakov) | Q   | `[kp_Q]` | `[KP_Q]` | no  | •   | •   |

Clinical medicine all over the world still uses  mm Hg to measure arterial blood pressure, and often the instruments used are real mercury columns. Likewise, the central venous blood pressure is often measured using simple water columns which is very practical for the routine. The units  m H2O and  m Hg are metric units even though they are “not accepted” for use with the SI for quite a while. Although more and more hospitals in Europe switch to using the pascal to measure partial pressures in blood gas analysis, the older units will not vanish any time soon.

In the U.S. the inch is sometimes used instead of the millimeter, and because the inch is non-metric the inch of mercury or water columns is non-metric as well.

The peripheral vascular resistance unit is the vascular resistance on which a perfusion pressure of 1 mm Hg causes a flow of 1 ml/s.

The “mesh” occurs in the NIST Guide to the SI. It seems like it is the customary counterpart of the diopter.

The unit “charrière” originates from a French manufacturer of medical instruments by that name. One charrière is the gauge of a catheter with a circumference of approximately 1 mm such that it is by convention exactly one third of a millimeter. In the U.S. the charrière is simply called “french”

> **NOTE:** Note that _Unified Code for Units of Measure_ versions prior to 1.9 defined this unit as 1/π, this use, however this was never common use of the unit. This is why the definition has been corrected instead of adding another one.

A drop is a variable amount of fluid and depends on the device and technique used to produce the drop and on the physical properties of the fluid. This is similar to units like cup, tablespoon, and teaspoon that depend on the spoon or cup and are not exact either. However, in clinical medicine medication is dispensed by drops and unlike a “tablet” a drop refers to a real physical kind of quantity, volume, though not very exact.

> **NOTE:** Note that _Unified Code for Units of Measure_ versions prior to 1.9 defined this unit as mL/12, this use, however this was not common use of the unit. This is why the definition has been corrected instead of adding another one. Typically it is stated as mL/20. Original research using a 20 mL syringe filled to 5 mL shows that 1 mL has 25 drops of water, when tensides are added, the number goes up to 45. A saturated saline solution required 30 and plant oil 35. The speed of dropping, pressure and position of the syringe or the lumen of the outlet, open or partially clogged with wax, did not have a significant influence on the number of drops. While the original research suggests that the division should be by 25 or more, we use the common notion. It is discouraged to use the drop as any standard unit.

The Hounsfield unit is a unit of X-ray attenuation used in evaluating CT scans. It is defined on an interval scale where air is -1000 HF, water is 0 HF and bone is +1000 HF. Any advice as to how this unit can be related to metric units of radiant intensity decremence is appreciated.

We have always pointed out that the homeopathic teaching takes potency not as equivalent to dilution and the C and X series would not equate to each other in the strictly numerical manner. Homeopathic potency includes the “agitation” (a vigorous shaking) that needs to occur in every step of the dilluting process. Therefore as of April 2010, the homeopathic units are declared "arbitrary units", that is, they are no longer convertible. Therefore, also, we discontinue defining them using the dilution functions. The dilution functions sometimes cause truly astronomical values, leading to overflow conditions, e.g. in such potencies as 30 C or 100 X or 10 M, which do actually occur in homeopathics that are on the market. The previous units continue to exist as "retired", but their symbols now have a prime (apostrophe) in them.

**§45 chemical and biochemical units**      ** ■1** Units used mainly in chemical and biochemical laboratories are defined in [Table 19](#chemical).  ** ■2** The meaning of the columns is declared in [§43](#para-43).  ** ■3** The function pair denoted “`ph(1 mol/l)`” is defined as _f_pH(_x_) = \- lg _x_ to convert from moles per liter to the pH value, and _f_pH-1(_x_) = 10-_x_ to convert from the pH value back to moles per liter.

#### Table 19: Units used in Chemical and Biomedical Laboratories

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| equivalents | amount of substance | eq  | `eq` | `EQ` | yes | 1   | `mol` |
| osmole | amount of substance (dissolved particles) | osm | `osm` | `OSM` | yes | 1   | `mol` |
| pH  | acidity | pH  | `[pH]` | `[PH]` | no  | •   | `pH(1 mol/l)` |
| gram percent | mass concentration | g%  | `g%` | `G%` | yes | 1   | `g/dl` |
| Svedberg unit | sedimentation coefficient | S   | `[S]` | `[S]` | no  | 1   | `10*-13.s` |
| high power field | view area in microscope | HPF | `[HPF]` | `[HPF]` | no  | 1   | `1` |
| low power field | view area in microscope | LPF | `[LPF]` | `[LPF]` | no  | 100 | `1` |
| katal | catalytic activity | kat | `kat` | `KAT` | yes | 1   | `mol/s` |
| Unit | catalytic activity | U   | `U` | `U` | yes | 1   | `umol/min` |
| international unit | arbitrary | IU  | `[iU]` | `[IU]` | yes | •   | •   |
| international unit | arbitrary | i.U. | `[IU]` | `[IU]` | yes | •   | •   |
| arbitrary unit | arbitrary | arb. U | `[arb'U]` | `[ARB'U]` | no  | •   | •   |
| United States Pharmacopeia unit | arbitrary | U.S.P. | `[USP'U]` | `[USP'U]` | no  | •   | •   |
| GPL unit | biologic activity of anticardiolipin IgG |     | `[GPL'U]` | `[GPL'U]` | no  | •   | •   |
| MPL unit | biologic activity of anticardiolipin IgM |     | `[MPL'U]` | `[MPL'U]` | no  | •   | •   |
| APL unit | biologic activity of anticardiolipin IgA |     | `[APL'U]` | `[APL'U]` | no  | •   | •   |
| Bethesda unit | biologic activity of factor VIII inhibitor |     | `[beth'U]` | `[BETH'U]` | no  | •   | •   |
| anti factor Xa unit | biologic activity of factor Xa inhibitor (heparin) |     | `[anti'Xa'U]` | `[ANTI'XA'U]` | no  | •   | •   |
| Todd unit | biologic activity antistreptolysin O |     | `[todd'U]` | `[TODD'U]` | no  | •   | •   |
| Dye unit | biologic activity of amylase |     | `[dye'U]` | `[DYE'U]` | no  | •   | •   |
| Somogyi unit | biologic activity of amylase |     | `[smgy'U]` | `[SMGY'U]` | no  | •   | •   |
| Bodansky unit | biologic activity of phosphatase |     | `[bdsk'U]` | `[BDSK'U]` | no  | •   | •   |
| King-Armstrong unit | biologic activity of phosphatase |     | `[ka'U]` | `[KA'U]` | no  | •   | •   |
| Kunkel unit | arbitrary biologic activity |     | `[knk'U]` | `[KNK'U]` | no  | •   | •   |
| Mac Lagan unit | arbitrary biologic activity |     | `[mclg'U]` | `[MCLG'U]` | no  | •   | •   |
| tuberculin unit | biologic activity of tuberculin |     | `[tb'U]` | `[TB'U]` | no  | •   | •   |
| 50% cell culture infectious dose | biologic activity (infectivity) of an infectious agent preparation | CCID50 | `[CCID_50]` | `[CCID_50]` | no  | •   | •   |
| 50% tissue culture infectious dose | biologic activity (infectivity) of an infectious agent preparation | TCID50 | `[TCID_50]` | `[TCID_50]` | no  | •   | •   |
| 50% embryo infectious dose | biologic activity (infectivity) of an infectious agent preparation | EID50 | `[EID_50]` | `[EID_50]` | no  | •   | •   |
| plaque forming units | amount of an infectious agent | PFU | `[PFU]` | `[PFU]` | no  | •   | •   |
| focus forming units | amount of an infectious agent | FFU | `[FFU]` | `[FFU]` | no  | •   | •   |
| colony forming units | amount of a proliferating organism | CFU | `[CFU]` | `[CFU]` | no  | •   | •   |
| index of reactivity | amount of an allergen calibrated through in-vivo testing using the Stallergenes® method | IR  | `[IR]` | `[IR]` | no  | •   | •   |
| bioequivalent allergen unit | amount of an allergen calibrated through in-vivo testing based on the ID50EAL method of (intradermal dilution for 50mm sum of erythema diameters | BAU | `[BAU]` | `[BAU]` | no  | •   | •   |
| allergen unit | procedure defined amount of an allergen using some reference standard | AU  | `[AU]` | `[AU]` | no  | •   | •   |
| allergen unit for Ambrosia artemisiifolia | procedure defined amount of the major allergen of ragweed | Amb a 1 U | `[Amb'a'1'U]` | `[AMB'A'1'U]` | no  | •   | •   |
| protein nitrogen unit | procedure defined amount of a protein substance | PNU | `[PNU]` | `[PNU]` | no  | •   | •   |
| Limit of flocculation | procedure defined amount of an antigen substance | Lf  | `[Lf]` | `[LF]` | no  | •   | •   |
| D-antigen unit | procedure defined amount of a poliomyelitis d-antigen substance |     | `[D'ag'U]` | `[D'AG'U]` | no  | •   | •   |
| fibrinogen equivalent unit | amount of fibrinogen broken down into the measured d-dimers |     | `[FEU]` | `[FEU]` | no  | •   | •   |
| ELISA unit | arbitrary ELISA unit |     | `[ELU]` | `[ELU]` | no  | •   | •   |
| Ehrlich unit | Ehrlich unit |     | `[EU]` | `[EU]` | no  | •   | •   |

The amount of electrolytes (including acids and bases) is often reported as _equivalents_ instead of amount of substance. This habit originates in the measuring technique of titration. _The Unified Code for Units of Measure_ does not endorse using equivalents. We rather recommend to calculate the proper amount of substance after titration, so that 1 eq of Na+ ions is 1 mol, but 1 eq of Ca++ ions is 0.5 mol. The problem with equivalents is that the measurement results are difficult to compare because their magnitude depends on the degree of ionization of the substance. That is to say, the meaning of equivalents depend not only on the substance, but also on the state that the substance is in. For example, in iron we have to distinguish Fe2+ from Fe3+, so that no one can be sure how much 1 eq of iron really is.

Degrees of acidity are normally measured as “the pH value” that is the negative decadic logarithmus of the concentration of free protons (or hydronium ions) expressed in 1 mol/l. Usually the pH value is considered a dimensionless quantity. With the semantics of special units ([§§21ff](#para-21)). _The Unified Code for Units of Measure_ can link the pH value tighter to the system of proper units. Thus “`[pH]`” is defined as a unit symbol with the corresponding unit 1 mol/l. This allows conversions between pH and concentrations, and---because _The Unified Code for Units of Measure_ identifies the mole with the Avogadro number---can be converted to an absolute number of protons: for example, pH 7.4 converts instantly to 0.04 μmol/l and approximately 23975 protons per picoliter.

The unit osmol as the amount of dissolved particles is to be used with caution because it interferes with “osmolar” which is the amount of dissolved particles per liter.

The gram-percent (g%) is a metric unit that has the same origin as %vol. Originally it was a dimensionless quantity expressing a ratio of two masses and thus equal to 1/100 g/g. Because water is the most important solvent in biochemistry and 1 g of a solution in water has a volume of approximately 1 ml, the meaning of the unit 1 g% drifted towards 1/100 g/ml and farther off to 1 g/dl. That way, the unit 1 g% regained a proper dimension (mass concentration, _M_/_L_3). Most often it is used as 1 mg% = 1 mg/dl but all other SI prefixes are possible.

The Svedberg unit S is used to classify macromolecules (e.g., ribosomes) in different phases of a centrifugate.

The units “high power field” (HPF) and “low power field” (LPF) are used in microscopic analysis mostly of urine sediments. These units are used in semi-quantitative estimations of the abundance of things like crystals, bacteria or red and white blood cells. The number of the objects of interest is counted in one view field in the microscope with a 10 times (low) or 100 times (high) magnifying objective lens and then reported as the number per LPF or per HPF respectively. Obviously the number of objects seen depends on the way the slide is prepared: the amount of emulgate dropped, its initial dilution, and the way the drop is smeared. These preparations of the slides are usually carried out with great routine but little exactness, hence LPF and HPF can hardly relate to any exact and meaningful volume.

The best we could do is to define LPF and HPF as areas of the viewed field. However, the area of the field varies with the kind of eyepiece used in the microscope. The so called “field number” of the eyepiece, i.e., the diameter of the view area is typically between 18 mm and 25 mm which is divided by the magnification of the objective lense to yield the actual field diameter _d_. Because the area _A_ = π d2, the LPF can be anywhere between 2.5 mm^2 and 5 mm^2 and the HPF between 0.025 mm^2 and 0.05 mm^2. Because of this inexactness, we define LPF and HPF as dimensionless quantities with magnitudes that reflect the ratio of the view areas, i.e. 100:1. This allows at least to convert between numbers per LPF and per HPF and vice versa.

The unit “U” of enzymatic activity was defined in 1964 by the _International Union of Biochemistry_ as the catalytic activity that catalyzes the transformation of 1 μmol of the substrate per minute. This unit is defined so that normal biological enzyme activities are in the range of 1 U-100 U. This unit could not be adopted by the CGPM because it violates the style rules of the SI, i.e. “unit” is a very indistinctive word, “U” is a capital letter, and the definition is not coherent with the SI.

An SI-coherent unit katal 1 kat = 1 mol/s, had been proposed for adoption into the SI over 30 years ago and was finally adopted by the CGPM in 1999. However, perhaps because the unit katal is 7 orders of magnitudes greater than normal catalytic activities, in practice the katal has not gained much in popularity over the unit “U”.

In its 1999 decision to add the katal to the SI, the CGPM explicitly “recommends that when the katal is used, the measurand be specified by reference to the measurement procedure; the measurement procedure must identify the indicator reaction.” The general problem with catalytic activities is that these heavily depend not only on the substance but on many side-conditions, such as temperature, acidity of the solution, presence or absence of cofactors, inhibitors or activators, and the amount of substrate. Particularly a catalytic activity measured _in vitro_ says little about the activity _in vivo_. Hence the use of katal alone without specifying exactly the measurement method, is not sufficient to improve comparability of the measurement of catalytic substances.

Because of the influence of the measurement method, results of biologic activity measurement cannot usually be converted. This is a particular problem with the many named arbitrary units that are still used. _The Unified Code for Units of Measure_ initially defined all arbitrary units as dimensionless. But since this leads to the false conclusion that all arbitrary units are the same, the _Unified Code for Units of Measure_ now accounts for arbitrary units using a special flag. When a unit is marked as arbitrary, it is isolated from all other units, and no result can be converted from and to that unit (See [§24](#para-24)).

The unit “TCID50” expresses the result of quantifying an infectious agent in tissue culture. It is a titer, expressing the highest dilution of the specimen which produces a cytopathic effect in 50% of the cell cultures or wells inoculated. \[Sources: Clinical Microbiology Reviews, July 1998, Vol. 11(3), p. 533-554\]

The unit “CCID50” expresses the result of quantifying an infectious agent in a cell culture. It is a titer, expressing the highest dilution of the specimen which produces a cytopathic effect in 50% of the cell cultures or wells inoculated. \[Sources: Schmidt NJ. Cell culture procedures for diagnostic virology, p. 78-79. In Schmidt NJ, Emmons RW (ed.), _Diagnostic procedures for viral, rickettsial and chlamydial infections_, 5th ed. American Public Health Association, Inc., Washington, D.C.\]

The unit “PFU” measures viral infectivity in a sensitive assay in cell culture where the titer is determined by counting the number of visible plaques developed following viral infection of a sensitive cell culture and results recorded as PFU/ml.

The unit “FFU” measures viral infectivity in a sensitive assay in cell culture, for example, using immunofocus or vital dyes technology. For example, the titer is determined by visualizing infected areas of a cell monolayer by probing with virus-specific antibodies and results are recorded as FFU/ml. \[Sources: _WHO expert committee on biological standardization (55th Edition)._ WHO Technical Report #932;\]

The unit “BAU” measures amount of an allergen based on an in-vivo calibrated test using the Intradermal Dilution for 50mm sum of Erythema Diameters (ID50EAL) Method. \[Source: _Turkeltaub PC. Biological Standardization based on Quantitative Skin Testing - The 1D50 EAL Method. Arbeiten aus dem Paul-Ehrlich-Institut, dem Georg-Speyer-Haus und dem Ferdinand-Blum-Institut, Band 80 Gustav Fischer Verlag' Stuttgart, New York. 1987_\]

EDITORIAL NOTE: This method needs to be further investigated to determine a quantitative model which relates that would relate 1 BAU with a standardized amount of substance of the standardized allergenic protein. The situation is not unlike the titer and is not worse than for many of the arbitrary units listed already. In a future revision a stronger formalized metrologic model will be added to this specification.

The unit “AU” (for allergen unit) is for the amount of an allergen based some procedure defined and allergen specific reference standard. Note, do not confuse with astronomical unit, distinguish `[AU]` from `AU`

The unit “IR” has been defined to measure the allergenicity of an allergen extract. The allergen extract contains 100 IR/ml when, on a skin prick-test using a Stallerpoint®, it induces a wheal diameter of 7 mm in 30 patients sensitized to this allergen, (geometric mean). The cutaneous reactivity of these patients is simultaneously demonstrated by a positive skin prick-test to either 9 % codeine phosphate or 10 mg/ml histamine. The IR unit of Stallergenes is not comparable to the units used by other allergen manufacturers.

EDITORIAL NOTE: Should more manufacturer specific units come up in the future, we will include a manufacturer abbreviation in the unit symbol.

The unit “Amb a 1 U” is an arbitrary unit for the amount of Amb a 1, a 38 kD glycoprotein that is the major allergen in short ragweed (Ambrosia artemisiifolia) pollen allergen extracts. The amount of Amb a 1 units are determined by an in-vitro comparison of a test short ragweed extract to a FDA CBER Amb a 1 reference standard. Amb a 1 is the up-to-date term for the short ragweed pollen allergen that was originally described as Antigen E. They are synonyms. Although Antigen E is no longer used in the scientific literature, its meaning is unambiguous. The manufacturers are still licensed to use Antigen E as the designation. Therefore, Amb a 1 U = AgE U. There is an empiric relationship between Amb a 1 U and BAU (350 Amb a 1 U/mL = 100,000 BAU/mL). It was based on studies done decades ago on 15 study subjects. FDA's CBER considered mandating a conversion to BAU/mL in the labeling of short ragweed pollen products, based on AgE content, but this was never implemented. CBER provides two US standard reagents to manufacturers for their determination of Amb a 1 content, a reference standard and a reference serum. The assay used is a radial immunodiffusion assay (RID). Solid references discussing the relationship between Antigen E U/mL/Amb a 1 U/mL and micrograms of Antigen E U/mL/Amb a 1/mL are being researched.

EDITORIAL NOTE: The University of Texas' Structural Database of Allergenic Proteins (SDAP) contains close to 1000 allergens, isoallergens. Comparing the prospect of thousands of such special units for every allergen, one begins to appreciate even the metrologically complex BAU unit.

The unit “PNU” is defined as follows: 1 PNU/ml is equivalent to 1 x 10-5 mg of nitrogen determined to be in the material precipitated from 1 ml of allergenic extract by phosphotungstic acid (micro-Kjeldahl method). Typically, 1 mg of protein nitrogen equals 100,000 PNU. The unit “PNU” is an old protein unit unrelated to SI units. Several hundred products, from several manufacturers, are labeled in PNUs, and a switch to SI units for protein content is impractical.

The unit “Lf” is called the “Limit of Flocculation” or “limes flocculationis”. It is based on an antigen-antibody precipitation reaction and used for the quantification of the antigenic content of tetanus and diphteria toxin and toxoid. The limes flocculationis is the smallest amount of antigen that when mixed with one unit (Ramon) of antitoxin (antibody), produces the most rapid floccules in the flocculation test. For a purified crystalline tetanus or diphteria toxin 1 Lf is equivalent to ~ 2 μg of protein. For tetanus and diphtheria toxoids, antigenic purity is defined and controlled by Lf units per mg of protein nitrogen.

Many sources describe the unit of antitoxin as "international unit" (IU), however, this is no longer correct. It was correct for the first international standard for antitoxin, established in 1920s. It had an arbitrary unit defined as IU for in vivo antitoxic activity and that unit was also used for establishing Lf units of toxins and toxoids, that is why this standard had a ratio of 1 between flocculating activity (Lf) and antitoxic activity (IU). When WHO replaced that standard in 1970s, the second international standard related to Lf by a factor of 1.4 instead of 1. Ultimately, WHO decided to move to the toxoid standards and calibrated tetanus toxoid for flocculation using Lf unit (not IU). With the implementation of WHO standards for flocculation as tetanus and diphtheria toxoids, antitoxin standards were discontinued by the WHO. \[Source: _Lyng J. Quantitative Estimation of Diphtheria and Tetanus Toxoids - 4 - Toxoids as International Reference Materials Defining Lf-units for Diphtheria and Tetanus Toxoids. Biologicals (1990) 18, 11-17._ Also on the definition of the IU for antitoxin: _Spaun J, Lyng J. Replacement of the International Standard for Tetanus Antitoxin and the Use of the Standard in the Flocculation Test. Bull. Wid Hith Org. 1970, 42, 523-534._`http://www.ncbi.nlm.nih.gov/pmc/articles/PMC2427455` and personal communication with FDA CBER representatives.\]

**§46 levels**      ** ■1** Pseudo-units defined to express logarithms of ratios between two quantities of the same kind are defined in [Table 20](#levels).  ** ■2** The meaning of the columns is declared in [§43](#para-43).  ** ■3** The function pairs denoted “`ln`” “`lg`” and “`2lg`” are defined as the natural logarithm, the decadic logarithm, and the decadic logarithm times two with their respective inverse functions.

#### Table 20: Levels

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| neper | level | Np  | `Np` | `NEP` | yes | •   | `ln(1 1)` |
| bel | level | B   | `B` | `B` | yes | •   | `lg(1 1)` |
| bel sound pressure | pressure level | B(SPL) | `B[SPL]` | `B[SPL]` | yes | •   | `2lg(2 10*-5.Pa)` |
| bel volt | electric potential level | B(V) | `B[V]` | `B[V]` | yes | •   | `2lg(1 V)` |
| bel millivolt | electric potential level | B(mV) | `B[mV]` | `B[MV]` | yes | •   | `2lg(1 mV)` |
| bel microvolt | electric potential level | B(μV) | `B[uV]` | `B[UV]` | yes | •   | `2lg(1 uV)` |
| bel 10 nanovolt | electric potential level | B(10 nV) | `B[10.nV]` | `B[10.NV]` | yes | •   | `2lg(10 nV)` |
| bel watt | power level | B(W) | `B[W]` | `B[W]` | yes | •   | `lg(1 W)` |
| bel kilowatt | power level | B(kW) | `B[kW]` | `B[KW]` | yes | •   | `lg(1 kW)` |

These units are “pseudo-units” because of their standardized definition as being logarithms of a ratio of two measurements with the same kind-of-quantity: first, the units cancel out, and second, the logarithm does not produce a new unit. These units were defined as “metric” because they are used as such, although a multiplication operation is not defined on these quantities. Multiplication of the measurement value with a scalar _r_ is equivalent to raising the original ratio to the _r_-th power.

According to NIST, the neper is used as the ratio level of field quantities and the bel is used for the level of power quantities. The factor 2 comes into play when field quantities (like electric potential) are expressed in decibel. The specialized bel-units B(V), B(mV), B(W), etc. are defined as the level of the measured quantity with reference quantities 1 V, 1 mV, and 1 W respectively. \[NIST Sp. Pub. 811, 1995 Edition\]

Given the sound pressure level expressed in dB(SPL) it is feasible to define dB(A) for the A scale of loudness. Similar units such as phon and sone could be defined as well if a good approximation for the respective characteristic functions are available. Any advice is welcome.

**§47 miscellaneous units**      ** ■1** Not otherwise classified units are defined in [Table 21](#misc).  ** ■2** The meaning of the columns is declared in [§30](#para-30).2\.  ** ■3** Only the columns titled “c/s,” “c/i,” “M,” “value,” and “definition” are normative. Full name and print symbol are either not standardized or standardized by other bodies and are out of scope of _The Unified Code for Units of Measure_.  ** ■4** The function pair denoted “`sqrt`” is defined as the square root with its respective inverse function, the square.

#### Table 21: Miscellaneous Units

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| stere | volume | st  | `st` | `STR` | yes | 1   | `m3` |
| Ångström | length | Å   | `Ao` | `AO` | no  | 0.1 | `nm` |
| barn | action area | b   | `b` | `BRN` | no  | 100 | `fm2` |
| technical atmosphere | pressure | at  | `att` | `ATT` | no  | 1   | `kgf/cm2` |
| mho | electric conductance | mho | `mho` | `MHO` | yes | 1   | `S` |
| pound per square inch | pressure | psi | `[psi]` | `[PSI]` | no  | 1   | `[lbf_av]/[in_i]2` |
| circle | plane angle | circ | `circ` | `CIRC` | no  | 2   | `[pi].rad` |
| sphere | solid angle | sph | `sph` | `SPH` | no  | 4   | `[pi].sr` |
| metric carat | mass | ctm | `[car_m]` | `[CAR_M]` | no  | 0.2 | `g` |
| carat of gold alloys | mass fraction | ctAu | `[car_Au]` | `[CAR_AU]` | no  | 1   | `/24` |
| Smoot | length |     | `[smoot]` | `[SMOOT]` | no  | 67  | `[in_i]` |
| meter per square seconds per square root of hertz | amplitude spectral density |     | `[m/s2/Hz^(1/2)]` | `[M/S2/HZ^(1/2)]` | no  | •   | `sqrt(1 m2/s4/Hz)` |
| Nephelometric Turbidity Unit | turbidity |     | `[NTU]` | `[NTU]` | no  | 1   | `1` |
| Formazin Nephelometric Unit | turbidity |     | `[FNU]` | `[FNU]` | no  | 1   | `1` |

Although called “metric carat,” the carat really is a customary unit, still used for precious gems. The word carat comes from Greek κερατίκον (small horn) that originally was the horn-shaped grain of a locust-tree species in the pea family, hence the carat grain is about three barley grain that the other English systems of weights are based on. The arab carat was 1/24 of an ounce, the Imperial carat (1877) was 205.3 mg or 3.168 grain. In other European cities, the carat was 205.8 mg (Hamburg, Lisboa) but there were great variations from 188.5 mg (Bologna) to 213.5 mg (Torino). Due to these variations no customary carat has gained importance today aside from the “metric carat” defined as 200 mg exactly. \[_All About Carats_ URL: `http://www.channel1.com/users/scales/carat-def.htm`\]

The “Mark” was a mass unit for precious metals (Köln 234 g, Paris 245 g, Wien 277 g). A mark of gold was subdivided into 24 “karat,” a mark of silver into 16 “lot.” This led to the other use of the unit “carat” to mean 1/24 in measuring the finesse of pure gold in an alloy. For example, an 8 carat gold alloy contains 8 parts of gold on 16 parts of silver = 8/24 = 1/3, or 333 per mille. This carat is spelled “karat” in the U.S. while other countries do not use different spellings.

The unit “`[m/s2/Hz^(1/2)]`” is defined as a special unit to represent the odd fractional exponent of the second obtaining for the unit of the amplitude spectral density (ASD). It is defined based on the unit for the power spectral density (PSD), that is 1 (m/s2)2/Hz or 1 m2 · s-3. Since the two measurements are directly comparable, PSD = ASD2.

### 4.6 Prefixes and Units Used in Information Technology

**§48 units used in information technology**      ** ■1** Units used in information technology are defined in table 22.  ** ■2** The meaning of the columns is declared in [§43](#para-43).  ** ■3** The function pair denoted “`ld`” is defined as the dual logarithm with its respective inverse function _f_-1(_x_) = 2x).

This table is not complete. There are other units such as shannon (Sh), erlang (E), or hartley (Hart), for which we had no quantitative definitions. Any advice is appreciated.

The bit is defined twice. One definition with a subscript letter ‘s‘ is defined as the logarithmus dualis of the number of distinct signals. However this unit can not practically be used to express more than 1000 bits. Especially when the bit is used to express transmission rate or memory capacities, floating point registers would quickly overflow. Therefore we define a second symbol for bit, without the suffix, to be the dimensionless unit 1.

The baud (Bd) is the number of distinct signals transmitted per second, it is _not_ the same as bits per second since one distinct signal usually carries more than one bit of information.

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |Table 22: Units used in Information Science and Technology
| bit | amount of information | bits | `bit_s` | `BIT_S` | no  | •   | `ld(1 1)` |
| bit | amount of information | bit | `bit` | `BIT` | yes | 1   | `1` |
| byte | amount of information | B   | `By` | `BY` | yes | 8   | `bit` |
| baud | signal transmission rate | Bd  | `Bd` | `BD` | yes | 1   | `/s` |

**§49 prefixes**      ** ■1** The prefix symbols based on powers of two for use in information technology as proposed by the IEEE are defined in [Table 23](#infopfx).  ** ■2** The meaning of the columns is declared in [§49](#para-49).2\.  ** ■3** Only the columns titled “c/s,” “c/i,” and “value,” are normative. Full name and print symbol are out of scope of _The Unified Code for Units of Measure_.

This table reflects proposed prefixes which are not yet standardized. \[Bruce Barrow, _A Lesson in Megabytes._ IEEE Standards Bearer, January 1997\]

#### Table 23: The special prefix symbols for powers of 2

| name | print | c/s | c/i | value |
| --- | --- | --- | --- | --- |
| kibi | Ki  | `Ki` | `KIB` | 1024 |
| mebi | Mi  | `Mi` | `MIB` | 1048576 |
| gibi | Gi  | `Gi` | `GIB` | 1073741824 |
| tebi | Ti  | `Ti` | `TIB` | 1099511627776 |


### 4.7 Examples for some Non-Units.

**§50 Non-units**      ** ■1** Symbols commonly used as units that are no real units of measurements are not defined by _The Unified Code for Units of Measure_.  ** ■2** Users are free to use curly braces expressions ([§12](#para-12)) if they think it is important to use symbols rather than the default unit **1**.  ** ■3** Curly braces expressions are equivalent to the unit **1**. The details of the annotations in the curly braces have no defined meaning in _The Unified Code for Units of Measure_.  ** ■4**[Table 24](#junk) gives some example for those non-units but is not normative.

#### Table 24: Examples for Non-Units

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

_The Unified Code for Units of Measure_ is designed and maintained so that severe name conflicts do not occur. However, avoiding all conflicts is possible only at the cost of defining very unusual symbols for those units. As the [Table 25](#conflicts) shows, all current conflicts are of type IVa between metric and nonmetric units. This means that there is only a conflict if the metric predicate is violated so that non-metric units are used with a prefix. \[Schadow G, McDonald CJ et al: Units of Measure in Clinical Information Systems. _JAMIA_ 6(2); Mar/Apr 1999. p. 151-162.\]

#### Table 25: Summary of name conflicts

| Gb  | G-b | Type IVa (metric-nonmetric) |
| Pa  | P-a | Type IVa (metric-nonmetric) |
| ph  | p-h | Type IVa (metric-nonmetric) |
| cd  | c-d | Type IVa (metric-nonmetric) |
| CD  | C-D | Type IVa (metric-nonmetric) |
